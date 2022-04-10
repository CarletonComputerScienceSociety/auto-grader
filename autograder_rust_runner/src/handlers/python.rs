use std::{io::Write, process::Command};

use autograder_rust_schema::Job;

use super::Handler;

pub struct Python();

impl Handler for Python {
    fn handle(request: Job) -> String {
        // Write the input to a file
        let mut file = std::fs::File::create(request.file_name.clone()).unwrap();
        file.write_all(&request.file_data).unwrap();

        // Run a Python script
        let output = Command::new("python")
            .arg(request.file_name)
            .output()
            .expect("failed to execute process");

        // Return the output
        String::from_utf8_lossy(&output.stdout).to_string()
    }
}
