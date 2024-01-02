use std::{env::Args, fmt::Debug, io::Write};

#[derive(Debug)]
pub struct Config {
    pub verbose: bool,
    pub login: Option<String>,
    pub password: Option<String>,
    pub year: Option<String>,
    pub bearer_token: Option<String>,
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
            login: get_argument_value(&args, "-l"),
            password: get_argument_value(&args, "-p"),
            year: get_argument_value(&args, "-y"),
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

pub fn choose<T>(prompt: &str, options: &Vec<T>) -> T
where
    T: Debug,
    T: Clone,
{
    println!("\n{}", prompt);
    for (i, option) in options.iter().enumerate() {
        println!("{}. {:?}", i, option);
    }

    let choice: usize = readln("\n->\t")
        .parse()
        .expect("Choose like a normal person");

    let chosen: T = options
        .get(choice)
        .cloned()
        .expect("Choose like a normal person");

    chosen
}

pub fn log_if(s: &str, b: bool) {
    if b {
        println!("{}", s)
    }
}
