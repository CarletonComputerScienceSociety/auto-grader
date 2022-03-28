use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Job {
    name: String,
    rate: u32,
}