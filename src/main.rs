use crate::diary_structs::{Discipline, Lesson};

mod api;
mod credentials;
mod diary_structs;
mod from_json;
mod json_conversions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get the api responce
    let response: String = api::request_estimate().await?;

    // parse responce json
    let response_json: json::JsonValue = json::parse(&response)?;

    // convert it to my types
    let descipline_table: Vec<Discipline> = json_conversions::api_json_to_usable_vec(response_json);

    // leave only necesarry data
    let pretty_table: Vec<(String, Vec<i8>, f32, i8)> = descipline_table
        .iter()
        .map(|d| {
            (
                d.name.clone(),
                d.to_grades::<i8>(),
                d.estimate_grade(),
                d.total_grade.parse::<i8>().unwrap_or_default().clone(),
            )
        })
        .collect();

    // profit
    for p in pretty_table {
        println!("{:?}", p);
    }
    Ok(())
}
