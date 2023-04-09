use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    path: String,
    query: String,
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        } else {
            Ok(Config {
                path: args[1].clone(),
                query: args[2].clone(),
            })
        }
    }
}

/// 搜索关键词
pub fn search(config: &Config) -> Result<Vec<String>, Box<dyn Error>> {
    let mut rst: Vec<String> = Vec::new();
    let content = fs::read_to_string(&config.path)?;
    for line in content.lines() {
        if line.contains(&config.query) {
            rst.push(line.to_owned())
        }
    }
    Ok(rst)
}
