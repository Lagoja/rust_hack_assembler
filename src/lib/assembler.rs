use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::process;
use std::io::prelude::*;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pub filename: PathBuf,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => PathBuf::from(arg),
            None => {
                return Err("No filename provided");
            },
        };

        if filename.extension().unwrap() != "asm" {
            return Err("Please provide a .asm file");
        }

        Ok(Config {filename})
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f: File = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    print!("{}",contents.as_str());

    Ok(())
}
