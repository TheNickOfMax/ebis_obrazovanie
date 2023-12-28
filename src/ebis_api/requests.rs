use crate::ebis_api::credentials;

use reqwest::Method;

// make the api request and return the responce text
pub async fn request_estimate() -> Result<String, reqwest::Error> {
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
