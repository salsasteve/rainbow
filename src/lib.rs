use clap::{Parser, Subcommand};
use std::error::Error;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Search {
        #[arg(long = "query", short = 'q', env = "QUERY")]
        query: String,
        #[arg(long = "file", short = 'f', env = "FILE")]
        file_path: String,
        #[arg(
            long = "ignore-case",
            short = 'i',
            env = "IGNORE_CASE",
            default_value_t = false
        )]
        ignore_case: bool,
    },

    Example {
        #[arg(long, short)]
        example_arg: String,
    },
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.command {
        Commands::Search {
            query,
            file_path,
            ignore_case,
        } => {
            let contents = fs::read_to_string(file_path)?;

            let results = if ignore_case {
                search_case_insensitive(&query, &contents)
            } else {
                search(&query, &contents)
            };

            for line in results {
                println!("{line}");
            }
        }
        Commands::Example { example_arg } => {
            println!("Example command executed with argument: {example_arg}");
        }
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
