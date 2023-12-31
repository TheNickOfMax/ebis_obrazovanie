use crate::{
    ebis_api::requests::{
        current_calss_id, current_year_id, lessons_table, period_ids, student_id,
    },
    ebis_lib::{diary::Periods, errors::ParseOrReqError},
};

use prettytable::{row, Table};
use std::io::Write;

mod ebis_api;
mod ebis_lib;

#[tokio::main]
async fn main() -> Result<(), ParseOrReqError> {
    let login = get_input("Login ->\t");
    let password = get_input("Password ->\t");

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
    let choice_str = get_input("\n->\t");
    let choice: i32 = choice_str.parse().expect("Choose like a normal person");

    //This is stupid, i'll probably find a better solution later
    let period = periods
        .iter()
        .find(|p: &&(String, String)| p.0 == Periods::from(choice).as_str())
        .expect("Something deeply fucked up happened")
        .1
        .clone();

    // Request the grades
    println!("\n> Getting grades for that period\n");
    let disciplines = lessons_table(&year, &class, &period, &id, &bearer).await?;

    // Leave only valid grades
    let table: Vec<(String, Vec<i8>, f32, String)> = disciplines
        .iter()
        .map(|d| {
            (
                d.name.clone(),
                d.to_grades(),
                d.estimate_grade(),
                d.total_grade.clone(),
            )
        })
        .collect();

    // Convert to pretty table and print
    let mut pretty = Table::new();
    for p in table {
        let grd: String = p.1.into_iter().map(|g| g.to_string() + " ").collect();
        pretty.add_row(row![p.0, grd, p.2, p.3]);
    }
    println!("{}", pretty.to_string());

    // Revoke the token
    println!("> Revoking token");
    _ = ebis_api::auth::revoke_token(&bearer);

    // Wait for input to exit
    _ = get_input("\nPress enter to exit");

    Ok(())
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();

    print!("{}", prompt);
    _ = std::io::stdout().flush();
    _ = std::io::stdin().read_line(&mut input);

    input = input.trim().to_string();
    input
}
