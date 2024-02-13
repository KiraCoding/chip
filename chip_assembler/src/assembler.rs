use chip_lexer::lexer::Lexer;

use crate::parser::{Instruction, Parser};

pub struct Assembler<'a> {
    parser: Parser<'a>,
}

impl<'a> From<Parser<'a>> for Assembler<'a> {
    fn from(parser: Parser<'a>) -> Self {
        Self { parser }
    }
}

impl<'a> From<&'a str> for Assembler<'a> {
    fn from(value: &'a str) -> Self {
        let lexer = Lexer::from(value);
        let parser = Parser::from(lexer);

        Self { parser }
    }
}

impl<'a> Iterator for Assembler<'a> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        self.parser.next().map(|instruction| match instruction {
            Instruction::Cls => 0x00E0,
            Instruction::Ret => 0x00EE,
            Instruction::JmpAddress(address) => 0x1000 | (address & 0x0FFF),
            Instruction::Call(address) => 0x2000 | (address & 0x0FFF),
            Instruction::SeRegVal(vx, nn) => 0x3000 | vx << 8 | nn,
            Instruction::SneRegVal(vx, nn) => 0x4000 | vx << 8 | nn,
            Instruction::SeRegReg(vx, vy) => 0x5000 | vx << 8 | vy << 4,
            Instruction::LdRegVal(vx, nn) => 0x6000 | vx << 8 | nn,
            Instruction::AddRegVal(vx, nn) => 0x7000 | vx << 8 | nn,
            Instruction::LdRegReg(vx, vy) => 0x8000 | vx << 8 | vy << 4,
            Instruction::Or(vx, vy) => 0x8001 | vx << 8 | vy << 4,
            Instruction::And(vx, vy) => 0x8002 | vx << 8 | vy << 4,
            Instruction::Xor(vx, vy) => 0x8003 | vx << 8 | vy << 4,
            Instruction::AddRegReg(vx, vy) => 0x8004 | vx << 8 | vy << 4,
            Instruction::Sub(vx, vy) => 0x8005 | vx << 8 | vy << 4,
            Instruction::Shr(vx, vy) => 0x8006 | vx << 8 | vy << 4, 
            Instruction::Subn(vx, vy) => 0x8007 | vx << 8 | vy << 4,
            Instruction::Shl(vx, vy) => 0x800E | vx << 8 | vy << 4,
            Instruction::SneRegReg(vx, vy) => 0x9000 | vx << 8 | vy << 4,
            Instruction::LdIndex(i, address) => 0xA000 | i << 8 | (address & 0x0FFF),
            Instruction::JmpRegAddress(vx, address) => 0xB000 | vx << 8 | (address & 0x0FFF),
            Instruction::Rnd(vx, nn) => 0xC000 | vx << 8 | nn,
            Instruction::Drw(vx, vy, nn) => 0xD000 | vx << 8 | vy << 4 | nn,
            Instruction::Skp(vx) => 0xE09E | vx << 8,
            Instruction::Skpn(vx) => 0xE0A1 | vx << 8,
            Instruction::LdRegDelay(vx, dt) => 0xF007 | vx << 8 | dt << 4,
            Instruction::LdRegKey(vx, key) => 0xF00A | vx << 8 | key << 4,
            Instruction::LdDelayReg(dt, vx) => 0xF015 | vx << 8 | dt << 4,
            Instruction::LdSoundReg(st, vx) => 0xF018 | vx << 8 | st << 4,
            Instruction::AddIndexReg(i, vx) => 0xF01E | vx << 8 | i << 4,
            Instruction::LdFReg(vx) => 0xF029 | vx << 8,
            Instruction::LdBReg(vx) => 0xF033 | vx << 8,
            Instruction::LdMemIndexReg(vx) => 0xF055 | vx << 8,
            Instruction::LdRegMemIndex(vx) => 0xF065 | vx << 8,
        })
    }
}
