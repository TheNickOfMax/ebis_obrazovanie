use ebis_api::{
    credentials,
    requests::{request_current_calss_id, request_lessons_table, request_period_ids},
};

use crate::ebis_lib::diary::{Discipline, Lesson, Periods};

mod ebis_api;
mod ebis_lib;
mod json_utils;

#[tokio::main]
async fn main() -> Result<(), ebis_lib::diary::Error> {
    let year = ebis_api::requests::request_current_year_id(credentials::STUDENT_ID).await?;
    println!("{}", year);

    let class = request_current_calss_id(credentials::STUDENT_ID, &year).await?;
    println!("{}", class);

    let periods = request_period_ids(credentials::STUDENT_ID, &year, &class).await?;
    println!("{:#?}", periods);

    let period = periods
        .iter()
        .find(|p| p.0 == Periods::Term2.as_str())
        .unwrap()
        .1
        .clone();

    let table = request_lessons_table(&year, &class, &period, credentials::STUDENT_ID).await?;
    println!("{:#?}", table);
    Ok(())
}
