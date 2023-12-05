use crate::lexer::Lexer;
use crate::token::{Delimeter, Mnemonic, Register, Token};
use core::fmt::{Display, Formatter};
use core::iter::Peekable;
use std::error::Error;
use std::fmt::Debug;

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
                    Operand::Number(number) => Ok(Instruction::SeRegVal(vx, number)),
                }
            }
            Mnemonic::Sne => {
                let vx = self.parse_register()?;

                self.parse_token(Token::Delimeter(Delimeter::Comma))?;

                match self.parse_operand()? {
                    Operand::Register(vy) => Ok(Instruction::SneRegReg(vx, vy)),
                    Operand::Number(number) => Ok(Instruction::SneRegVal(vx, number)),
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
        while let Some(_) = self.lexer.peek() {
            if let Ok(instruction) = self.parse_instruction() {
                return Some(instruction);
            } else {
                // Consume tokens until a valid instruction or end of input
                self.lexer.next();
            }
        }
        None
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
    SeRegVal(Register, u16),
    SneRegVal(Register, u16),
    SeRegReg(Register, Register),
    LdRegVal(Register, u16),
    AddRegVal(Register, u16),
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
    LdIndex(u16, u16),
    JmpRegAddress(Register, u16),
    Rnd(Register, u16),
    Drw(Register, Register, u16),
    Skp(Register),
    Skpn(Register),
    LdRegDelay(Register, u16),
    LdRegKey(Register, u16),
    LdDelayReg(u16, Register),
    LdSoundReg(u16, Register),
    AddIndexReg(u16, Register),
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

#[derive(Debug)]
pub enum ParserError2<D>
where
    D: Debug,
{
    Expected(D, D),
    ExpectedMnemonic(D),
    ExpectedRegister(D),
    ExpectedNumber(D),
    InputEnded(D),
    Unsupported(Mnemonic),
}