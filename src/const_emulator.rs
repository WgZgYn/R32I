use crate::alu::ALU;
use crate::arch::{Address, Byte, PC_STEP};
use crate::instruction_type::{IInstruction, Instruction, SInstruction};
use crate::mask::{BYTE_MASK, HALF_WORD_MASK, WORD_MASK};
use crate::opcode::{AUIPC, B_TYPE, I_TYPE, J_TYPE, JALR, LUI, NOP, R_TYPE, RI_TYPE, S_TYPE};
use crate::register::Registers;

pub struct ConstantEmulator;
impl ConstantEmulator {
    const fn run(
        regs: &mut Registers,
        data: &mut [u32],
        code_len: usize,
        pc: &mut u32,
        stop: &mut bool,
    ) {
        if *pc >> 2 >= code_len as u32 {
            *stop = true;
            return;
        }
        let instruction = Instruction(data[(*pc >> 2) as usize]);

        match instruction.opcode() as Byte {
            R_TYPE => ALU::with(regs).execute(instruction.as_r()),
            RI_TYPE => ALU::with(regs).immediate(instruction.as_i()),
            B_TYPE => ALU::with(regs).branch(pc, instruction.as_b()),
            I_TYPE => {
                let i = instruction.as_i();
                match i.funct3() {
                    0b000 => Self::lb(regs, i, data),
                    0b001 => Self::lh(regs, i, data),
                    0b010 => Self::lw(regs, i, data),
                    0b100 => Self::lbu(regs, i, data),
                    0b101 => Self::lhu(regs, i, data),
                    _ => panic!("Unexpected instruction type I_TYPE "),
                }
            }
            S_TYPE => {
                let s = instruction.as_s();
                match s.funct3() {
                    0b000 => Self::sb(regs, s, data),
                    0b001 => Self::sh(regs, s, data),
                    0b010 => Self::sw(regs, s, data),
                    _ => panic!("no such instruction S_TYPE"),
                }
            }
            J_TYPE => {
                let j = instruction.as_j();
                *regs.get_mut(j.rd()) = *pc;
                *pc = ((*pc - PC_STEP) as i32 + j.imm()) as u32;
            }
            JALR => {
                let i = instruction.as_i();
                *regs.get_mut(i.rd()) = *pc;
                *pc = (((regs.get(i.rs1())) & !1) as i32 + i.imm()) as Address;
            }
            LUI => {
                let u = instruction.as_u();
                *regs.get_mut(u.rd()) = (regs.get(u.rd()) & 0xFFF) + u.high_imm();
            }
            AUIPC => {
                let u = instruction.as_u();
                *regs.get_mut(u.rd()) = *pc + u.high_imm();
            }
            NOP => *stop = true,
            _ => *stop = true,
        }
    }
    pub const fn run_loop<const N: usize>(code: &[u32; N]) -> u32 {
        assert!(N < 1 << 16);
        let mut regs = Registers::default();
        let mut pc = 0;
        let mut stop = false;
        let mut data = [0; 1 << 21];
        data.first_chunk_mut::<N>().unwrap().copy_from_slice(code);
        *regs.sp() = ((data.len() as u32) << 2) - 100;
        while !stop {
            Self::run(&mut regs, &mut data, code.len(), &mut pc, &mut stop);
            pc += 4;
        }
        *regs.a(0)
    }
    const fn lb(registers: &mut Registers, i: IInstruction, data: &[u32]) {
        let base = registers.get(i.rs1()) as u32;
        let offset = i.imm();
        let target = (base as i32 + offset) as u32;
        let data = data[(target >> 2) as usize];
        *registers.get_mut(i.rd()) = (data) << 24 >> 24;
    }

    const fn lbu(registers: &mut Registers, i: IInstruction, data: &[u32]) {
        let base = registers.get(i.rs1()) as u32;
        let offset = i.imm();
        let target = (base as i32 + offset) as u32;
        let data = data[(target >> 2) as usize];
        *registers.get_mut(i.rd()) = data;
    }

    const fn lh(registers: &mut Registers, i: IInstruction, data: &[u32]) {
        let base = registers.get(i.rs1()) as u32;
        let offset = i.imm();
        let target = (base as i32 + offset) as u32;
        let data = data[(target >> 2) as usize];
        *registers.get_mut(i.rd()) = (data) << 16 >> 16;
    }

    const fn lhu(registers: &mut Registers, i: IInstruction, data: &[u32]) {
        let base = registers.get(i.rs1()) as u32;
        let offset = i.imm();
        let target = (base as i32 + offset) as u32;
        let data = data[(target >> 2) as usize];
        *registers.get_mut(i.rd()) = data;
    }

    const fn lw(registers: &mut Registers, i: IInstruction, data: &[u32]) {
        let base = registers.get(i.rs1()) as u32;
        let offset = i.imm();
        let target = (base as i32 + offset) as u32;
        let data = data[(target >> 2) as usize];
        *registers.get_mut(i.rd()) = data;
    }

    const fn sb(registers: &mut Registers, s: SInstruction, data: &mut [u32]) {
        let base = registers.get(s.rs1()) as u32;
        let offset = s.imm();
        let target = base as i32 + offset;
        data[target as usize >> 2] = registers.get(s.rs2()) & BYTE_MASK;
    }

    const fn sw(registers: &mut Registers, s: SInstruction, data: &mut [u32]) {
        let base = registers.get(s.rs1()) as u32;
        let offset = s.imm();
        let target = base as i32 + offset;
        data[target as usize >> 2] = registers.get(s.rs2()) & WORD_MASK;
    }

    const fn sh(registers: &mut Registers, s: SInstruction, data: &mut [u32]) {
        let base = registers.get(s.rs1()) as u32;
        let offset = s.imm();
        let target = base as i32 + offset;
        data[target as usize >> 2] = registers.get(s.rs2()) & HALF_WORD_MASK;
    }
}
