use ebis_api::{
    credentials,
    requests::{request_current_calss_id, request_lessons_table, request_period_ids},
};
use ebis_lib::errors::ParseOrReqError;

use crate::ebis_lib::diary::{Discipline, Lesson, Periods};

mod ebis_api;
mod ebis_lib;
mod json_utils;

#[tokio::main]
async fn main() -> Result<(), ParseOrReqError> {
    let bearer = ebis_api::auth::gos_login("", "").await?;

    let year =
        ebis_api::requests::request_current_year_id(credentials::STUDENT_ID, &bearer).await?;
    println!("{}", year);

    let class = request_current_calss_id(credentials::STUDENT_ID, &year, &bearer).await?;
    println!("{}", class);

    let periods = request_period_ids(credentials::STUDENT_ID, &year, &class, &bearer).await?;
    println!("{:#?}", periods);

    let period = periods
        .iter()
        .find(|p| p.0 == Periods::Term2.as_str())
        .unwrap()
        .1
        .clone();

    let table =
        request_lessons_table(&year, &class, &period, credentials::STUDENT_ID, &bearer).await?;

    let pretty: Vec<(String, Vec<i8>, f32, String)> = table
        .iter()
        .map(|d| {
            (
                d.name.clone(),
                d.to_grades::<i8>(),
                d.estimate_grade(),
                d.total_grade.clone(),
            )
        })
        .collect();

    pretty.iter().for_each(|p| println!("{:?}", p));

    Ok(())
}
