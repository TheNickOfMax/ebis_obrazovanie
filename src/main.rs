use std::io::Write;

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
    print!("login -> ");
    _ = std::io::stdout().flush();

    _ = stdin.read_line(&mut login);

    print!("password -> ");
    _ = std::io::stdout().flush();

    _ = stdin.read_line(&mut password);

    login = login.trim().to_string();
    password = password.trim().to_string();

    // Login with logging

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

    // Print out all possible periods and ask what to show

    println!("What period would you like to get grades for?\n");
    for i in 0..5 {
        println!("{}. {}", i, ebis_lib::diary::Periods::from(i).as_str());
    }

    let mut choice_str = String::new();

    _ = stdin.read_line(&mut choice_str);
    choice_str = choice_str.trim().to_string();

    let choice: i32 = choice_str.parse().expect("Choose like a normal person");

    let period = periods
        .iter()
        .find(|p: &&(String, String)| p.0 == Periods::from(choice).as_str())
        .expect("Something deeply fucked up happened")
        .1
        .clone();

    // Request the grades

    println!("> Getting grades for that period");
    let disciplines = lessons_table(&year, &class, &period, &id, &bearer).await?;

    // Covert and pretty print the table

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

    // Wait for input to stop

    println!("\nPress enter to exit");
    _ = stdin.read_line(&mut String::new());
    Ok(())
}
