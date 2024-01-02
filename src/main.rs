use std::env;

use crate::{
    ebis_api::requests::{calss_id, lessons_table, period_ids, student_id, year_ids},
    ebis_lib::errors::ParseOrReqError,
    input::{readln, Config},
};

use prettytable::{row, Table};

mod ebis_api;
mod ebis_lib;
mod input;

#[tokio::main]
async fn main() -> Result<(), ParseOrReqError> {
    let conf = Config::from(env::args());

    println!("{:#?}", conf);

    let bearer = if let Some(bearer_token) = conf.bearer_token {
        // If token is provided then you don't need to login
        bearer_token
    } else {
        // Ask for login and password if not provided in args
        let login = conf.login.unwrap_or_else(|| readln("Login ->\t"));
        let password = conf.password.unwrap_or_else(|| readln("Password ->\t"));

        println!("\n> Logging in {} {}", login, password);
        ebis_api::auth::gos_login(&login, &password).await?
    };

    println!("\n----< Logged in >----\n");

    println!("> Getting student id");
    let id = student_id(&bearer).await?;

    let year = if let Some(y) = conf.year {
        y
    } else {
        // Provide choice if no year in args
        println!("> Getting years\n");
        let years = year_ids(&id, &bearer).await?;

        println!("Which year to get info for?");
        for (i, y) in years.iter().enumerate() {
            println!("{}. {}", i, y);
        }

        let year_choice: usize = readln("\n->\t")
            .parse()
            .expect("Choose like a normal person");

        years
            .get(year_choice)
            .expect("Choose like a normal person")
            .to_string()
    };

    println!("> Getting class id");
    let class = calss_id(&id, &year, &bearer).await?;

    println!("> Getting period ids\n");
    let periods = period_ids(&id, &year, &class, &bearer).await?;

    // Print out all possible periods and ask what to show
    println!("What period would you like to get grades for?\n");
    for (i, p) in periods.iter().enumerate() {
        println!("{}. {}", i, p.0);
    }

    let period_choice: usize = readln("\n->\t")
        .parse()
        .expect("Choose like a normal person");

    let period = periods
        .get(period_choice)
        .expect("Choose like a normal person")
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
    _ = readln("\nPress enter to exit");

    Ok(())
}
