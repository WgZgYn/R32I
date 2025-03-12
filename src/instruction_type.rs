#![allow(dead_code)]

use crate::arch::{Byte, R32I};
use crate::mask;
use crate::opcode::{AUIPC, B_TYPE, I_TYPE, JALR, J_TYPE, LUI, NOP, RI_TYPE, R_TYPE, S_TYPE};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Default)]
pub struct Instruction(pub R32I);

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // f.write_fmt(format_args!("instructions: {:032b}\n", self.0))?;
        match self.opcode() as Byte {
            I_TYPE => write!(f, "{}", self.as_i())?,
            RI_TYPE => write!(f, "{}", self.as_i())?,
            R_TYPE => write!(f, "{}", self.as_r())?,
            S_TYPE => write!(f, "{}", self.as_s())?,
            B_TYPE => write!(f, "{}", self.as_b())?,
            J_TYPE => {
                let j = self.as_j();
                write!(f, "j: rd: {}, imm: {}", j.rd(), j.imm())?;
            }
            JALR => {
                let i = self.as_i();
                write!(f, "jalr: rs1: {}, imm: {}", i.rs1(), i.imm())?;
            }
            LUI => {
                let u = self.as_u();
                write!(f, "lui: rd: {}, imm: {}", u.rd(), u.high_imm())?;
            }
            AUIPC => {
                let u = self.as_u();
                write!(f, "lui: rd: {}, imm: {}", u.rd(), u.high_imm())?;
            }
            NOP => f.write_str("stop")?,
            _ => unimplemented!(),
        }
        Ok(())
    }
}

impl Display for IInstruction<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let op = match self.0.opcode() as Byte {
            RI_TYPE => match self.funct3() {
                0b000 => "addi",
                0b010 => "slti",
                0b011 => "sltiu",
                0b100 => "xori",
                0b110 => "ori",
                0b111 => "andi",
                0b001 => "slli",
                0b101 => {
                    if self.imm() >> 5 == 0 {
                        "srli"
                    } else {
                        "srai"
                    }
                }
                _ => unreachable!(),
            },
            I_TYPE => match self.funct3() {
                0b000 => "lb",
                0b001 => "lh",
                0b010 => "lw",
                0b100 => "lbu",
                0b101 => "lhu",
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        write!(
            f,
            "{}, rd: {}, rs1: {}, imm: {}",
            op,
            self.rd(),
            self.rs1(),
            self.imm()
        )
    }
}

impl Display for RInstruction<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let op = match (self.funct3(), self.funct7()) {
            (0b000, 0) => "add",
            (0b000, 0x20) => "sub",
            (0b001, 0) => "sll",
            (0b010, 0) => "slt",
            (0b011, 0) => "sltu",
            (0b100, 0) => "xor",
            (0b101, 0) => "srl",
            (0b101, 0x20) => "sra",
            (0b110, 0) => "or",
            (0b111, 0) => "and",
            _ => {
                panic!("no such instruction");
            }
        };
        write!(
            f,
            "{}: rd: {}, rs1: {}, rs2: {}",
            op,
            self.rd(),
            self.rs1(),
            self.rs2()
        )
    }
}

impl Display for BInstruction<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let op = match self.funct3() {
            0b000 => "beq",
            0b001 => "bne",
            0b100 => "blt",
            0b101 => "bge",
            0b110 => "bltu",
            0b111 => "bgeu",
            _ => panic!("branch unknown funct3"),
        };
        write!(
            f,
            "{}: rs1: {}, rs2: {}, imm: {}",
            op,
            self.rs1(),
            self.rs2(),
            self.imm()
        )
    }
}

impl Display for SInstruction<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let op = match self.funct3() {
            0b000 => "sb",
            0b001 => "sh",
            0b010 => "sw",
            _ => unreachable!(),
        };
        write!(
            f,
            "{}: rs1: {}, rs2: {}, imm: {}",
            op,
            self.rs1(),
            self.rs2(),
            self.imm()
        )
    }
}

impl Instruction {
    pub const fn from(data: R32I) -> Self {
        Self(data)
    }
}
pub struct RInstruction<'a>(pub &'a Instruction);
pub struct IInstruction<'a>(pub &'a Instruction);
pub struct ISInstruction<'a>(pub &'a Instruction);
pub struct SInstruction<'a>(pub &'a Instruction);
pub struct BInstruction<'a>(pub &'a Instruction);
pub struct UInstruction<'a>(pub &'a Instruction);
pub struct JInstruction<'a>(pub &'a Instruction);

impl Instruction {
    pub const fn opcode(&self) -> u32 {
        self.0 & mask::OPCODE
    }

    pub const fn as_r(&self) -> RInstruction {
        RInstruction(self)
    }

    pub const fn as_i(&self) -> IInstruction {
        IInstruction(self)
    }

    pub const fn as_j(&self) -> JInstruction {
        JInstruction(self)
    }

    pub const fn as_b(&self) -> BInstruction {
        BInstruction(self)
    }

