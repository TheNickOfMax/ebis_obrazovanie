use crate::{
    ebis_api::requests::{
        current_calss_id, current_year_id, lessons_table, period_ids, student_id,
    },
    ebis_lib::{diary::Periods, errors::ParseOrReqError},
};

mod ebis_api;
mod ebis_lib;
mod json_utils;

#[tokio::main]
async fn main() -> Result<(), ParseOrReqError> {
    let bearer = ebis_api::auth::gos_login("", "").await?;

    let id = student_id(&bearer).await?;

    let year = current_year_id(&id, &bearer).await?;

    let class = current_calss_id(&id, &year, &bearer).await?;

    let periods = period_ids(&id, &year, &class, &bearer).await?;

    let period = periods
        .iter()
        .find(|p| p.0 == Periods::Term2.as_str())
        .unwrap()
        .1
        .clone();

    let table = lessons_table(&year, &class, &period, &id, &bearer).await?;

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
