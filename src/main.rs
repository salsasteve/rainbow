use rainbow::commands::{Config, run};
use clap::Parser;
use std::process::exit;

#[tokio::main]
async fn main() {
    println!("Parsing arguments...");
    let config = Config::parse();

    println!("Configuration parsed successfully.");
    println!("Starting application...");
    if let Err(e) = run(config).await {
        eprintln!("Application error: {e}");
        exit(1);
    }
}