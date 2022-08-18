use std::{
    collections::VecDeque,
    convert::Infallible,
    sync::{Arc, Condvar, Mutex},
};

use bytes::BufMut;
use futures::TryStreamExt;
use nomad_client::apis::{configuration::Configuration, nodes_api::get_nodes};
use reqwest::StatusCode;
use schema::{Job, Language};
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