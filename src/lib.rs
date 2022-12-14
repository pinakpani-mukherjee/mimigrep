use std::error::Error;
use std::fs;
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.trim().to_lowercase().contains(query) {
            results.push(line.trim());
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.trim().to_lowercase().contains(&query) {
            results.push(line.trim());
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive!
        Pick three.
        ";
        assert_eq!(vec!["safe, fast, productive!"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rusT";
        let contents = "\
        Rust:
        safe, fast, productive!
        Pick three.
        Trust me :)
        ";
        assert_eq!(
            vec!["Rust:", "Trust me :)"],
            search_case_insensitive(query, contents)
        );
    }
}
