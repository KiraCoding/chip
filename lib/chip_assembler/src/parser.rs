use crate::lexer::Lexer;
use crate::token::{Delimeter, Mnemonic, Register, Token};
use core::fmt::{Display, Formatter};
use core::iter::Peekable;
use std::error::Error;

pub struct Parser<'p> {
    lexer: Peekable<Lexer<'p>>,
}

impl<'p> Parser<'p> {
    fn parse_instruction(&mut self) -> Result<Instruction, ParserError<'p>> {
        match self.parse_mnemonic()? {
            Mnemonic::Cls => Ok(Instruction::Cls),
            Mnemonic::Ret => Ok(Instruction::Ret),
            sys @ Mnemonic::Sys => Err(ParserError::Unsupported(sys)),
            Mnemonic::Jmp => Ok(Instruction::JmpAddress(self.parse_number()?)),
            Mnemonic::Call => Ok(Instruction::Call(self.parse_number()?)),
            Mnemonic::Se => {
                let vx = self.parse_register()?;

                self.parse_token(Token::Delimeter(Delimeter::Comma))?;

                match self.parse_operand()? {
                    Operand::Register(vy) => Ok(Instruction::SeRegReg(vx, vy)),
                    Operand::Number(number) => Ok(Instruction::SeRegVal(vx, number as u8)),
                }
            }
            Mnemonic::Sne => {
                let vx = self.parse_register()?;

                self.parse_token(Token::Delimeter(Delimeter::Comma))?;

                match self.parse_operand()? {
                    Operand::Register(vy) => Ok(Instruction::SneRegReg(vx, vy)),
                    Operand::Number(number) => Ok(Instruction::SneRegVal(vx, number as u8)),
                }
            }
            Mnemonic::Ld => {
                todo!()
            }
            Mnemonic::Add => {
                todo!()
            }
            Mnemonic::Or => {
                let vx = self.parse_register()?;

                self.parse_token(Token::Delimeter(Delimeter::Comma))?;

                let vy = self.parse_register()?;

                Ok(Instruction::Or(vx, vy))
            }
            Mnemonic::And => {
                let vx = self.parse_register()?;

                self.parse_token(Token::Delimeter(Delimeter::Comma))?;

                let vy = self.parse_register()?;

                Ok(Instruction::And(vx, vy))
            }
            Mnemonic::Xor => {
                let vx = self.parse_register()?;

                self.parse_token(Token::Delimeter(Delimeter::Comma))?;

                let vy = self.parse_register()?;

                Ok(Instruction::Xor(vx, vy))
            }
            Mnemonic::Sub => {
                let vx = self.parse_register()?;

                self.parse_token(Token::Delimeter(Delimeter::Comma))?;

                let vy = self.parse_register()?;

                Ok(Instruction::Sub(vx, vy))
            }
            Mnemonic::Shr => {
                let vx = self.parse_register()?;

                self.parse_token(Token::Delimeter(Delimeter::Comma))?;

                let vy = self.parse_register()?;

                Ok(Instruction::Shr(vx, vy))
            }
            instruction => panic!("{:#?}", instruction),
        }
    }

    fn parse_token(&mut self, expected: Token<'p>) -> Result<Token<'p>, ParserError<'p>> {
        match self.lexer.next() {
            Some(token) if token == expected => Ok(token),
            Some(token) => Err(ParserError::Expected(expected, token)),
            None => Err(ParserError::InputEnded(expected)),
        }
    }

    fn parse_mnemonic(&mut self) -> Result<Mnemonic, ParserError<'p>> {
        match self.lexer.next() {
            Some(Token::Mnemonic(mnemonic)) => Ok(mnemonic),
            Some(token) => Err(ParserError::ExpectedMnemonic(token)),
            None => todo!("InputEnded error"),
        }
    }

    fn parse_register(&mut self) -> Result<Register, ParserError<'p>> {
        match self.lexer.next() {
            Some(Token::Register(register)) => Ok(register),
            Some(token) => Err(ParserError::ExpectedRegister(token)),
            None => todo!("InputEnded error"),
        }
    }

    fn parse_number(&mut self) -> Result<u16, ParserError<'p>> {
        match self.lexer.next() {
            Some(Token::Number(number)) => Ok(number),
            Some(token) => Err(ParserError::ExpectedNumber(token)),
            None => todo!("InputEnded error"),
        }
    }

    fn parse_operand(&mut self) -> Result<Operand, ParserError<'p>> {
        match self.lexer.next() {
            Some(Token::Register(register)) => Ok(Operand::Register(register)),
            Some(Token::Number(number)) => Ok(Operand::Number(number)),
            Some(_token) => todo!("Expected operand ParserError"),
            None => todo!("InputEnded error"),
        }
    }
}

impl<'p> Iterator for Parser<'p> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_instruction().ok()
    }
}

impl<'p> From<Lexer<'p>> for Parser<'p> {
    fn from(lexer: Lexer<'p>) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }
}

pub enum Operand {
    Register(Register),
    Number(u16),
}

#[derive(Debug)]
pub enum Instruction {
    Cls,
    Ret,
    JmpAddress(u16),
    Call(u16),
    SeRegVal(Register, u8),
    SneRegVal(Register, u8),
    SeRegReg(Register, Register),
    LdRegVal(Register, u8),
    AddRegVal(Register, u8),
    LdRegReg(Register, Register),
    Or(Register, Register),
    And(Register, Register),
    Xor(Register, Register),
    AddRegReg(Register, Register),
    Sub(Register, Register),
    Shr(Register, Register),
    Subn(Register, Register),
    Shl(Register, Register),
    SneRegReg(Register, Register),
    LdIndex(u8, u16),
    JmpRegAddress(Register, u16),
    Rnd(Register, u8),
    Drw(Register, Register, u8),
    Skp(Register),
    Skpn(Register),
    LdRegDelay(Register, u8),
    LdRegKey(Register, u8),
    LdDelayReg(u8, Register),
    LdSoundReg(u8, Register),
    AddIndexReg(u8, Register),
    LdFReg(Register),
    LdBReg(Register),
    LdMemIndexReg(Register),
    LdRegMemIndex(Register),
}

#[derive(Debug)]
pub enum ParserError<'t> {
    Expected(Token<'t>, Token<'t>),
    ExpectedMnemonic(Token<'t>),
    ExpectedRegister(Token<'t>),
    ExpectedNumber(Token<'t>),
    InputEnded(Token<'t>),
    Unsupported(Mnemonic),
}

impl<'t> Display for ParserError<'t> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expected(expected, found) => {
                write!(f, "Expected {:?}, but found {:?}", expected, found)
            }
            Self::ExpectedMnemonic(found) => {
                writeln!(f, "Expected mnemonic, but found {:?}", found)
            }
            Self::ExpectedRegister(found) => write!(f, "Expected register, but found {:?}", found),
            Self::ExpectedNumber(found) => write!(f, "Expected number, but found {:?}", found),
            Self::InputEnded(token) => write!(f, "Expected {:?}, but the input has ended", token),
            Self::Unsupported(mnemonic) => {
                write!(f, "The instruction {:?} isn't supported", mnemonic)
            }
        }
    }
}

impl<'t> Error for ParserError<'t> {}
