use regex::Regex;

pub fn comment_re() -> Regex {
    Regex::new(r"//.*$").unwrap()
}

pub fn pseudocommand_re() -> Regex {
    Regex::new(r"\(\D*\)").unwrap()
}

pub fn pseudocommand_inner_re() -> Regex {
    Regex::new(r"\b.*\b").unwrap()
}

pub fn acommand_re() -> Regex {
    Regex::new(r"^@.*").unwrap()
}

pub fn acommand_symbol_re() -> Regex {
    Regex::new(r"\b.+").unwrap()
}

pub fn ccommand_re() -> Regex {
    Regex::new(r"(^.*=.*|^.*;.*)").unwrap()
}

pub fn numeric_asymbol_re() -> Regex{
    Regex::new(r"^\d*$").unwrap()
}