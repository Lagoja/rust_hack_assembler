// Module for parsing and reading assembly code. Iterates over a vector of strings and returns Command objects that represent the command.
use lib::symbol_table::*;
use regex::Regex;

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

}

#[derive(Debug)]
pub struct Parser {
    file: Vec<String>,
    symbol_table: SymbolTable,
    next_command: usize,
}

impl Parser {
    #![allow(dead_code)]
    pub fn new() -> Parser {
        Parser {
            file: vec![],
            next_command: 0,
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn from(file: Vec<String>, symbol_table: SymbolTable) -> Parser {
        Parser {
            file: file,
            next_command: 0,
            symbol_table: symbol_table,
        }
    }

    pub fn has_more_commands(&self) -> bool {
        &self.file.len() - &self.next_command > 0
    }

    pub fn advance(&mut self) -> Command {
        let line = self.file.get(self.next_command).unwrap().to_string();
        self.next_command += 1;
        self.parse(line)
    }

    pub fn parse(&mut self, buf: String) -> Command {
        //By this point, the L commands should be in the symboltable. So all I need to do is check the a_command symbols against the table and add them if they are not present
        let mut command = String::from(buf.trim());
        command = Parser::clean_inline_comments(command);
        lazy_static! {
            static ref AC_RE: Regex = Regex::new(r"^@.*").unwrap();
            static ref N_SYM_RE: Regex = Regex::new(r"^\d*$").unwrap();
            static ref AC_SYMBOL_RE: Regex = Regex::new(r"\b.+").unwrap();
        }
        //Extract this into a separate function
        if AC_RE.is_match(&command) {
            let mut sym = String::from(
                AC_SYMBOL_RE
                    .captures(&command)
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str(),
            );
            //check if the symbol is in the SymbolTable
            if !N_SYM_RE.is_match(&sym) {
                if self.symbol_table.contains(&sym) {
                    sym = self.symbol_table.get_address(&sym).unwrap().to_string();
                } else {
                    //TODO: This is bad. I should add error handling in case no address is returned
                    let addr = self.symbol_table.get_free_address();
                    self.symbol_table.add_entry(&sym, addr);
                    {
                        self.symbol_table.current_address += 1;
                    }
                    sym = addr.to_string();
                }
            }
            sym = Parser::clean_inline_comments(sym);
            return Command::new(Some(CommandType::ACommand), Some(sym), None, None, None);
        } else {
            let (dest, comp, jump) = Parser::c_lexer(command.as_ref());
            return Command::new(Some(CommandType::CCommand), None, dest, comp, jump);
        };
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
    

    fn clean_inline_comments(sym: String) -> String {
        let mut s = sym.clone();
        lazy_static! {
            static ref IC_RE: Regex = Regex::new(r"\s*//.*").unwrap();
        }
        match IC_RE.find(&sym) {
            Some(x) => {
                s.split_off(x.start());
                return s;
            }
            None => return s,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    //Clean Inline Comments Test
    #[test]
    fn clean_inline_comments_acommand() {
        let input = "@1234      //Test";
        assert_eq!(
            Parser::clean_inline_comments(String::from(input)),
            String::from("@1234")
        );
    }

    #[test]
    fn clean_inline_comments_ccommand() {
        let input = "D+1    //Test";
        assert_eq!(
            Parser::clean_inline_comments(String::from(input)),
            String::from("D+1")
        );
    }

    //Lexer Tests
    #[test]
    fn c_lexer_destcomp() {
        let input = "A=D";
        let (dest, comp, jmp) = Parser::c_lexer(input);
        assert_eq!(dest, Some(String::from("A")));
        assert_eq!(comp, Some(String::from("D")));
        assert_eq!(jmp, None);
    }

    #[test]
    fn c_lexer_compjmp() {
        let input = "D;JEQ";
        let (dest, comp, jmp) = Parser::c_lexer(input);
        assert_eq!(dest, None);
        assert_eq!(comp, Some(String::from("D")));
        assert_eq!(jmp, Some(String::from("JEQ")));
    }

    #[test]
    fn c_lexer_destcompjmp() {
        let input = "D=A+1;JGT";
        let (dest, comp, jmp) = Parser::c_lexer(input);
        assert_eq!(dest, Some(String::from("D")));
        assert_eq!(comp, Some(String::from("A+1")));
        assert_eq!(jmp, Some(String::from("JGT")));
    }

    #[test]
    //Currently this assigns to comp, which might be ok since it's found and handled later in the flow. Still, this should probably return none, or throw an error indicating the statement is invalid.
    fn c_lexer_nonsense() {
        let input = "blahblahblah";
        let (dest, comp, jmp) = Parser::c_lexer(input);
        assert_eq!(dest, None);
        assert_eq!(comp, Some(String::from("blahblahblah")));
        assert_eq!(jmp, None);
    }

    #[test]
    fn parser_acommand() {
        let mut parser = Parser::new();
        let input = "@12345";
        let comm: Command = parser.parse(String::from(input));
        assert_eq!(
            comm,
            Command::new(
                Some(CommandType::ACommand),
                Some(String::from("12345")),
                None,
                None,
                None
            )
        );
    }

    #[test]
    fn parser_acommand_nonnumeric() {
        let st = SymbolTable::new();
        let mut parser = Parser::from(vec![], st);
        let input = "@ABCDE";
        let comm: Command = parser.parse(String::from(input));
        assert_eq!(
            comm,
            Command::new(
                Some(CommandType::ACommand),
                Some(16.to_string()),
                None,
                None,
                None
            )
        );
    }

    #[test]
    fn parser_ccommand() {
        let mut parser = Parser::new();
        let input = "D=D+1";
        let comm: Command = parser.parse(String::from(input));
        assert_eq!(
            comm,
            Command::new(
                Some(CommandType::CCommand),
                None,
                Some(String::from("D")),
                Some(String::from("D+1")),
                None
            )
        );
    }

    #[test]
    fn parser_inline_comment() {
        let mut parser = Parser::new();
        let input = "D=D+1  //Test";
        let comm: Command = parser.parse(String::from(input));
        assert_eq!(
            comm,
            Command::new(
                Some(CommandType::CCommand),
                None,
                Some(String::from("D")),
                Some(String::from("D+1")),
                None
            )
        );
    }

}
