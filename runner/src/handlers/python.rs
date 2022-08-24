use std::{io::Write, process::Command};

use entity::job::Model as Job;

use super::Handler;

pub struct Python();

impl Handler for Python {
    fn handle(request: Job) -> String {
        // Write the input to a file
        let mut file = std::fs::File::create("main.py").unwrap();
        file.write_all(&request.file.unwrap()).unwrap();

        // Run a Python script
        let output = Command::new("python")
            .arg("main.py")
            .output()
            .expect("failed to execute process");

        // Return the output
        String::from_utf8_lossy(&output.stdout).to_string()
    }
}
