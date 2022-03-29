use std::{convert::Infallible, process::Command, time::Duration};

use bytes::BufMut;
use futures::TryStreamExt;
use handlers::{java::Java, python::Python, Handler, Request};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use uuid::Uuid;
use warp::{
    http::StatusCode,
    multipart::{FormData, Part},
    Filter, Rejection, Reply,
};

mod handlers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Send a get request to the server
    let client = reqwest::Client::new();

    // When the server gets a request, start the task

    // Start a thread with the server
    let server_thread = tokio::spawn(async move {
        let upload_route = warp::path!("job")
            .and(warp::post())
            // Accept a file
            .and(warp::multipart::form().max_length(5 * 1024 * 1024))
            // Accept a job
            // .and(warp::body::content_length_limit(4 * 1024))
            // .and(warp::body::json())
            .and_then(upload);

        let download_route = warp::path("files").and(warp::fs::dir("./files/"));

        let router = upload_route.or(download_route).recover(handle_rejection);

        warp::serve(router).run(([0, 0, 0, 0], 5001)).await;
    });

    // Register as a runner
    let res = client
        .get("http://192.168.1.102:5000/register")
        .send()
        .await;

    // Print the response body
    println!("{:?}", res.unwrap().text().await.unwrap());

    // Wait for server thread to finish
    server_thread.await?;

    // loop {
    //     sleep(Duration::from_millis(1000)).await;
    //     let res = client.get("http://192.168.1.102:5000/hello").send().await;

    //     // Make sure the Python script returns the correct output
    //     let python_output = Python::handle(&Request {
    //         file_location: "/opt/tests/python/main.py".to_string(),
    //     });

    //     dbg!(python_output.clone());

    //     assert!(python_output == "Hello, World!\n");

    //     // Make sure the Java code returns the correct output
    //     let java_output = Java::handle(&Request {
    //         file_location: "/opt/tests/java/HelloWorld.java".to_string(),
    //     });

    //     dbg!(java_output.clone());

    //     assert!(java_output == "Hello, World!\n");

    //     // Handle the response
    //     match res {
    //         Ok(res) => {
    //             // println!("Response: {}", res.text().await?);
    //         }
    //         Err(e) => {
    //             println!("Error: {}", e);
    //         }
    //     }
    // }

    Ok(())
}

// From https://blog.logrocket.com/file-upload-and-download-in-rust/
async fn upload(form: FormData) -> Result<impl Reply, Rejection> {
    // dbg!(form);

    // return Ok("success");

    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error: {}", e);
        warp::reject::reject()
    })?;

    for p in parts {
        // dbg!(p);
        // continue;
        if p.name() == "file" {
            let content_type = p.content_type();
            let file_ending;
            match content_type {
                Some(file_type) => match file_type {
                    "application/pdf" => {
                        file_ending = "pdf";
                    }
                    "image/png" => {
                        file_ending = "png";
                    }
                    // Java file .java
                    "application/octet-stream" => {
                        file_ending = "java";
                    }
                    v => {
                        eprintln!("invalid file type found: {}", v);
                        return Err(warp::reject::reject());
                    }
                },
                None => {
                    eprintln!("file type could not be determined");
                    return Err(warp::reject::reject());
                }
            }

            let value = p
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|e| {
                    eprintln!("reading file error: {}", e);
                    warp::reject::reject()
                })?;

            let file_name = format!("./files/{}.{}", Uuid::new_v4().to_string(), file_ending);
            tokio::fs::write(&file_name, value).await.map_err(|e| {
                eprint!("error writing file: {}", e);
                warp::reject::reject()
            })?;
            println!("created file: {}", file_name);
        }
    }

    Ok("success")
}

// From https://blog.logrocket.com/file-upload-and-download-in-rust/
async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, "Payload too large".to_string())
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    Ok(warp::reply::with_status(message, code))
}
