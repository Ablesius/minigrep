use std::process;
use minigrep::Cli;
use clap::Parser;

fn main() {
    let args = Cli::parse();

    if let Err(e) = minigrep::run(args) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
