use prettytable::{row, table, Table};

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
    println!("Input your gosuslugi login and password:");

    let stdin = std::io::stdin();

    let mut login = String::new();
    let mut password = String::new();

    _ = stdin.read_line(&mut login);
    _ = stdin.read_line(&mut password);

    login = login.trim().to_string();
    password = password.trim().to_string();

    println!("\n> Logging in {login} {password}\n");

    let bearer = ebis_api::auth::gos_login(&login, &password).await?;
    println!("\n----< Logged in >----\n");

    println!("> Getting student id");
    let id = student_id(&bearer).await?;

    println!("> Getting year id");
    let year = current_year_id(&id, &bearer).await?;

    println!("> Getting class id");
    let class = current_calss_id(&id, &year, &bearer).await?;

    println!("> Getting period ids\n");
    let periods = period_ids(&id, &year, &class, &bearer).await?;

    let period = periods
        .iter()
        .find(|p| p.0 == Periods::Term2.as_str())
        .unwrap()
        .1
        .clone();

    let disciplines = lessons_table(&year, &class, &period, &id, &bearer).await?;

    let table: Vec<(String, Vec<i8>, f32, String)> = disciplines
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

    let mut pretty = Table::new();
    for p in table {
        let grd =
            p.1.into_iter()
                .map(|g| g.to_string() + " ")
                .collect::<String>();
        pretty.add_row(row![p.0, grd, p.2, p.3]);
    }

    println!("{}", pretty.to_string());

    Ok(())
}
