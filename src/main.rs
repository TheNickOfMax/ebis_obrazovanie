use json::JsonValue;
use reqwest::Method;
use std::error::Error;
mod cred;

const url: &str = "";
#[tokio::main]
async fn req() -> Result<String, reqwest::Error> {
    let cli: reqwest::Client = reqwest::Client::new();
    let resp: String = cli
        .request(Method::GET, url)
        .bearer_auth(cred::AUTH) // Replace with your actual token
        .send()
        .await?
        .text()
        .await?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let response = req().await?;
    let response_json: JsonValue = json::parse(&response)?;

    let sensible: Vec<(&str, Vec<String>)> = response_json["periodGradesTable"]["disciplines"]
        .members()
        .map(|j| {
            let name = j["name"].as_str().unwrap_or_default();
            let grades = j["grades"]
                .members()
                .flat_map(|gg| {
                    gg["grades"].members().flat_map(|i| {
                        i.members().map(|m| {
                            let grade = m.as_str().unwrap_or_default();
                            grade.to_owned()
                        })
                    })
                })
                .collect::<Vec<String>>();
            (name, grades)
        })
        .collect();

    Ok(())
}
