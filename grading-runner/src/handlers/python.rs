use std::{io::Write, process::Command};

use grading_schema::Job;

use super::Handler;

pub struct Python();

impl Handler for Python {
    fn handle(request: Job) -> String {
        // Write the input to a file
        let input_file_name = format!("HelloWorld.py");
        let mut file = std::fs::File::create(input_file_name.clone()).unwrap();
        file.write_all(&request.file_data).unwrap();

        // Run a Python script
        let output = Command::new("python")
            .arg(input_file_name)
            .output()
            .expect("failed to execute process");

        // Return the output
        String::from_utf8_lossy(&output.stdout).to_string()
    }
}
