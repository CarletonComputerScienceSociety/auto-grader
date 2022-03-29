use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Language {
    Java,
    Python,
    C,
    Cpp,
}

// TODO: Add ID field
#[derive(Serialize, Deserialize)]
pub struct Job {
    pub file_data: Vec<u8>,
    pub file_type: Language,
}
