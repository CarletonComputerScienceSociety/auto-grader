use std::{
    collections::VecDeque,
    fs::File,
    sync::{Arc, Condvar, Mutex},
};

use nomad_client::apis::{configuration::Configuration, nodes_api::get_nodes};
use serde::{Deserialize, Serialize};
use warp::{http::Response, Filter, Rejection, Reply};

type RunnerAddress = String;

#[derive(Serialize, Deserialize)]
pub enum Language {
    Java,
    Python,
    C,
    Cpp,
}

#[derive(Serialize, Deserialize)]
pub struct Job {
    pub file_data: Vec<u8>,
    pub file_type: Language,
}

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tokio::spawn(async move {
    //     match initialize_runners().await {
    //         Ok(_) => println!("Successfully initialized runners"),
    //         Err(e) => println!("Failed to initialize runners: {}", e),
    //     };
    // });

    let runner_pool: Arc<Mutex<VecDeque<RunnerAddress>>> = Arc::new(Mutex::new(VecDeque::new()));

    let job_pool = Arc::new(JobPool::new());

    // Add some job
    for i in 0..100 {
        job_pool.add_job(Job {
            file_data: vec![1, 2, 3],
            file_type: Language::Java,
        });
    }

    // Start a thread to manage the runner pool

    // Path to register a new runner
    let register = warp::path!("register").and(warp::get()).map(move || {
        println!("Runner has registered");

        // Get a job
        loop {
            let job = job_pool.get_job();

            if let None = job {
                continue;
            }

            println!("Runner dispatched");

            return Response::builder()
                .status(200)
                .body(serde_json::to_string(&job.unwrap()).unwrap())
                .unwrap();
        }
    });

    warp::serve(register).run(([0, 0, 0, 0], 5000)).await;

    Ok(())
}

// Initialize stuff on Nomad
async fn initialize_runners() -> Result<(), Box<dyn std::error::Error>> {
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

// async fn register() -> Result<impl Reply, Rejection> {
//     // Send a file in a blocking thread
//     let handle = tokio::spawn({
//         let client = reqwest::blocking::Client::new();

//         let file = File::open("../grading-runner/tests/java/HelloWorld.java");

//         // Make a request back
//         let res = client.post("http://localhost:5001/job").body(file).send();
//     });

//     // Wait for the thread to finish
//     let res = handle.await.unwrap();

//     Ok("success")
// }
