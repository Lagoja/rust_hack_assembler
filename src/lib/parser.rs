// Module for parsing and reading the files.

#[derive(Debug, PartialEq)]
pub enum CommandType {
    ACommand,
    CCommand,
}

#[derive(Debug, PartialEq)]
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
        let vec2: Vec<&str> = vec1[vec1.len() - 1].split(";").collect::<Vec<&str>>();

        let dest: Option<String>;

        if vec1.len() > 1 {
            dest = vec1.get(0).map(|s| s.to_string());
        } else {
            dest = None
        }
        let comp: Option<String> = vec2.get(0).map(|s| s.to_string());
        let jmp: Option<String> = vec2.get(1).map(|s| s.to_string());

        (dest, comp, jmp)
    }

    pub fn parse(buf: String) -> Command {
        let mut buf_iter = buf.chars();
        let next_char = buf_iter.next().unwrap_or('/');
        if next_char == '/' {
            // Signal that we should skip by returning None in all Command fields
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

#[cfg(test)]
mod test {
    use super::*;

    //Lexer Tests
    #[test]
    fn c_lexer_destcomp() {
        let input = "A=D";
        let (dest, comp, jmp) = Command::c_lexer(input);
        assert_eq!(dest, Some(String::from("A")));
        assert_eq!(comp, Some(String::from("D")));
        assert_eq!(jmp, None);
    }

    #[test]
    fn c_lexer_compjmp() {
        let input = "D;JEQ";
        let (dest, comp, jmp) = Command::c_lexer(input);
        assert_eq!(dest, None);
        assert_eq!(comp, Some(String::from("D")));
        assert_eq!(jmp, Some(String::from("JEQ")));
    }

    #[test]
    fn c_lexer_destcompjmp() {
        let input = "D=A+1;JGT";
        let (dest, comp, jmp) = Command::c_lexer(input);
        assert_eq!(dest, Some(String::from("D")));
        assert_eq!(comp, Some(String::from("A+1")));
        assert_eq!(jmp, Some(String::from("JGT")));
    }

    #[test]
    //Currently this assigns to comp, which might be ok since it's found and handled later in the flow. Still, this should probably return none, or throw an error indicating the statement is invalid.
    fn c_lexer_nonsense() {
        let input = "blahblahblah";
        let (dest, comp, jmp) = Command::c_lexer(input);
        assert_eq!(dest, None);
        assert_eq!(comp, Some(String::from("blahblahblah")));
        assert_eq!(jmp, None);
    }

    //Parse Tests
    #[test]
    fn parser_emptyline() {
        let input = "";
        let comm: Command = Command::parse(String::from(input));
        assert_eq!(comm, Command::new(None, None, None, None, None));
    }

   #[test]
    fn parser_comment() {
        let input = "// Test comment";
        let comm: Command = Command::parse(String::from(input));
        assert_eq!(comm, Command::new(None, None, None, None, None));
    }

   #[test]
    fn parser_acommand() {
        let input = "@12345";
        let comm: Command = Command::parse(String::from(input));
        assert_eq!(comm, Command::new(Some(CommandType::ACommand), Some(String::from("12345")), None, None, None));
    }

   #[test]
    fn parser_ccommand() {
        let input = "D=D+1";
        let comm: Command = Command::parse(String::from(input));
        assert_eq!(comm, Command::new(Some(CommandType::CCommand), None, Some(String::from("D")), Some(String::from("D+1")), None));
        
    }
    
}
