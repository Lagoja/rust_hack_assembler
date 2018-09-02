// Module for parsing and reading the files.

#[derive(Debug, PartialEq)]
pub enum CommandType {
    ACommand,
    CCommand,
}

#[derive(Debug)]
pub struct Command {
    pub command_type: Option<CommandType>,
    pub symbol: Option<String>,
    pub dest: Option<String>,
    pub comp: Option<String>,
    pub jump: Option<String>,
}

impl Command {
    pub fn new(
        command_type: Option<CommandType>,
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

    fn c_lexer(buf: &str) -> (Option<String>, Option<String>, Option<String>) {
        let vec1: Vec<&str> = buf.split("=").collect::<Vec<&str>>();
        let vec2: Vec<&str> = vec1[vec1.len()-1].split(";").collect::<Vec<&str>>();

        let dest: Option<String> = vec1.get(0).map(|s| s.to_string());
        let comp: Option<String> = vec2.get(0).map(|s| s.to_string());
        let jmp: Option<String> = vec2.get(1).map(|s| s.to_string());

        (dest, comp, jmp)
    }

    pub fn parse(buf: String) -> Command {
        let mut buf_iter = buf.chars();
        let next_char = buf_iter.next().unwrap_or('/');
        if next_char == '/' {
            return Command::new(None, None, None, None, None);
        } else if next_char == '@' {
            let sym = String::from(buf_iter.as_str());
            return Command::new(Some(CommandType::ACommand), Some(sym), None, None, None);
        } else {
            let (dest, comp, jump) = Command::c_lexer(buf.as_ref());
            return Command::new(Some(CommandType::CCommand), None, dest, comp, jump);
        };
    }
}

#[derive(Debug)]
pub struct Parser {
    file: Vec<String>,
    next_command: usize,
}

impl Parser {
    pub fn new(file: Vec<String>) -> Parser {

        Parser {
            file: file,
            next_command: 0,
        }
    }

    pub fn has_more_commands(&self) -> bool {
        &self.file.len() - &self.next_command > 0
    }

    pub fn advance(&mut self) -> Command {
        let line = self.file.get(self.next_command).unwrap().to_string();
        self.next_command += 1;
        Command::parse(line)
    }
}
