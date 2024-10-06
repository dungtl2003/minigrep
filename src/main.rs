use std::env;
use std::process;
use std::process::exit;

use minigrep_dungtl2003::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep_dungtl2003::run(&config) {
        eprintln!("Application error: {e}");
        exit(1);
    };
}
