use std::process::Command;

use super::{Handler, Request};

pub struct Python();

impl Handler for Python {
    fn handle(request: &Request) -> String {
        // Run a Python script
        let output = Command::new("python")
            .arg(request.file_location.clone())
            .output()
            .expect("failed to execute process");

        // Return the output
        String::from_utf8_lossy(&output.stdout).to_string()
    }
}
