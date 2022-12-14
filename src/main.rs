use minigrep;
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem processing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application Error: {}", e);
        process::exit(1)
    }
}
