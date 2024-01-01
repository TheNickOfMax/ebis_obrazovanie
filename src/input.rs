use std::{env::Args, io::Write};

pub struct Config {
    pub verbose: bool,
    pub credentials_path: Option<String>,
    pub login: Option<String>,
    pub password: Option<String>,
    pub year: Option<String>,
    pub class_id: Option<String>,
    pub period_id: Option<String>,
    pub student_id: Option<String>,
    pub bearer_token: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            verbose: false,
            credentials_path: None,
            login: None,
            password: None,
            year: None,
            class_id: None,
            period_id: None,
            student_id: None,
            bearer_token: None,
        }
    }
}

impl From<Args> for Config {
    fn from(arguments: Args) -> Self {
        let args: Vec<String> = arguments.collect();

        fn get_argument_value(args: &[String], flag: &str) -> Option<String> {
            args.iter()
                .position(|a| a == flag)
                .and_then(|i| args.get(i + 1).cloned())
        }

        Config {
            verbose: args.iter().any(|a| a == "-v"),
            credentials_path: get_argument_value(&args, "-f"),
            login: get_argument_value(&args, "-l"),
            password: get_argument_value(&args, "-p"),
            year: get_argument_value(&args, "-y"),
            class_id: get_argument_value(&args, "-c"),
            period_id: get_argument_value(&args, "-p"),
            student_id: get_argument_value(&args, "-s"),
            bearer_token: get_argument_value(&args, "-t"),
        }
    }
}

pub fn readln(prompt: &str) -> String {
    let mut input = String::new();

    print!("{}", prompt);
    _ = std::io::stdout().flush();
    _ = std::io::stdin().read_line(&mut input);

    input = input.trim().to_string();
    input
}
