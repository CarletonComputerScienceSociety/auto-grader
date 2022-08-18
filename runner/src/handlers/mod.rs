use schema::Job;

pub mod java;
pub mod python;

pub struct _Request {
    pub file_location: String,
}

pub trait Handler {
    fn handle(job: Job) -> String;
}
