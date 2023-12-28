use std::error::Error;

use reqwest::Method;
mod cred;

const url: &str = "";
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = reqwest::Client::new();
    let resp = cli
        .request(Method::GET, url)
        .bearer_auth(cred::AUTH)
        .send()
        .await?
        .text()
        .await?;

    return Ok(());
}
