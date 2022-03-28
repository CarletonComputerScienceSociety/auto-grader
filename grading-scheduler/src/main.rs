use std::sync::{Arc, Mutex};

use nomad_client::apis::{configuration::Configuration, nodes_api::get_nodes};
use reqwest::{
    multipart::{self, Part},
    Body,
};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use warp::{Filter, Rejection, Reply};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tokio::spawn(async move {
    //     match initialize_runners().await {
    //         Ok(_) => println!("Successfully initialized runners"),
    //         Err(e) => println!("Failed to initialize runners: {}", e),
    //     };
    // });

    // Path to register a new runner
    let register = warp::path!("register").and(warp::get()).and_then(register);

    warp::serve(register).run(([0, 0, 0, 0], 5000)).await;

    Ok(())
}

// Initialize stuff on Nomad
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

async fn register() -> Result<impl Reply, Rejection> {
    let client = reqwest::Client::new();

    // Since we're sending from an async context, we have to do some interesting
    // things with wrapping the file in some stream. Should probably look into
    // this later.
    let file = File::open("../grading-runner/tests/java/HelloWorld.java")
        .await
        .unwrap();
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);

    // Make a request back
    let res = client
        .post("http://localhost:5001/job")
        .body(body)
        .send()
        .await;

    Ok("success")
}
