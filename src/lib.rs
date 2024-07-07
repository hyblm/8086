mod parser;
#[cfg(test)]
mod tests;

use crate::parser::parse_instruction;
use core::fmt;
use std::fmt::Display;

pub fn disassemble(program: &[u8]) -> String {
    let mut instruction_stream = (program, 0);
    let mut asm = "bits 16\n\n".to_string();

    while !instruction_stream.0.is_empty() {
        let (tail, instruction) = parse_instruction(instruction_stream).unwrap();
        // println!("{instruction:?}");
        asm.push_str(&format!("{instruction}\n"));

        instruction_stream = tail;
    }
    asm
}

pub struct Instruction {
    _address: usize,
    _size: usize,
    operation: Op,
    destination: Location,
    source: Source,
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} {:?}, {:?}",
            self.operation, self.destination, self.source
        )
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}, {}",
            self.operation, self.destination, self.source
        )
    }
}

#[derive(Debug)]
pub enum Location {
    Reg(Register),
    Addr(EAddress),
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::Reg(reg) => write!(f, "{reg}"),
            Location::Addr(eaddr) => match eaddr {
                EAddress::Bare(addr) => write!(f, "[{addr}]"),
                EAddress::WithOffset(addr, offset) => write!(f, "[{addr} + {offset}]"),
            },
        }
    }
}

#[derive(Debug)]
pub enum Source {
    Loc(Location),
    Imm(Immediate),
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::Loc(loc) => write!(f, "{}", loc),
            Source::Imm(imm) => write!(f, "{imm}"),
        }
    }
}

#[derive(Debug)]
pub enum Immediate {
    Byte(u8),
    Word(u16),
}

impl Display for Immediate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Immediate::Byte(imm) => write!(f, "{imm}"),
            Immediate::Word(imm) => write!(f, "{imm}"),
        }
    }
}

impl From<u8> for Immediate {
    fn from(value: u8) -> Self {
        Immediate::Byte(value)
    }
}

impl From<u16> for Immediate {
    fn from(value: u16) -> Self {
        Immediate::Word(value)
    }
}

#[derive(Debug)]
pub enum Op {
    MovRegRM,
    MovImmediateRM,
    MovImmediateReg,
    Unimplemented,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let opcode_string = match self {
            Op::MovRegRM | Op::MovImmediateRM | Op::MovImmediateReg => "mov",
            _ => "unimplemented!",
        };
        write!(f, "{}", opcode_string)
    }
}

#[derive(Debug)]
pub enum EAddress {
    Bare(Address),
    WithOffset(Address, Immediate),
}

#[derive(Debug, Copy, Clone)]
pub enum Address {
    BxSi,
    BxDi,
    BpSi,
    BpDi,
    Si,
    Di,
    Bp,
    Bx,
}

impl Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Address::BxSi => "bx + si",
            Address::BxDi => "bx + di",
            Address::BpSi => "bp + si",
            Address::BpDi => "bp + di",
            Address::Si => "si",
            Address::Di => "di",
            Address::Bp => "bp",
            Address::Bx => "bx",
        };
        write!(f, "{str}")
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Register {
    Al,
    Dl,
    Cl,
    Bl,
    Ah,
    Dh,
    Ch,
    Bh,
    Ax,
    Dx,
    Cx,
    Bx,
    Sp,
    Bp,
    Si,
    Di,
}

impl Register {
    fn byte(value: u8) -> Self {
        match value {
            0b000 => Self::Al,
            0b001 => Self::Cl,
            0b010 => Self::Dl,
            0b011 => Self::Bl,
            0b100 => Self::Ah,
            0b101 => Self::Ch,
            0b110 => Self::Dh,
            _ => Self::Bh,
        }
    }

    fn word(value: u8) -> Self {
        match value {
            0b000 => Self::Ax,
            0b001 => Self::Cx,
            0b010 => Self::Dx,
            0b011 => Self::Bx,
            0b100 => Self::Sp,
            0b101 => Self::Bp,
            0b110 => Self::Si,
            _ => Self::Di,
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let register_str = match self {
            Register::Al => "al",
            Register::Dl => "dl",
            Register::Cl => "cl",
            Register::Bl => "bl",
            Register::Ah => "ah",
            Register::Dh => "dh",
            Register::Ch => "ch",
            Register::Bh => "bh",
            Register::Ax => "ax",
            Register::Dx => "dx",
            Register::Cx => "cx",
            Register::Bx => "bx",
            Register::Sp => "sp",
            Register::Bp => "bp",
            Register::Si => "si",
            Register::Di => "di",
        };
        write!(f, "{}", register_str)
    }
}
