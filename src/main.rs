use crate::ebis_lib::diary::{Discipline, Lesson};

mod ebis_api;
mod ebis_lib;
mod json_utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get the api responce
    let response: String = ebis_api::requests::request_estimate().await?;

    // parse responce json
    let response_json: json::JsonValue = json::parse(&response)?;

    // convert it to my types
    let descipline_table: Vec<Discipline> =
        json_utils::conversions::api_json_to_ebis_structs(response_json);

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
