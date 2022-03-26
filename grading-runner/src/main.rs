use std::time::Duration;

use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Send a get request to the server
    let client = reqwest::Client::new();

    loop {
        sleep(Duration::from_millis(1000)).await;
        let res = client.get("http://192.168.1.102:4000/hello").send().await;

        // Handle the response
        match res {
            Ok(res) => {
                println!("Response: {}", res.text().await?);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
