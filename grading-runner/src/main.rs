#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Send a get request to the server
    let client = reqwest::Client::new();
    let res = client.get("http://scheduler:4000/hello").send().await?;

    // Parse the response
    let body = res.text().await?;

    Ok(())
}
