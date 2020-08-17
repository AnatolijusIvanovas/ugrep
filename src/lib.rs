use std::fs;
use std::env;
use std::error::Error;

pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_sensitive: bool,
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &'static str> {
		if args.len() < 3 {
			return Err("not enough arguments")
		}
		let query = args[2].clone();
		let filename = args[1].clone();
		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
		Ok(Config { query, filename, case_sensitive, })
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
	let results = if config.case_sensitive {
		search(&config.query, &contents)
	} else {
		search_case_insensitive(&config.query, &contents)
	};
    for line in results {
        println!("{}", line);
    }
	Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
	for line in contents.lines() {
		if line.contains(query) {
            results.push(line);
        }
    }
	results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn bad_result() {
		let query = "";
		let contents = "one, two, three";
		assert_ne!(vec!["safe, fast, productive"], search(query, contents));
	}
	
	#[test]
	fn good_result() {
		let query = "";
		let contents = "safe, fast, productive";
		assert_eq!(vec!["safe, fast, productive"], search(query, contents));
	}
	
	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "\
		safe and productive,
		DuCt TaPe";
		assert_eq!(vec!["safe and productive,"], search(query, contents));
	}
	
	#[test]
	fn case_insensitive() {
        let query = "rUsT";
        let contents =  "Trust RuSt";

        assert_eq!(
            vec!["Trust RuSt"],
            search_case_insensitive(query, contents)
        );
    }
	
}