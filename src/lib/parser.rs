// Module for parsing and reading the files.
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
pub enum CommandType {
    A_COMMAND,
    C_COMMAND,
    L_COMMAND,
}

#[derive(Debug)]
pub struct Command {
    pub command_type: CommandType,
    pub symbol: Option<String>,
    pub dest: Option<String>,
    pub comp: Option<String>,
    pub jump: Option<String>,
}

impl Command {
    pub fn new(
        command_type: CommandType,
        symbol: Option<String>,
        dest: Option<String>,
        comp: Option<String>,
        jump: Option<String>,
    ) -> Command {
        Command {
            command_type,
            symbol,
            dest,
            comp,
            jump,
        }
    }

    pub fn c_lexer(buf: &str) -> (Option<String>, Option<String>, Option<String>) {
        let vec1: Vec<&str> = buf.split("=").collect::<Vec<&str>>();
        let vec2: Vec<&str> = vec1[1].split(";").collect::<Vec<&str>>();

        let dest: Option<String> = vec1.get(0).map(|s| s.to_string());
        let comp: Option<String> = vec2.get(0).map(|s| s.to_string());
        let jmp: Option<String> = vec2.get(1).map(|s| s.to_string());

        (dest, comp, jmp)
    }

    pub fn parse(buf: String) -> Command {
        let mut buf_iter = buf.chars();
        if buf_iter.next().unwrap() == '@' {
            let sym = String::from(buf_iter.as_str());
            return Command::new(CommandType::A_COMMAND, Some(sym), None, None, None);
        } else {
            let (dest, comp, jump) = Command::c_lexer(buf.as_ref());
            return Command::new(CommandType::C_COMMAND, None, dest, comp, jump);
        };
    }
}

#[derive(Debug)]
pub struct Parser {
    file: BufReader<File>,
    next_command: usize,
    pub current_command: Option<Command>,
}

impl Parser {
    pub fn new(file: File) -> Parser {
        let f = BufReader::new(file);

        Parser {
            file: f,
            next_command: 0,
            current_command: None,
        }
    }

    pub fn has_more_commands(self) -> bool {
        self.file.lines().count() > 0
    }

    pub fn advance(&mut self) {
        let mut buf = String::new();
        self.file.read_line(&mut buf);
        self.current_command = Some(Command::parse(buf));
    }
}
