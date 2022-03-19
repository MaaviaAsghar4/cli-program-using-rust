use std::env;
use std::process;

// use cli_program::Config;
mod lib;

fn main() {
    // recieving arguments from cli
    let args: Vec<String> = env::args().collect();
    if let Err(e) = lib::Config::new(&args) {
        eprintln!("error: {}", e);
        process::exit(1)
    }
    let config = lib::Config::new(&args).unwrap();

    // reading the contents of the file
    if let Err(e) = lib::run(config) {
        eprintln!("error: {}", e);
        process::exit(1)
    }
}
