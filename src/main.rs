use rainbow::commands::{Config, run};
use clap::Parser;

fn main() {
    // Parse the command-line arguments
    let config = Config::parse();

    // Run the application logic
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}
