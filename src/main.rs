use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_args(&args);

    println!("Searching for {}", query);
    println!("...in file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Unable to read the file");

    println!("Text is:\n{contents}");
}

fn parse_args(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
