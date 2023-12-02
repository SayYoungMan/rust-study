use std::{env, process};

use minigrep::Config;
fn main() {
    // First value of the vector is the name of our binary
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Applilcation error: {e}");
        process::exit(1);
    }
}
