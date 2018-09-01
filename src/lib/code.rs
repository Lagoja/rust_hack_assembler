use std::u16;

pub fn dest(mnemonic: &str) -> Result<u16, &'static str> {
    let result = match mnemonic {
        "M" => 1,
        "D" => 2,
        "MD" => 3,
        "A" => 4,
        "AM" => 5,
        "AD" => 6,
        "AMD" => 7,
        "" => 0,
        _ => return Err("Could not parse dest")
    };

    Ok(result)
}

pub fn jmp(mnemonic: &str) -> Result<u16, &'static str> {
    let result = match mnemonic {
        "JGT" => 8,
        "JEQ" => 16,
        "JGE" => 24,
        "JLT" => 32,
        "JNE" => 40,
        "JLE" => 48,
        "JMP" => 56,
        "" => 0,
        _ => return Err("Could not parse jmp")
    };

    Ok(result)
}

pub fn comp(mnemonic: &str) -> Result<u16, &'static str> {
    let result: u16 = match mnemonic {
        "0" => 2688,
        "1" => 4032,
        "-1" => 3456,
        "D" => 769,
        "!D" => 832,
        "-D" => 960,
        "-A" => 3264,
        "D+1" => 1984,
        "D-1" => 896,
        "A" => 3072,
        "A+1" => 3520,
        "!A" => 3136,
        "A-1" => 3200,
        "D+A" => 128,
        "D-A" => 1216,
        "A-D" => 448,
        "D&A" => 0,
        "D|A" => 1344,
        "M" => 3072 + 4096,
        "M+1" => 3520 + 4096,
        "!M" => 3136 + 4096,
        "M-1" => 3200 + 4096,
        "D+M" => 128 + 4096,
        "D-M" => 1216 + 4096,
        "M-D" => 448 + 4096,
        "D&M" => 0 + 4096,
        "D|M" => 1344 + 4096,
        _ => return Err("Could not parse comp"),
    };

    return Ok(result);
}
