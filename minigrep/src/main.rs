use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let result = Config::new(env::args()).unwrap_or_else(|err| {
        print!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(result) {
        print!("Application error: {}", e);
        process::exit(1);
    }
}
