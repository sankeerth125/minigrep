use minigrep::Config;
use std::env;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    println!(
        "Searching for '{}' in file '{}'\n",
        config.query, config.filename
    );

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}
