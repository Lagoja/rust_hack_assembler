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

pub fn ccommand_re() -> Regex {
    Regex::new(r"(^.*=.*|^.*;.*)").unwrap()
}
