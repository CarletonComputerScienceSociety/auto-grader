use warp::Filter;

pub struct Backend {}

impl Backend {
    pub fn new() -> Backend {
        Backend {}
    }

    pub fn run(&self) {}

    pub fn register(&self) {
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
    }

    pub fn add_job(&self) {
        warp::path!("add_job")
        .and(warp::post())
        .and(warp::multipart::form().max_length(5 * 1024 * 1024))
        .and(with_job_pool(job_pool_clone))
        .and_then(upload)
    }
}