    pub const fn as_u(&self) -> UInstruction {
        UInstruction(self)
    }

    pub const fn as_s(&self) -> SInstruction {
        SInstruction(self)
    }
    pub const fn mask(&self, mask: u32) -> u32 {
        self.0 & mask
    }

    pub const fn range(&self, range: std::ops::RangeInclusive<u8>) -> u32 {
        self.mask(mask::range_mask(*range.start(), *range.end())) >> *range.start()
    }
}

impl RInstruction<'_> {
    pub const fn rd(&self) -> u8 {
        (self.0.mask(mask::RD) >> 7) as u8
    }

    pub const fn funct3(&self) -> u8 {
        (self.0.mask(mask::FUNCT3) >> 12) as u8
    }

    pub const fn rs1(&self) -> u8 {
        (self.0.mask(mask::RS1) >> 15) as u8
    }

    pub const fn rs2(&self) -> u8 {
        (self.0.mask(mask::RS2) >> 20) as u8
    }

    pub const fn funct7(&self) -> u8 {
        ((self.0.mask(mask::FUNCT7) >> 25) & 0x7F) as u8
    }
}

impl IInstruction<'_> {
    pub const fn rd(&self) -> u8 {
        (self.0.mask(mask::RD) >> 7) as u8
    }

    pub const fn funct3(&self) -> u32 {
        self.0.mask(mask::FUNCT3) >> 12
    }

    pub const fn rs1(&self) -> u8 {
        (self.0.mask(mask::RS1) >> 15) as u8
    }

    // need sign extend
    pub const fn imm(&self) -> i32 {
        (self.0.mask(mask::IMM11_0) as i32) >> 20
    }

    pub const fn umm(&self) -> u32 {
        self.0.mask(mask::IMM11_0) >> 20
    }

    pub const fn as_s(&self) -> ISInstruction {
        ISInstruction(self.0)
    }
}

impl ISInstruction<'_> {
    pub const fn rd(&self) -> u8 {
        (self.0.mask(mask::RD) >> 7) as u8
    }

    pub const fn funct3(&self) -> u32 {
        self.0.mask(mask::FUNCT3) >> 12
    }

    pub const fn rs1(&self) -> u8 {
        (self.0.mask(mask::RS1) >> 15) as u8
    }

    pub const fn umm(&self) -> u8 {
        self.0.range(20..=24) as u8
    }
}

impl SInstruction<'_> {
    pub const fn funct3(&self) -> u32 {
        self.0.mask(mask::FUNCT3) >> 12
    }

    pub const fn rs1(&self) -> u8 {
        (self.0.mask(mask::RS1) >> 15) as u8
    }

    pub const fn rs2(&self) -> u8 {
        (self.0.mask(mask::RS2) >> 20) as u8
    }

    pub const fn imm(&self) -> i32 {
        let part1 = self.0.range(7..=11);
        let part2 = self.0.range(25..=31);
        ((part2 << 5 | part1) as i32) << 20 >> 20
    }

    pub const fn umm(&self) -> u32 {
        let part1 = self.0.range(7..=11);
        let part2 = self.0.range(25..=31);
        part2 << 5 | part1
    }
}

impl BInstruction<'_> {
    pub const fn rs1(&self) -> u8 {
        ((self.0.mask(mask::RS1) >> 15) & 0x1F) as u8
    }

    pub const fn rs2(&self) -> u8 {
        ((self.0.mask(mask::RS2) >> 20) & 0x1F) as u8
    }

    pub const fn funct3(&self) -> u8 {
        (self.0.mask(mask::FUNCT3) >> 12) as u8
    }

    pub const fn imm(&self) -> i32 {
        let part1 = self.0.range(8..=11) << 1;
        let part2 = self.0.range(25..=30) << 5;
        let part3 = self.0.range(7..=7) << 11;
        let part4 = self.0.range(31..=31) << 12;
        ((part1 | part2 | part3 | part4) as i32) << 19 >> 19
    }

    pub const fn umm(&self) -> u32 {
        let part1 = self.0.range(8..=11) << 1;
        let part2 = self.0.range(25..=30) << 5;
        let part3 = self.0.range(7..=7) << 11;
        let part4 = self.0.range(31..=31) << 12;
        part1 | part2 | part3 | part4
    }
}

impl UInstruction<'_> {
    pub const fn rd(&self) -> u8 {
        (self.0.mask(mask::RD) >> 7) as u8
    }
    pub const fn high_imm(&self) -> u32 {
        self.0.mask(mask::IMM32_12)
    }
}

impl JInstruction<'_> {
    pub const fn rd(&self) -> u8 {
        (self.0.mask(mask::RD) >> 7) as u8
    }

    pub const fn imm(&self) -> i32 {
        let part1 = self.0.range(31..=31) << 20;
        let part2 = self.0.range(21..=30) << 1;
        let part3 = self.0.range(20..=20) << 11;
        let part4 = self.0.range(12..=19) << 12;
        ((part1 | part2 | part3 | part4) as i32) << 12 >> 12
    }
}
