use std::error::Error;
use std::fs;
use std::env;
extern crate getopts;
use getopts::Options;
use std::process;

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

pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_sensitive: bool,
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &'static str> {
		if args.len() < 3 {
			return Err("not enough arguments");		
		}

		let query = args[1].clone();
		let filename = args[2].clone();
		
		/*let arg: Vec<String> = env::args().collect();
		let _input = arg[0].clone();

		let mut opts = Options::new(); 
		opts.optopt("i", "", "toggle case insensitivity", "");

		let matches = match opts.parse(&arg[1..]) {
			Ok(m) => { m },
			Err(f) => { panic!(f.to_string()) }
		};*/
		let mut opts = Options::new(); 
		opts.optflag("i", "", "toggle case insensitivity");
		opts.optflag("h", "help", "print help menu");
		
		

		let matches = match opts.parse(&args[1..]) {
		Ok(m) => { m }
		Err(f) => { panic!(f.to_string()) }
		};
		
		if matches.opt_present("h") {
			println!("");
			println!("Usage:");
			println!("");
			println!("cargo run [SEARCH] [FILE] [INSENSITIVITY]");
			println!("");
			process::exit(1);		
		};
			
		if matches.opt_present("i") {
			println!("");
			println!("Case Insensitivity On!");
			println!("");
		};
		
		let case_sensitive = !matches.opt_present("i");
			
		//let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

		Ok(Config { query, filename, case_sensitive })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn one_result() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(
			vec!["safe, fast, productive."],
			search(query, contents)		
		);	
	}
	
	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
		
		assert_eq!(
			vec!["Rust:", "Trust me."],
			search_case_insensitive(query, contents)		
		);	
	}
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();

	for line in contents.lines() {
		if line.contains(query) {
			results.push(line);
		}	
	}
	
	results
}
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	let mut results = Vec::new();

	for line in contents.lines() {
		if line.to_lowercase().contains(&query) {
			results.push(line);	
		}
	}
	results
}
