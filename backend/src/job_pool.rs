use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};

use entity::job::Model as JobModel;

pub struct JobPool {
    cvar: (Condvar, Mutex<Option<VecDeque<JobModel>>>),
}

impl JobPool {
    pub fn new() -> Self {
        JobPool {
            cvar: (Condvar::new(), Mutex::new(Some(VecDeque::new()))),
        }
    }

    pub fn add_job(&self, job: JobModel) {
        let (cvar, mutex) = &self.cvar;

        let mut guard = mutex.lock().unwrap();

        guard.as_mut().unwrap().push_back(job);
        cvar.notify_one();
    }

    pub fn get_job(&self) -> JobModel {
        let (cvar, mutex) = &self.cvar;

        let mut guard = mutex.lock().unwrap();

        // Wait until the mutex is storing a true value, meaning that we have
        // work to do. This wait is always fine, since the producer will only
        // notify one at a time.
        guard = cvar.wait(guard).unwrap();

        // Get the job from the mutex
        let job = guard.as_mut().unwrap().pop_front().unwrap();

        job
    }
}

impl Default for JobPool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_pool() {
        let pool = JobPool::new();
        let job = JobModel {
            id: 1,
            started: None,
            completed: None,
            file: None,
            result: None,
        };
        pool.add_job(job);
        let job = pool.get_job();
        assert_eq!(job.id, 1);
    }
}
