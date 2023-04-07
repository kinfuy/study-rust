use std::env;
use std::error::Error;
use std::fs;
use std::process;

#[derive(Debug)]
struct Config {
    path: String,
    word: String,
}

impl Config {
    fn build(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        } else {
            Ok(Config {
                path: args[2].clone(),
                word: args[1].clone(),
            })
        }
    }
}

fn search(config: &Config) -> Result<String, Box<dyn Error>> {
    let rst = fs::read_to_string(&config.path)?;

    Ok(rst)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1)
    });
    let rst = search(&config).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1)
    });

    println!("{}", rst);
}
