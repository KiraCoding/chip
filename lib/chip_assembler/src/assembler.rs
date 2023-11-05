use crate::parser::{Instruction, Parser};

pub struct Assembler<'a> {
    parser: Parser<'a>,
}

impl<'a> From<Parser<'a>> for Assembler<'a> {
    fn from(parser: Parser<'a>) -> Self {
        Self { parser }
    }
}

impl<'a> Iterator for Assembler<'a> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        self.parser.next().map(|instruction| {
            dbg!(&instruction);
            match instruction {
                Instruction::Cls => 0x00E0,
                Instruction::Ret => 0x00EE,
                Instruction::JmpAddress(address) => 0x1000 | (address & 0x0FFF),
                Instruction::Call(address) => 0x2000 | (address & 0x0FFF),
                Instruction::SeRegVal(vx, nn) => 0x3000 | (vx as u16) << 8 | nn as u16,
                Instruction::SneRegVal(vx, nn) => 0x4000 | (vx as u16) << 8 | nn as u16,
                Instruction::SeRegReg(vx, vy) => 0x5000 | (vx as u16) << 8 | (vy as u16) << 4,
                Instruction::LdRegVal(vx, nn) => 0x6000 | (vx as u16) << 8 | nn as u16,
                Instruction::AddRegVal(vx, nn) => 0x7000 | (vx as u16) << 8 | nn as u16,
                Instruction::LdRegReg(vx, vy) => 0x8000 | (vx as u16) << 8 | (vy as u16) << 4,
                Instruction::Or(vx, vy) => 0x8001 | (vx as u16) << 8 | (vy as u16) << 4,
                Instruction::And(vx, vy) => 0x8002 | (vx as u16) << 8 | (vy as u16) << 4,
                Instruction::Xor(vx, vy) => 0x8003 | (vx as u16) << 8 | (vy as u16) << 4,
                Instruction::AddRegReg(vx, vy) => 0x8004 | (vx as u16) << 8 | (vy as u16) << 4,
                Instruction::Sub(vx, vy) => 0x8005 | (vx as u16) << 8 | (vy as u16) << 4,
                Instruction::Shr(vx, vy) => 0x8006 | (vx as u16) << 8 | (vy as u16) << 4,
                Instruction::Subn(vx, vy) => 0x8007 | (vx as u16) << 8 | (vy as u16) << 4,
                Instruction::Shl(vx, vy) => 0x800E | (vx as u16) << 8 | (vy as u16) << 4,
                Instruction::SneRegReg(vx, vy) => 0x9000 | (vx as u16) << 8 | (vy as u16) << 4,
                Instruction::LdIndex(i, address) => 0xA000 | (i as u16) << 8 | (address & 0x0FFF),
                Instruction::JmpRegAddress(vx, address) => {
                    0xB000 | (vx as u16) << 8 | (address & 0x0FFF)
                }
                Instruction::Rnd(vx, nn) => 0xC000 | (vx as u16) << 8 | (nn as u16),
                Instruction::Drw(vx, vy, nn) => {
                    0xD000 | (vx as u16) << 8 | (vy as u16) << 4 | nn as u16
                }
                Instruction::Skp(vx) => 0xE09E | (vx as u16) << 8,
                Instruction::Skpn(vx) => 0xE0A1 | (vx as u16) << 8,
                Instruction::LdRegDelay(vx, dt) => 0xF007 | (vx as u16) << 8 | (dt as u16) << 4,
                Instruction::LdRegKey(vx, key) => 0xF00A | (vx as u16) << 8 | (key as u16) << 4,
                Instruction::LdDelayReg(dt, vx) => 0xF015 | (vx as u16) << 8 | (dt as u16) << 4,
                Instruction::LdSoundReg(st, vx) => 0xF018 | (vx as u16) << 8 | (st as u16) << 4,
                Instruction::AddIndexReg(i, vx) => 0xF01E | (vx as u16) << 8 | (i as u16) << 4,
                Instruction::LdFReg(vx) => 0xF029 | (vx as u16) << 8,
                Instruction::LdBReg(vx) => 0xF033 | (vx as u16) << 8,
                Instruction::LdMemIndexReg(vx) => 0xF055 | (vx as u16) << 8,
                Instruction::LdRegMemIndex(vx) => 0xF065 | (vx as u16) << 8,
            }
        })
    }
}
