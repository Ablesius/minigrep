use std::error::Error;
use std::fs;
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
pub struct Cli {
    /// The pattern to look for
    pub query: String,
    /// The path to the file to read
    pub file_path: String,
}

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(cli.file_path)?;
    for line in search(&cli.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    // Iterate through each line of the contents.
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    // Check whether the line contains our query string.
    // If it does, add it to the list of values we’re returning.
    // If it doesn’t, do nothing.
    // Return the list of results that match.
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
safe, fast, productive.
Pick three. (lol)";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
