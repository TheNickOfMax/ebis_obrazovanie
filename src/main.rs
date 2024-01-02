use crate::{
    ebis_api::requests::{calss_id, lessons_table, period_ids, student_id, year_ids},
    ebis_lib::errors::ParseOrReqError,
    input::{readln, Config},
};

use prettytable::{row, Table};
use std::env;

mod ebis_api;
mod ebis_lib;
mod input;

#[tokio::main]
async fn main() -> Result<(), ParseOrReqError> {
    let conf = Config::from(env::args());

    log_if(&format!("\n{:#?}\n", conf), conf.verbose.clone());

    let bearer = if let Some(bearer_token) = conf.bearer_token {
        // If token is provided then you don't need to login
        bearer_token
    } else {
        // Ask for login and password if not provided in args
        let login = conf.login.unwrap_or_else(|| readln("Login ->\t"));
        let password = conf.password.unwrap_or_else(|| readln("Password ->\t"));

        log_if("\n> Logging in", conf.verbose.clone());
        ebis_api::auth::gos_login(&login, &password).await?
    };

    log_if("\n-----<Logged in>-----\n", conf.verbose.clone());

    log_if("> Getting student id", conf.verbose.clone());
    let id = student_id(&bearer).await?;

    let year = if let Some(y) = conf.year {
        y
    } else {
        // Provide choice if no year in args
        log_if("> Getting years", conf.verbose.clone());
        let years = year_ids(&id, &bearer).await?;

        println!("\nWhich year?");
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

    log_if("> Getting class id", conf.verbose.clone());
    let class = calss_id(&id, &year, &bearer).await?;

    log_if("> Getting period ids", conf.verbose.clone());
    let periods = period_ids(&id, &year, &class, &bearer).await?;
    //-----------------------------------------------Main loop-------------------------------------------------------------
    loop {
        // Print out all possible periods and ask what to show
        println!("\nWhich period?");
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
        log_if("> Getting grades", conf.verbose.clone());
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
        let x: &str = &readln("Exit? (Y/n) ->\t");

        match x {
            "y" => break,
            "Y" => break,
            "n" => continue,
            "N" => continue,
            _ => break,
        }
    }
    //-------------------------------------------------------------------------------------------------------------------------
    // Revoke the token
    log_if("> Revoking token", conf.verbose.clone());
    _ = ebis_api::auth::revoke_token(&bearer);

    Ok(())
}

fn log_if(s: &str, b: bool) {
    if b {
        println!("{}", s)
    }
}
