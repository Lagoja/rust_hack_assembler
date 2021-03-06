#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::process;
use lib::assembler;

mod lib;

fn main() {
    let config = assembler::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Could not parse file {}", err);
        process::exit(1);
    });

    if let Err(e) = assembler::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
