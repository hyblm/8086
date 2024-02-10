use crate::{Address, EAddress, Immediate, Instruction, Location, Op, Register, Source};

use winnow::{
    bits::{bool, take},
    error::ParseError,
    stream::{AsBytes, Stream, StreamIsPartial},
    IResult, Parser,
};

pub type BitInput<'a> = (&'a [u8], usize);

pub fn parse_instruction(i: BitInput) -> IResult<BitInput, Instruction> {
    let (i, opcode) = parse_opcode(i)?;
    let (i, destination, source) = match opcode {
        Op::MovRegRM => {
            let (i, (d_bit, is_word, mode)) = (bool, bool, take_2bits).parse_next(i)?;
            let (i, reg) = parse_reg(is_word).parse_next(i)?;
            let (i, rm) = parse_rm(mode, is_word, i)?;
            if d_bit {
                (i, Location::Reg(reg), Source::Loc(rm))
            } else {
                (i, rm, Source::Loc(Location::Reg(reg)))
            }
        }
        Op::MovImmediateReg => {
            let (i, is_word) = bool(i)?;
            let (i, reg) = parse_reg(is_word).parse_next(i)?;
            let (i, val) = parse_immediate(i, is_word)?;
            (i, Location::Reg(reg), Source::Imm(val))
        }
        Op::MovImmediateRM => todo!(),
        Op::Unimplemented => todo!(),
    };
    let instruction = Instruction {
        _address: 0,
        _size: 0,
        operation: opcode,
        destination,
        source,
    };
    Ok((i, instruction))
}

fn parse_immediate(i: BitInput, is_word: bool) -> IResult<BitInput, Immediate> {
    let (i, low) = take(8u8).parse_next(i)?;
    Ok(if !is_word {
        (i, Immediate::Byte(low))
    } else {
        let (i, high): (BitInput, u16) = take(8u8).parse_next(i)?;
        let high = high << 8;
        let word = high + u16::from(low);
        (i, Immediate::Word(word))
    })
}

// TODO(matyas): remove is_word and mode parameters from RM parser
fn parse_rm(mode: u8, w_bit: bool, i: BitInput) -> IResult<BitInput, Location> {
    assert!(mode <= 3);
    if let 0b11 = mode {
        let (i, reg) = parse_reg(w_bit).parse_next(i)?;
        Ok((i, Location::Reg(reg)))
    } else {
        let (i, addr) = parse_addr(i)?;
        let (i, eaddr) = parse_eaddr(i, mode, addr)?;
        Ok((i, Location::Addr(eaddr)))
    }
}

fn parse_eaddr(i: BitInput, mode: u8, addr: Address) -> IResult<BitInput, EAddress> {
    let (i, eaddr) = if mode == 0 {
        (i, EAddress::Bare(addr))
    } else {
        let is_word = mode == 0b10;
        let (i, imm) = parse_immediate(i, is_word)?;
        (i, EAddress::WithOffset(addr, imm))
    };

    Ok((i, eaddr))
}

fn parse_addr(i: BitInput) -> IResult<BitInput, Address> {
    let (i, addr) = take_3bits(i)?;
    use Address::*;
    let addr = match addr {
        0b000 => BxSi,
        0b001 => BxDi,
        0b010 => BpSi,
        0b011 => BpDi,
        0b100 => Si,
        0b101 => Di,
        0b110 => Bp,
        _ => Bx,
    };
    Ok((i, addr))
}

pub fn take_nibble(i: BitInput) -> IResult<BitInput, u8> {
    take(4u8).parse_next(i)
}

pub fn take_3bits(i: BitInput) -> IResult<BitInput, u8> {
    take(3u8).parse_next(i)
}

pub fn take_2bits(i: BitInput) -> IResult<BitInput, u8> {
    take(2u8).parse_next(i)
}

pub fn parse_reg<I, E: ParseError<(I, usize)>>(w_bit: bool) -> impl Parser<(I, usize), Register, E>
where
    I: Stream<Token = u8> + AsBytes + StreamIsPartial,
{
    Parser::map(
        take(3u8),
        if w_bit {
            Register::word
        } else {
            Register::byte
        },
    )
}

pub fn parse_opcode(i: BitInput) -> IResult<BitInput, Op> {
    let (i, partial) = take_nibble(i)?;
    let (i, opcode) = match partial {
        0b1000 => {
            let (i, _) = take_2bits(i)?;
            (i, Op::MovRegRM)
        }
        0b1011 => (i, Op::MovImmediateReg),
        0b1100 => todo!("Immediate to register/memory"),
        0b1010 => todo!("Memory to/from accumulator"),
        _ => {
            println!("partial: {partial:0b}");
            println!("input: {:?}", i.0);
            (i, Op::Unimplemented)
        }
    };
    Ok((i, opcode))
}
