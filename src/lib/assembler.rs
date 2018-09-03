use lib::code::*;
use lib::parser::*;
use lib::regexes::*;
use lib::symbol_table::*;
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
    let raw_commands: Vec<String> = br.lines()
        .map(|l| l.expect("Could not load file"))
        .collect();

    let mut st: SymbolTable = SymbolTable::new();
    st.load_starting_table();

    let l_commands = process_pseudocommands(raw_commands, &mut st).unwrap();

    let parser = Parser::new(l_commands);
    let machine_code: String = parse_commands(parser);

    write_hack_file(machine_code, &config.outfile)
}

fn process_pseudocommands(
    //Adds pseudocommands to the symbol table, and also cleans out comments and empty lines for good measure.
    raw_commands: Vec<String>,
    st: &mut SymbolTable,
) -> Result<Vec<String>, u16> {
    let mut pc = 0;
    let mut err_pc = 0;
    let mut l_commands: Vec<String> = vec![];

    for command in raw_commands {
        let command = String::from(command.trim());
        if command.is_empty() | comment_re().is_match(&command) {
            err_pc += 1;
            continue;
        }

        if pseudocommand_re().is_match(&command) {
            let caps = pseudocommand_inner_re().captures(&command).unwrap();
            let symbol = caps.get(0).unwrap().as_str();
            st.add_entry(symbol, pc);
            err_pc += 1;
        } else if acommand_re().is_match(&command) | ccommand_re().is_match(&command) {
            l_commands.push(command);
            pc += 1;
            err_pc += 1
        } else {
            return Err(err_pc);
        }
    }
    Ok(l_commands)
}

fn parse_commands(mut parser: Parser) -> String {
    let mut machine_code = String::new();

    while parser.has_more_commands() {
        let comm: Command = parser.advance();
        let mut mc = match comm.command_type {
            Some(CommandType::ACommand) => translate_acommand(comm),
            Some(CommandType::CCommand) => translate_ccommand(comm),
            None => continue,
        };

        mc.push_str("\n");
        machine_code.push_str(&mc);
    }
    machine_code
}

fn write_hack_file(machine_code: String, path_name: &PathBuf) -> Result<(), Box<Error>> {
    let mut f = File::create(path_name)?;

    f.write_all(machine_code.as_bytes())?;

    Ok(())
}

fn translate_acommand(comm: Command) -> String {
    let a: u16 = comm.symbol.unwrap().parse::<u16>().unwrap();
    format!("{:016b}", a)
}

fn translate_ccommand(comm: Command) -> String {
    let d = dest(&comm.dest.unwrap_or("".to_string())).unwrap_or(0);
    let c = comp(&comm.comp.unwrap_or("".to_string())).unwrap_or(0);
    let j = jmp(&comm.jump.unwrap_or("".to_string())).unwrap_or(0);
    format!("{:016b}", 57344 + d + c + j)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn translate_acommand_test() {
        let comm = Command::new(
            Some(CommandType::ACommand),
            Some(String::from("2")),
            None,
            None,
            None,
        );
        let outstring = translate_acommand(comm);
        assert_eq!(outstring, "0000000000000010");
    }

    #[test]
    fn translate_ccommand_destcomp_test() {
        let comm = Command::parse(String::from("D=A"));
        let outstring = translate_ccommand(comm);
        assert_eq!(outstring, "1110110000010000");
    }

    #[test]
    fn process_pseudocommands_test() {
        let input: Vec<String> = vec![
            String::from("//A comment"),
            String::new(),
            String::from("(LOOP)"),
            String::from("@1234"),
            String::from("    D=D+A"),
        ];

        let testResult: Vec<String> = vec![String::from("@1234"), String::from("D=D+A")];

        let mut st = SymbolTable::new();

        let output = process_pseudocommands(input, &mut st).unwrap();
        assert_eq!(output, testResult);
        assert_eq!(st.get_address("LOOP").unwrap(), &0);
    }

}
