use std::{
    collections::VecDeque,
    convert::Infallible,
    sync::{Arc, Condvar, Mutex},
};

use autograder_rust_schema::{Job, Language};
use bytes::BufMut;
use futures::TryStreamExt;
use nomad_client::apis::{configuration::Configuration, nodes_api::get_nodes};
use reqwest::StatusCode;
use uuid::Uuid;
use warp::{
    http::Response,
    multipart::{FormData, Part},
    Filter, Rejection, Reply,
};

pub struct JobPool {
    runners: Mutex<Option<VecDeque<Job>>>,
    cvar: Condvar,
}

impl JobPool {
    pub fn new() -> Self {
        JobPool {
            runners: Mutex::new(Some(VecDeque::new())),
            cvar: Condvar::new(),
        }
    }

    pub fn add_job(&self, job: Job) {
        let mut runners = self.runners.lock().unwrap();
        if let Some(queue) = runners.as_mut() {
            queue.push_back(job);
            self.cvar.notify_one();
        }
    }

    pub fn get_job(&self) -> Option<Job> {
        let mut runners = self.runners.lock().unwrap();

        loop {
            match runners.as_mut()?.pop_front() {
                Some(job) => return Some(job),
                None => {
                    runners = self.cvar.wait(runners).unwrap();
                }
            }
        }
    }
}

impl Default for JobPool {
    fn default() -> Self {
        Self::new()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let job_pool = Arc::new(JobPool::new());

    // Add some jobs
    for _ in 0..100_000 {
        job_pool.add_job(Job {
            file_data: "print(\"Hello, World!\")".as_bytes().to_vec(),
            file_type: Language::Python,
            file_name: "HelloWorld.py".to_string(),
        });
    }

    // Track the number of jobs dispatched
    let num_jobs_dispatched: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

    let job_pool_clone = job_pool.clone();

    // Path to register a new runner
    let register = warp::path!("register").and(warp::get()).map(move || {
        println!("Runner has registered");

        // Get a job
        loop {
            // This will cause long polling. Until there is a job that is
            // atomically returned, the runner will stay connected and wait.
            let job = job_pool_clone.get_job();

            if job.is_none() {
                continue;
            }

            let mut count = num_jobs_dispatched.lock().unwrap();
            *count += 1;
            println!("Runner {} dispatched", count);

            return Response::builder()
                .status(200)
                .body(serde_json::to_string(&job.unwrap()).unwrap())
                .unwrap();
        }
    });

    let job_pool_clone = job_pool.clone();

    // Path to add a job
    let add_job = warp::path!("add_job")
        .and(warp::post())
        .and(warp::multipart::form().max_length(5 * 1024 * 1024))
        .and(with_job_pool(job_pool_clone))
        .and_then(upload);

    let routes = register.or(add_job);

    // Start the server
    warp::serve(routes).run(([0, 0, 0, 0], 4000)).await;

    Ok(())
}

// Refereence: https://github.com/seanmonstar/warp/blob/3ff2eaf41eb5ac9321620e5a6434d5b5ec6f313f/examples/todos.rs#L99
fn with_job_pool(
    job_pool: Arc<JobPool>,
) -> impl Filter<Extract = (Arc<JobPool>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || job_pool.clone())
}

// Initialize stuff on Nomad
async fn _initialize_runners() -> Result<(), Box<dyn std::error::Error>> {
    // Make sure the job is deployed
    // Make sure the job has the right number of instances
    let config = Configuration::new();
    let namespace = None;
    let region = None;
    let index = None;
    let wait = None;
    let prefix = None;

    let _nodes = get_nodes(&config, namespace, region, index, wait, prefix).await?;

    Ok(())
}

async fn upload(form: FormData, job_pool: Arc<JobPool>) -> Result<impl Reply, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error: {}", e);
        warp::reject::reject()
    })?;

    println!("{:?}", parts);

    for p in parts {
        if p.name() == "file_uploaded" {
            let content_type = p.content_type();
            let file_ending: &str;

            println!("Getting the content type");

            // match content_type {
            //     // Make sure the file is some type of archive
            //     Some(file_type) => match file_type {
            //         "application/zip" => {
            //             file_ending = "zip";
            //         }
            //         v => {
            //             eprintln!("invalid file type found: {}", v);
            //             return Err(warp::reject::reject());
            //         }
            //     },
            //     None => {
            //         eprintln!("file type could not be determined: {:?}", content_type);
            //         return Err(warp::reject::reject());
            //     }
            // }

            println!("Getting the file data");

            // Get the file from the part
            let value = p
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|e| {
                    eprintln!("reading file error: {}", e);
                    warp::reject::reject()
                })?;

            println!("Writing the file to disk");

            let file_name = format!("./files/{}.{}", Uuid::new_v4(), "zip");
            tokio::fs::write(&file_name, value).await.map_err(|e| {
                eprint!("error writing file: {}", e);
                warp::reject::reject()
            })?;
            println!("created file: {}", file_name);
        }
    }

    Ok("success")
}

async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, "Payload too large".to_string())
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    Ok(warp::reply::with_status(message, code))
}
