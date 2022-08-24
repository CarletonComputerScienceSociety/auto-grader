use std::{io::Write, process::Command};

use entity::job::Model as Job;

use super::Handler;

pub struct Java();

impl Handler for Java {
    fn handle(request: Job) -> String {
        // Write the input to a file
        let input_file_name = format!("HelloWorld.java");

        let mut file = std::fs::File::create(input_file_name.clone()).unwrap();
        file.write_all(&request.file.unwrap()).unwrap();

        // Compile Java code
        let _output = Command::new("javac")
            .arg(input_file_name)
            .output()
            .expect("failed to execute process");

        // Run Java code
        let output = Command::new("java")
            .arg("-cp")
            .arg("/opt/")
            .arg("HelloWorld")
            .output()
            .expect("failed to execute process");

        // Return the output
        String::from_utf8_lossy(&output.stdout).to_string()
    }
}
