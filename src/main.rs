use crate::diary_structs::{Discipline, Lesson};

use json::JsonValue;
use std::error::Error;

mod api;
mod credentials;
mod diary_structs;
mod from_json;
mod json_conversions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // get the api responce
    let response: String = api::req().await?;

    // parse responce json
    let response_json: JsonValue = json::parse(&response)?;

    // convert it to my types
    let disciplines: Vec<Discipline> = json_conversions::api_json_to_usable_vec(response_json);

    // profit
    println!("{:#?}", disciplines);
    Ok(())
}
