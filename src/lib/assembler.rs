use lib::code::*;
use lib::parser::*;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub filename: PathBuf,
    pub outfile: PathBuf,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => PathBuf::from(arg),
            None => {
                return Err("No filename provided");
            }
        };

        if filename.extension().unwrap() != "asm" {
            return Err("Please provide a .asm file");
        }

        let of = filename.clone();
        let outfile = PathBuf::from(of.with_extension("hack"));

        Ok(Config { filename, outfile })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let f: File = File::open(&config.filename)?;
    let br = BufReader::new(f);
    let commands: Vec<String> = br.lines()
        .map(|l| l.expect("Could not load file"))
        .collect();

    let mut machine_code = String::new();

    let mut parser = Parser::new(commands);

    while parser.has_more_commands() {
        let comm: Command = parser.advance();
        let mut mc = match comm.command_type {
            Some(CommandType::ACommand) => translate_acommand(comm),
            Some(CommandType::CCommand) => translate_ccommand(comm),
            None => continue
        };

        mc.push_str("\n");
        machine_code.push_str(&mc);
    }

    write_hack_file(machine_code, &config.outfile)
}

pub fn write_hack_file(machine_code: String, path_name: &PathBuf) -> Result<(), Box<Error>> {
    let mut f = File::create(path_name)?;

    f.write_all(machine_code.as_bytes())?;

    Ok(())
}

pub fn translate_acommand(comm: Command) -> String {
    let a: u16 = comm.symbol.unwrap().parse::<u16>().unwrap();
    format!("{:016b}", a)
}

pub fn translate_ccommand(comm: Command) -> String {
    let d = dest(&comm.dest.unwrap_or("".to_string())).unwrap_or(0);
    let c = comp(&comm.comp.unwrap_or("".to_string())).unwrap_or(0);
    let j = jmp(&comm.jump.unwrap_or("".to_string())).unwrap_or(0);
    format!("{:016b}", 57344 + d + c + j)
}
