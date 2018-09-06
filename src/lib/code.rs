use lazy_static::*;
use std::collections::HashMap;
use std::u16;

lazy_static! {
    static ref DEST_MAP: HashMap<&'static str, u16> = [
        ("M", 8),
        ("D", 16),
        ("MD", 24),
        ("A", 32),
        ("AM", 40),
        ("AD", 48),
        ("AMD", 56),
        ("", 0),
    ].iter().cloned().collect();

    static ref JMP_MAP: HashMap<&'static str, u16> = [
        ("JGT" , 1),
        ("JEQ" , 2),
        ("JGE" , 3),
        ("JLT" , 4),
        ("JNE" , 5),
        ("JLE" , 6),
        ("JMP" , 7),
        ("" , 0)
    ].iter().cloned().collect();

    static ref COMP_MAP: HashMap<&'static str, u16> = [
        ("0" , 2688),
        ("1" , 4032),
        ("-1", 3712),
        ("D" , 768),
        ("!D", 832),
        ("-D" , 960),
        ("-A" , 3264),
        ("D+1", 1984),
        ("D-1" , 896),
        ("A" , 3072),
        ("A+1" , 3520),
        ("!A" , 3136),
        ("A-1" , 3200),
        ("D+A" , 128),
        ("D-A" , 1216),
        ("A-D" , 448),
        ("D&A" , 0),
        ("D|A" , 1344),
        ("M" , 3072 + 4096),
        ("M+1" , 3520 + 4096),
        ("!M" , 3136 + 4096),
        ("M-1" , 3200 + 4096),
        ("D+M" , 128 + 4096),
        ("D-M" , 1216 + 4096),
        ("M-D" , 448 + 4096),
        ("D&M" , 0 + 4096),
        ("D|M" , 1344 + 4096),
    ].iter().cloned().collect();
}

pub fn dest(mnemonic: &str) -> Result<u16, &'static str> {
    match DEST_MAP.get(mnemonic) {
        Some(addr) => Ok(*addr),
        None => return Err ("Could not parse dest"),
    }
}

pub fn jmp(mnemonic: &str) -> Result<u16, &'static str> {
    match JMP_MAP.get(mnemonic) {
        Some(addr) => Ok(*addr),
        None => return Err ("Could not parse jmp"),
    }
}

pub fn comp(mnemonic: &str) -> Result<u16, &'static str> {
    match COMP_MAP.get(mnemonic) {
        Some(addr) => Ok(*addr),
        None => return Err("Could not parse comp"),
    }
}
