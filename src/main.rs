use crate::{
    ebis_api::requests::{calss_id, lessons_table, period_ids, student_id, year_ids},
    ebis_lib::errors::ParseOrReqError,
    utils::{choose, log_if, readln, Config},
};

use prettytable::{row, Table};
use std::env;

mod ebis_api;
mod ebis_lib;
mod utils;

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
        match ebis_api::auth::gos_login(&login, &password).await {
            Ok(token) => token,
            Err(err) => {
                println!("Error during login");
                return Err(err);
            }
        }
    };

    println!("\n-----<Logged in>-----\n");

    log_if("> Getting student id", conf.verbose.clone());
    let id = match student_id(&bearer).await {
        Ok(student_id) => student_id,
        Err(err) => {
            println!("Error getting student id");
            return Err(err);
        }
    };

    let year = if let Some(y) = conf.year {
        y
    } else {
        log_if("> Getting years", conf.verbose.clone());
        let years = match year_ids(&id, &bearer).await {
            Ok(years) => years,
            Err(err) => {
                println!("Error getting years");
                return Err(err);
            }
        };

        choose("Which year?", &years)
    };

    log_if("> Getting class id", conf.verbose.clone());
    let class = match calss_id(&id, &year, &bearer).await {
        Ok(class_id) => class_id,
        Err(err) => {
            println!("Error getting class id");
            return Err(err);
        }
    };

    log_if("> Getting period ids", conf.verbose.clone());
    let periods = match period_ids(&id, &year, &class, &bearer).await {
        Ok(periods) => periods,
        Err(err) => {
            println!("Error getting period ids");
            return Err(err);
        }
    };

    //-----------------------------------------------Main loop-------------------------------------------------------------
    loop {
        // Print out all possible periods and ask what to show
        let period = choose("Which period?", &periods).1;

        // Request the grades
        log_if("> Getting grades", conf.verbose.clone());
        let disciplines = match lessons_table(&year, &class, &period, &id, &bearer).await {
            Ok(disciplines) => disciplines,
            Err(err) => {
                println!("Error getting grades");
                return Err(err);
            }
        };

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
            "y" | "Y" => break,
            "n" | "N" => continue,
            _ => break,
        }
    }
    //-------------------------------------------------------------------------------------------------------------------------
    // Revoke the token
    log_if("> Revoking token", conf.verbose.clone());
    if let Err(_) = ebis_api::auth::revoke_token(&bearer).await {
        println!("Error revoking token");
    }

    Ok(())
}
