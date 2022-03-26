pub mod java;
pub mod python;

pub struct Request {
    pub file_location: String,
}

pub trait Handler {
    fn handle(request: &Request) -> String;
}
