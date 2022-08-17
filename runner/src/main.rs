use std::convert::Infallible;

use bytes::BufMut;
use futures::TryStreamExt;
use schema::{Job, Language};
use handlers::{java::Java, python::Python, Handler};
use uuid::Uuid;
use warp::{
    http::StatusCode,
    multipart::{FormData, Part},
    Rejection, Reply,
};

mod handlers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Send a get request to the server
    let client = reqwest::Client::new();

    // Get the env var of the hostname
    let scheduler_hostname = std::env::var("SCHEDULER_HOSTNAME").unwrap_or("localhost".to_string());

    let job;

    loop {
        // Get a job from the scheduler
        let res = client
            .get(&format!("http://{}:4000/register", scheduler_hostname))
            .send()
            .await;

        // Make sure the request was successful
        match res {
            Ok(response) => {
                job = match serde_json::from_str::<Job>(
                    response.text().await.unwrap_or("".to_string()).as_str(),
                ) {
                    Ok(job) => job,
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };
                break;
            }
            Err(err) => {
                println!("Couldn't get job\n{}", err);
                std::thread::sleep(std::time::Duration::from_secs(1));
                continue;
            }
        }
    }

    // Run the job
    let output = match job.file_type {
        Language::Java => Java::handle(job),
        Language::Python => Python::handle(job),
    };

    println!("{}", output);

    Ok(())
}

// From https://blog.logrocket.com/file-upload-and-download-in-rust/
async fn _upload(form: FormData) -> Result<impl Reply, Rejection> {
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
async fn _handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
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
