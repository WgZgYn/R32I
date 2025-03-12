#![allow(dead_code)]

use crate::instruction_type::{BInstruction, IInstruction, ISInstruction, RInstruction};
use crate::{Address, Registers};

pub struct ALU<'a>(&'a mut Registers);

impl<'a> ALU<'a> {
    pub const fn with(registers: &'a mut Registers) -> Self {
        Self(registers)
    }

    const fn add(&mut self, r: RInstruction) {
        *self.0.get_mut(r.rd()) = self.0.get(r.rs1()).overflowing_add(self.0.get(r.rs2())).0
    }

    const fn sub(&mut self, r: RInstruction) {
        *self.0.get_mut(r.rd()) = self.0.get(r.rs1()).overflowing_sub(self.0.get(r.rs2())).0
    }

    const fn xor(&mut self, r: RInstruction) {
        *self.0.get_mut(r.rd()) = self.0.get(r.rs1()) ^ self.0.get(r.rs2())
    }

    const fn and(&mut self, r: RInstruction) {
        *self.0.get_mut(r.rd()) = self.0.get(r.rs1()) & self.0.get(r.rs2())
    }

    const fn or(&mut self, r: RInstruction) {
        *self.0.get_mut(r.rd()) = self.0.get(r.rs1()) | self.0.get(r.rs2())
    }

    const fn sll(&mut self, r: RInstruction) {
        *self.0.get_mut(r.rd()) = self.0.get(r.rs1()) << self.0.get(r.rs2())
    }

    const fn slt(&mut self, r: RInstruction) {
        *self.0.get_mut(r.rd()) = if (self.0.get(r.rs1()) as i32) < (self.0.get(r.rs2()) as i32) {
            1
        } else {
            0
        }
    }

    const fn sltu(&mut self, r: RInstruction) {
        *self.0.get_mut(r.rd()) = if self.0.get(r.rs1()) < self.0.get(r.rs2()) {
            1
        } else {
            0
        }
    }

    const fn srl(&mut self, r: RInstruction) {
        *self.0.get_mut(r.rd()) = self.0.get(r.rs1()) >> self.0.get(r.rs2())
    }

    const fn sra(&mut self, r: RInstruction) {
        *self.0.get_mut(r.rd()) =
            ((self.0.get(r.rs1()) as i32) >> (self.0.get(r.rs2()) as i32)) as u32;
    }

    // TODO: fix btype
    const fn beq(&mut self, pc: &mut Address, b: BInstruction) {
        if *self.0.get_mut(b.rs1()) == self.0.get(b.rs2()) {
            *pc = ((*pc - 4) as i32 + b.imm()) as u32;
        }
    }

    const fn bne(&mut self, pc: &mut Address, b: BInstruction) {
        if *self.0.get_mut(b.rs1()) != self.0.get(b.rs2()) {
            *pc = ((*pc - 4) as i32 + b.imm()) as u32;
        }
    }

    const fn blt(&mut self, pc: &mut Address, b: BInstruction) {
        if (*self.0.get_mut(b.rs1()) as i32) < (self.0.get(b.rs2()) as i32) {
            *pc = ((*pc - 4) as i32 + b.imm()) as u32;
        }
    }

    const fn bge(&mut self, pc: &mut Address, b: BInstruction) {
        if (*self.0.get_mut(b.rs1()) as i32) >= (self.0.get(b.rs2()) as i32) {
            *pc = ((*pc - 4) as i32 + b.imm()) as u32;
        }
    }

    const fn bltu(&mut self, pc: &mut Address, b: BInstruction) {
        if *self.0.get_mut(b.rs1()) < self.0.get(b.rs2()) {
            *pc = ((*pc - 4) as i32 + b.imm()) as u32;
        }
    }

    const fn bgeu(&mut self, pc: &mut Address, b: BInstruction) {
        if *self.0.get_mut(b.rs1()) >= self.0.get(b.rs2()) {
            *pc = ((*pc - 4) as i32 + b.imm()) as u32;
        }
    }

    const fn addi(&mut self, i: IInstruction) {
        *self.0.get_mut(i.rd()) = (self.0.get(i.rs1()) as i32 + i.imm()) as u32;
    }

    const fn slti(&mut self, i: IInstruction) {
        *self.0.get_mut(i.rd()) = if (self.0.get(i.rs1()) as i32) < i.imm() {
            1
        } else {
            0
        };
    }

    const fn sltiu(&mut self, i: IInstruction) {
        *self.0.get_mut(i.rd()) = if self.0.get(i.rs1()) < i.umm() { 1 } else { 0 };
    }

    const fn xori(&mut self, i: IInstruction) {
        *self.0.get_mut(i.rd()) = self.0.get(i.rs1()) ^ i.umm();
    }

    const fn ori(&mut self, i: IInstruction) {
        *self.0.get_mut(i.rd()) = self.0.get(i.rs1()) | i.umm();
    }

    const fn andi(&mut self, i: IInstruction) {
        *self.0.get_mut(i.rd()) = self.0.get(i.rs1()) & i.umm();
    }

    const fn slli(&mut self, i: ISInstruction) {
        *self.0.get_mut(i.rd()) = self.0.get(i.rs1()) << (i.umm() & 0b11111);
    }

    const fn srli(&mut self, i: ISInstruction) {
        *self.0.get_mut(i.rd()) = self.0.get(i.rs1()) >> (i.umm() & 0b11111);
    }

    const fn srai(&mut self, i: ISInstruction) {
        *self.0.get_mut(i.rd()) = (self.0.get(i.rs1()) as i32 >> (i.umm() & 0b11111)) as u32;
    }

    pub const fn execute(&mut self, r: RInstruction) {
        match (r.funct3(), r.funct7()) {
            (0b000, 0) => self.add(r),
            (0b000, 0x20) => self.sub(r),
            (0b001, 0) => self.sll(r),
            (0b010, 0) => self.slt(r),
            (0b011, 0) => self.sltu(r),
            (0b100, 0) => self.xor(r),
            (0b101, 0) => self.srl(r),
            (0b101, 0x20) => self.sra(r),
            (0b110, 0) => self.or(r),
            (0b111, 0) => self.and(r),
            _ => {
                panic!("no such instruction");
            }
        }
    }

    pub const fn branch(&mut self, pc: &mut Address, b: BInstruction) {
        match b.funct3() {
            0b000 => self.beq(pc, b),
            0b001 => self.bne(pc, b),
            0b100 => self.blt(pc, b),
            0b101 => self.bge(pc, b),
            0b110 => self.bltu(pc, b),
            0b111 => self.bgeu(pc, b),
            _ => panic!("branch unknown funct3"),
        }
    }

    pub const fn immediate(&mut self, i: IInstruction) {
        match i.funct3() {
            0b000 => self.addi(i),
            0b010 => self.slti(i),
            0b011 => self.sltiu(i),
            0b100 => self.xori(i),
            0b110 => self.ori(i),
            0b111 => self.andi(i),
            0b001 => self.slli(i.as_s()),
            0b101 => {
                if i.imm() >> 5 == 0 {
                    self.srli(i.as_s())
                } else {
                    self.srai(i.as_s())
                }
            }
            _ => panic!("immediate unknown funct3"),
        }
    }
}
