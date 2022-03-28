use std::process::Command;

use super::{Handler, Request};

pub struct Java();

impl Handler for Java {
    fn handle(request: &Request) -> String {
        // Compile Java code
        let output = Command::new("javac")
            .arg(request.file_location.clone())
            .output()
            .expect("failed to execute process");

        // Run Java code
        let output = Command::new("java")
            .arg("-cp")
            .arg("/opt/tests/java/")
            .arg("HelloWorld")
            .output()
            .expect("failed to execute process");

        // Return the output
        String::from_utf8_lossy(&output.stdout).to_string()
    }
}
