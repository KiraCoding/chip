use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Token<'t> {
    Delimeter(Delimeter),
    Mnemonic(Mnemonic),
    Number(u16),
    Register(Register),
    Unknown(&'t str),
}

impl<'t> TryFrom<&str> for Token<'t> {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(delimeter) = Delimeter::try_from(value) {
            Ok(Token::Delimeter(delimeter))
        } else if let Ok(instruction) = Mnemonic::try_from(value) {
            Ok(Token::Mnemonic(instruction))
        } else if let Ok(number) = value.parse::<u16>() {
            Ok(Token::Number(number))
        } else if let Ok(register) = Register::try_from(value) {
            Ok(Token::Register(register))
        } else {
            Err(())
        }
    }
}

impl<'t> Display for Token<'t> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Delimeter(_) => todo!(),
            Token::Mnemonic(_) => todo!(),
            Token::Number(_) => todo!(),
            Token::Register(_) => todo!(),
            Token::Unknown(_) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Delimeter {
    Comma,
}

impl<'t> TryFrom<&'t str> for Delimeter {
    type Error = ();

    fn try_from(value: &'t str) -> Result<Self, Self::Error> {
        let delimeter = match value {
            "," => Delimeter::Comma,
            _ => Err(())?,
        };

        Ok(delimeter)
    }
}

#[derive(Debug, PartialEq)]
pub enum Mnemonic {
    Cls,
    Ret,
    Sys,
    Jmp,
    Call,
    Se,
    Sne,
    Ld,
    Add,
    Or,
    And,
    Xor,
    Sub,
    Shr,
    Subn,
    Shl,
    Rnd,
    Drw,
    Skp,
    Sknp,
}

impl TryFrom<&str> for Mnemonic {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let instruction = match value {
            "cls" => Mnemonic::Cls,
            "ret" => Mnemonic::Ret,
            "sys" => Mnemonic::Sys,
            "jmp" => Mnemonic::Jmp,
            "call" => Mnemonic::Call,
            "se" => Mnemonic::Se,
            "sne" => Mnemonic::Sne,
            "ld" => Mnemonic::Ld,
            "add" => Mnemonic::Add,
            "or" => Mnemonic::Or,
            "and" => Mnemonic::And,
            "xor" => Mnemonic::Xor,
            "sub" => Mnemonic::Sub,
            "shr" => Mnemonic::Shr,
            "subn" => Mnemonic::Subn,
            "shl" => Mnemonic::Shl,
            "rnd" => Mnemonic::Rnd,
            "drw" => Mnemonic::Drw,
            "skp" => Mnemonic::Skp,
            "sknp" => Mnemonic::Sknp,
            _ => Err(())?,
        };

        Ok(instruction)
    }
}

impl Display for Mnemonic {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub enum Register {
    V0 = 0x0,
    V1 = 0x1,
    V2 = 0x2,
    V3 = 0x3,
    V4 = 0x4,
    V5 = 0x5,
    V6 = 0x6,
    V7 = 0x7,
    V8 = 0x8,
    V9 = 0x9,
    Va = 0xA,
    Vb = 0xB,
    Vc = 0xC,
    Vd = 0xD,
    Ve = 0xE,
    Vf = 0xF,
}

impl TryFrom<&str> for Register {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let register = match value {
            "v0" => Register::V0,
            "v1" => Register::V1,
            "v2" => Register::V2,
            "v3" => Register::V3,
            "v4" => Register::V4,
            "v5" => Register::V5,
            "v6" => Register::V6,
            "v7" => Register::V7,
            "v8" => Register::V8,
            "v9" => Register::V9,
            "va" => Register::Va,
            "vb" => Register::Vb,
            "vc" => Register::Vc,
            "vd" => Register::Vd,
            "ve" => Register::Ve,
            "vf" => Register::Vf,
            _ => Err(())?,
        };

        Ok(register)
    }
}
