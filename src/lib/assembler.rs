use lib::code::*;
use lib::parser::*;
use lib::symbol_table::*;
use regex::Regex;
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

        match filename.extension() {
            Some(x) => {
                if x != "asm" {
                    return Err("Please provide a .asm file");
                }
            }
            None => return Err("Please provide a .asm file"),
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

    let parser = Parser::from(l_commands, st);
    let machine_code: String = parse_commands(parser);

    write_hack_file(machine_code, &config.outfile)
}

fn process_pseudocommands(
    raw_commands: Vec<String>,
    st: &mut SymbolTable,
) -> Result<Vec<String>, String> {
    //Adds pseudocommands to the symbol table, and also cleans out line comments and empty lines for good measure. Will not remove inline comments at this stage.
    let mut pc = 0;
    let mut l_commands: Vec<String> = vec![];
    
    let pc_re = Regex::new(r"^\([a-zA-z0-9\.$_-]*\)").unwrap();
    let pc_re_i = Regex::new(r"\b\S*\b").unwrap();
    let c_re = Regex::new(r"^//.*$").unwrap();

    for command in raw_commands {
        let command = String::from(command.trim());

        if command.is_empty() | c_re.is_match(&command) {
            continue;
        } else if pc_re.is_match(&command) {
            let caps = pc_re_i.captures(&command).unwrap();
            let symbol = caps.get(0).unwrap().as_str();
            st.add_entry(symbol, pc);
        } else {
            l_commands.push(command);
            pc += 1;
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
    let sym = comm.symbol.unwrap();
    let a: u16 = sym.parse::<u16>().unwrap();
    return format!("{:016b}", a);
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
        let mut parser = Parser::new();
        let comm = parser.parse(String::from("D=A"));
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
            String::from("@1234  //Dumb Comment 2"),
            String::from("    D=D+A"),
            String::from("D=A"),
            String::from("@R13"),
            String::from("@OUTPUT_FIRST"),
            String::from("D;JGT    //dumb comment"),
        ];

        let test_result: Vec<String> = vec![
            String::from("@1234"),
            String::from("@1234  //Dumb Comment 2"),
            String::from("D=D+A"),
            String::from("D=A"),
            String::from("@R13"),
            String::from("@OUTPUT_FIRST"),
            String::from("D;JGT    //dumb comment"),
        ];

        let mut st = SymbolTable::new();

        let output = process_pseudocommands(input, &mut st).unwrap();
        assert_eq!(output, test_result);
        assert_eq!(st.get_address("LOOP").unwrap(), &0);
    }

    #[test]
    fn process_commands_test() {
        let input: Vec<String> = vec![
            String::from("@1234"),
            String::from("D=D+A  //Dumb Comment 2"),
            String::from("@Hello"),
            String::from("D=M     //dumb comment"),
        ];
        let mut st = SymbolTable::new();
        st.load_starting_table();
        let parser = Parser::from(input, st);
        let output = parse_commands(parser);
        let result = "0000010011010010
1110000010010000
0000000000010000
1111110000010000
";
        assert_eq!(output, result);
    }

}
