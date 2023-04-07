use std::env;
use std::process;

use finder::search;
use finder::Config;

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

    println!("{:#?}", rst);
}
