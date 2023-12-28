use crate::diary_structs::{Discipline, Lesson};
use crate::from_json::FromJson;
use json::JsonValue;
use reqwest::Method;
use std::error::Error;
mod cred;
mod diary_structs;
mod from_json;

// Just make the api request and return the responce text
async fn req() -> Result<String, reqwest::Error> {
    let cli: reqwest::Client = reqwest::Client::new();
    let resp: String = cli
        .request(Method::GET, cred::URL)
        .bearer_auth(cred::AUTH)
        .send()
        .await?
        .text()
        .await?;

    Ok(resp)
}

// Because of how fucked up the new diary's api response structure is
// I had to come up with this abomination. It's disgusting, huge and probably fucked.
// But it works.
fn api_json_to_usable_vec(api_json: JsonValue) -> Vec<Discipline> {
    let json_disciplines: Vec<JsonValue> =
        Vec::from_json_array(api_json["periodGradesTable"]["disciplines"].clone());

    let disciplines: Vec<Discipline> = json_disciplines
        .iter()
        .map(|dis| {
            let lessons_json = dis["grades"].clone();
            let lessons: Vec<Lesson> = Vec::<JsonValue>::from_json_array(lessons_json)
                .iter()
                .map(|lesson| Lesson {
                    lesson_id: lesson["lessonId"].as_str().unwrap_or_default().to_string(),
                    date: lesson["date"].as_str().unwrap_or_default().to_string(),
                    grades: Vec::<JsonValue>::from_json_array(lesson["grades"].clone())
                        .iter()
                        .map(|grade| {
                            Vec::<JsonValue>::from_json_array(grade.clone())
                                .iter()
                                .map(|g| g.as_str().unwrap_or_default().to_string())
                                .collect()
                        })
                        .collect(),
                })
                .collect();

            Discipline {
                name: dis["name"].as_str().unwrap_or_default().to_string(),
                total_grade: dis["totalGrade"].as_str().unwrap_or_default().to_string(),
                lessons,
            }
        })
        .collect();

    disciplines
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let response: String = req().await?;
    let response_json: JsonValue = json::parse(&response)?;
    let disciplines = api_json_to_usable_vec(response_json);
    println!("{:#?}", disciplines);
    Ok(())
}
