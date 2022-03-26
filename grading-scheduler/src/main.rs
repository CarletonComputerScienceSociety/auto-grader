use std::sync::{Arc, Mutex};

use nomad_client::apis::{configuration::Configuration, nodes_api::get_nodes};
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async move {
        match initialize_runners().await {
            Ok(_) => println!("Successfully initialized runners"),
            Err(e) => println!("Failed to initialize runners: {}", e),
        };
    });

    // Get each deployment to register with the main node

    let deployment_count: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    let hello = warp::path!("hello").map(move || {
        println!("registered");

        let mut count = deployment_count.lock().unwrap();
        *count += 1;

        format!("Registered {} deployments", *count)
    });

    warp::serve(hello).run(([0, 0, 0, 0], 4000)).await;

    Ok(())
}

async fn initialize_runners() -> Result<(), Box<dyn std::error::Error>> {
    // Make sure the job is deployed
    // Make sure the job has the right number of instances
    let config = Configuration::new();
    let namespace = None;
    let region = None;
    let index = None;
    let wait = None;
    let prefix = None;

    let _nodes = get_nodes(&config, namespace, region, index, wait, prefix).await?;

    Ok(())
}
