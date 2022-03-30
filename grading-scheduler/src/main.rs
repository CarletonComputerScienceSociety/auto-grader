use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

use grading_schema::{Job, Language};
use nomad_client::apis::{configuration::Configuration, nodes_api::get_nodes};
use warp::{http::Response, Filter};

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
    let job_pool = Arc::new(JobPool::new());

    // Add some jobs
    for _ in 0..1000 {
        job_pool.add_job(Job {
            file_data: "print(\"Hello, World!\")".as_bytes().to_vec(),
            file_type: Language::Python,
            file_name: "HelloWorld.py".to_string(),
        });
    }

    // Path to register a new runner
    let register = warp::path!("register").and(warp::get()).map(move || {
        println!("Runner has registered");

        // Get a job
        loop {
            // This will cause long polling. Until there is a job that is
            // atomically returned, the runner will stay connected and wait.
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

    // Start the server
    warp::serve(register).run(([0, 0, 0, 0], 4000)).await;

    Ok(())
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
