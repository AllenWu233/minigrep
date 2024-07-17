use std::env;
use std::process;

use minigrep::Config;

// NOTE: Separation of Concerns
fn main() {
    // Receive arguments
    let args: Vec<String> = env::args().collect();

    let config = Config::from(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Applicatoin error: {e}");
        process::exit(1);
    }
}
