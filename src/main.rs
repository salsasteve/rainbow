// filepath: /home/salsasteve/code/rainbow/src/main.rs
use rainbow::Config;
use rainbow::run;
use clap::Parser;

fn main() {
    let config = Config::parse();

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}