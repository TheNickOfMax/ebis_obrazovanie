use crate::credentials;

use reqwest::Method;

// make the api request and return the responce text
pub async fn req() -> Result<String, reqwest::Error> {
    let cli: reqwest::Client = reqwest::Client::new();
    let resp: String = cli
        .request(Method::GET, credentials::URL)
        .bearer_auth(credentials::AUTH)
        .send()
        .await?
        .text()
        .await?;

    Ok(resp)
}
