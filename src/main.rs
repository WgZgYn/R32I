#![allow(dead_code)]
mod alu;
mod arch;
mod const_emulator;
mod instruct_info;
mod instruction_type;
mod mask;
mod memory;
mod opcode;
mod register;
mod test_code;
mod traits;

use crate::alu::ALU;
use crate::arch::{Address, Byte, PC_DEFAULT_ADDRESS, PC_STEP, STACK_DEFAULT_ADDRESS};
use crate::instruction_type::*;
use crate::memory::Memory;
use crate::register::Registers;
use proc_macro::riscv_asm;

struct EmulatorContext {
    registers: Registers,
    memory: Memory,
    program_counter: Address,
    max_address: Address,
    data_offset: Address,
    stack_offset: Address,
    stop: bool,
}

impl Default for EmulatorContext {
    fn default() -> Self {
        let mut regs = Registers::default();
        *regs.sp() = STACK_DEFAULT_ADDRESS;
        Self {
            registers: regs,
            memory: Memory::default(),
            program_counter: PC_DEFAULT_ADDRESS,
            max_address: 0,
            data_offset: 0,
            stack_offset: STACK_DEFAULT_ADDRESS,
            stop: false,
        }
    }
}

impl EmulatorContext {
    // set the pc with it, it will consider the last code segment as main
    pub fn set_code_segment(&mut self, data: &[u32]) -> &mut Self {
        self.program_counter = self.max_address;
        self.max_address += (data.len() as Address) << 2;
        self.memory.append(data);
        self
    }

    // set the data offset
    pub fn set_data_segment(&mut self, data: &[u32]) -> &mut Self {
        self.data_offset = self.max_address;
        self.max_address += (data.len() as Address) << 2;
        self.memory.append(data);
        self
    }

    pub fn set_stack_offset(&mut self, offset: Address) -> &mut Self {
        self.stack_offset = offset;
        *self.registers.sp() = offset;
        self
    }

    fn execute(&mut self, instruction: &Instruction) {
        use opcode::*;
        match instruction.opcode() as Byte {
            I_TYPE => self.memory.load(&mut self.registers, instruction.as_i()),
            RI_TYPE => ALU::with(&mut self.registers).immediate(instruction.as_i()),
            R_TYPE => ALU::with(&mut self.registers).execute(instruction.as_r()),
            S_TYPE => self.memory.store(&mut self.registers, instruction.as_s()),
            J_TYPE => {
                let j = instruction.as_j();
                *self.registers.get_mut(j.rd()) = self.program_counter;
                self.program_counter = ((self.program_counter - PC_STEP) as i32 + j.imm()) as u32;
            }
            JALR => {
                let i = instruction.as_i();
                *self.registers.get_mut(i.rd()) = self.program_counter;
                self.program_counter =
                    (((self.registers.get(i.rs1())) & !1) as i32 + i.imm()) as Address;
            }
            B_TYPE => {
                ALU::with(&mut self.registers).branch(&mut self.program_counter, instruction.as_b())
            }
            LUI => {
                let u = instruction.as_u();
                *self.registers.get_mut(u.rd()) =
                    (self.registers.get(u.rd()) & 0xFFF) + u.high_imm();
            }
            AUIPC => {
                let u = instruction.as_u();
                *self.registers.get_mut(u.rd()) = self.program_counter + u.high_imm();
            }
            NOP => self.stop = true,
            _ => panic!("SegmentFault"),
        }
    }

    pub fn run(&mut self) {
        while !self.stop {
            let i: Instruction = Instruction::from(self.memory.read_word(&self.program_counter));
            self.program_counter += PC_STEP;
            // println!("{}", i);
            self.execute(&i);
        }
    }
}

fn main() {
    let mut context = EmulatorContext::default();
    const QUICK_SORT: &[u32; 85] = riscv_asm! {
    main:
            li a0, 0;
            li a1, 0;
            li a2, 9;
            call quick_sort;
            stop;

    quick_sort:
            addi    sp,sp,-16;
            slli    a5,a1,2;
            sw      s1,4(sp);
            sw      s2,0(sp);
            add     a5,a0,a5;
            sw      ra,12(sp);
            sw      s0,8(sp);
            lw      a7,0(a5);
            mv      s1,a0;
            mv      s2,a2;
            bge     a1,a2,L16;
            slli    a5,a2,2;
            add     a5,a0,a5;
            lw      a4,0(a5);
            mv      a3,a2;
            mv      a2,a1;
    L3:
            addi    a6,a3,-1;
            slli    a6,a6,2;
            add     a6,s1,a6;
            j       L11;
    L5:
            addi    a3,a3,-1;
            lw      a4,4(a5);
            beq     a3,a2,L4;
            mv      a6,a5;
    L11:
            addi    a5,a6,-4;
            ble     a7,a4,L5;
            slli    a5,a2,2;
            add     a5,s1,a5;
            slli    a6,a3,2;
            sw      a4,0(a5);
            add     a6,s1,a6;
            ble     a3,a2,L6;
            mv      s0,a2;
            j       L7;
    L9:
            addi    a5,a5,4;
            beq     a3,s0,L21;
    L7:
            lw      a4,0(a5);
            mv      a2,s0;
            addi    s0,s0,1;
            ble     a4,a7,L9;
            sw      a4,0(a6);
            blt     a2,a3,L3;
            mv      s0,a2;
            addi    a2,a2,-1;
            j       L15;
    L21:
            slli    a5,s0,2;
            add     a5,s1,a5;
            lw      a4,0(a5);
    L8:
            sw      a4,0(a6);
    L15:
            sw      a7,0(a5);
            bge     a1,a2,L2;
            mv      a0,s1;
            call    quick_sort;
    L2:
            addi    a1,s0,1;
            blt     a1,s2,L22;
            lw      ra,12(sp);
            lw      s0,8(sp);
            lw      s1,4(sp);
            lw      s2,0(sp);
            li      a0,0;
            addi    sp,sp,16;
            ret;
    L22:
            mv      a2,s2;
            mv      a0,s1;
            call    quick_sort;
            lw      ra,12(sp);
            lw      s0,8(sp);
            lw      s1,4(sp);
            lw      s2,0(sp);
            li      a0,0;
            addi    sp,sp,16;
            jr      ra;
    L16:
            mv      s0,a1;
            j       L2;
    L4:
            slli    a5,a2,2;
            add     a5,s1,a5;
            sw      a4,0(a5);
    L6:
            mv      s0,a2;
            addi    a2,a2,-1;
            j       L8;
        };
    let nums = [18, 46, 62, 59, 78, 71, 7, 99, 18, 28];
    context
        .set_data_segment(&nums)
        .set_code_segment(QUICK_SORT)
        .run();

    println!(
        "the sorted nums: {:?}",
        &context.memory.test_get_memory()[..nums.len()]
    );

    // #[allow(long_running_const_eval)]
    // const RES: u32 = ConstantEmulator::run_loop(QUICK_SORT);
    // println!("const RES sum: {}", RES);

    let data =  riscv_asm!(
    x:  .byte 0x12;
    y:  .byte 0x13;
    z:  .byte 0x14;
    w:  .byte 0x15;
        .word 0x12345678;
    );

    println!("{:#x?}", data);
}
