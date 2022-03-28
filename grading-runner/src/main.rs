use std::{process::Command, time::Duration};

use handlers::{java::Java, python::Python, Handler, Request};
use tokio::time::sleep;

mod handlers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Send a get request to the server
    let client = reqwest::Client::new();

    loop {
        sleep(Duration::from_millis(1000)).await;
        let res = client.get("http://192.168.1.102:4000/hello").send().await;

        // Make sure the Python script returns the correct output
        let python_output = Python::handle(&Request {
            file_location: "/opt/tests/python/main.py".to_string(),
        });

        dbg!(python_output.clone());

        assert!(python_output == "Hello, World!\n");

        // Make sure the Java code returns the correct output
        let java_output = Java::handle(&Request {
            file_location: "/opt/tests/java/HelloWorld.java".to_string(),
        });

        dbg!(java_output.clone());

        assert!(java_output == "Hello, World!\n");

        // Handle the response
        match res {
            Ok(res) => {
                // println!("Response: {}", res.text().await?);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
