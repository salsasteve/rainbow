use rainbow::commands::{Config, run};
use clap::Parser;
use tokio::runtime::Runtime;
use std::process::exit;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        println!("Parsing arguments...");
        let config = Config::parse();

        println!("Configuration parsed successfully.");
        println!("Starting application...");
        if let Err(e) = run(config).await {
            eprintln!("Application error: {e}");
            exit(1);
        }
    });
}
