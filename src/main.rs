#![allow(dead_code)]
mod alu;
mod arch;
mod instruct_info;
mod instruction_type;
mod mask;
mod memory;
mod opcode;
mod register;
mod traits;

use crate::alu::ALU;
use crate::arch::{Address, Byte, PC_DEFAULT_ADDRESS, PC_STEP};
use crate::instruction_type::*;
use crate::memory::Memory;
use crate::register::Registers;
use std::arch::asm;
// use crate::traits::{ADD, GetCode, SUB};
use opcode::*;

struct EmulatorContext {
    registers: Registers,
    memory: Memory,
    program_counter: Address,
    stop: bool,
}

impl Default for EmulatorContext {
    fn default() -> Self {
        Self {
            registers: Registers::default(),
            memory: Memory::default(),
            program_counter: PC_DEFAULT_ADDRESS,
            stop: false,
        }
    }
}

impl EmulatorContext {
    pub fn set_code_segment(&mut self, memory: &[u32]) -> &mut Self {
        for (index, value) in memory.iter().enumerate() {
            self.memory.write(&((index * 4) as Address), *value);
        }
        self
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction.opcode() as Byte {
            I_TYPE => self.memory.load(&mut self.registers, instruction.as_i()),
            RI_TYPE => ALU::with(&mut self.registers).immediate(instruction.as_i()),
            R_TYPE => ALU::with(&mut self.registers).execute(instruction.as_r()),
            S_TYPE => self.memory.store(&mut self.registers, instruction.as_s()),
            J_TYPE => {
                let j = instruction.as_j();
                *self.registers.get_mut(j.rd()) = self.program_counter;
                self.program_counter = self.program_counter.overflowing_add(j.imm()).0;
            }
            JALR => {
                let i = instruction.as_i();
                *self.registers.get_mut(i.rd()) = self.program_counter;
                self.program_counter = (self.program_counter as i32
                    + ((self.registers.get(i.rs1())) & !1) as i32
                    + i.imm()) as Address;
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
            let i: Instruction = Instruction::from(self.memory.access(&self.program_counter));
            self.program_counter += PC_STEP;
            self.execute(&i);
        }
    }
}

fn main() {
    use instruct_info::btype::*;
    use instruct_info::itype::*;
    use instruct_info::rtype::*;
    use instruct_info::*;
    use register::alias::*;

    let mut context = EmulatorContext::default();
    const LOOP_ADD: &[u32] = &[
        // init:
        li(t0, 1),   // 初始化增量
        li(t1, 101), // 设置结束条件
        li(t2, 0),   // 初始化sum
        // start:
        add(t2, t2, t0),
        addi(t0, t0, 1),
        bne(t1, t0, -4),
        // stop
        mv(a0, t2),
        nop(),
    ];

    // context.set_code_segment(LOOP_ADD).run();
    // for v in fibonacci::<40>() {
    //     println!("{:032b}", v as u32);
    // }
    // let mut f = std::fs::File::create("./fibonacci.rbin").expect("should create fibonacci.rbin");
    // f.write(fibonacci::<40>().into_iter().map(|v| v.to_le_bytes()).flatten().collect::<Vec<u8>>().as_slice()).expect("should write fibonacci.rbin");

    // let binary = std::fs::read("./fibonacci.rbin").expect("File not found");
    // let instructions: Vec<u32> = binary.chunks(4).map(|c| {
    //     let mut bs = [0; 4];
    //     bs.copy_from_slice(&c[0..4]);
    //     u32::from_le_bytes(bs)
    // }).collect();

    let start = std::time::Instant::now();
    let res = fibonacci_native(1000);
    println!("{res}, {}", start.elapsed().as_nanos());
    // TODO: FIX imm sign extend

    let start = std::time::Instant::now();
    context
        .set_code_segment(&fibonacci_greater_2048::<1000>())
        .run();

    println!(
        "{}, {}",
        context.registers.get(a0),
        start.elapsed().as_nanos()
    );

    let start = std::time::Instant::now();
    unsafe {
        let mut a: u64 = 0;
        asm!(
        "mov {0}, 1000",
        "mov {1}, 0",
        "mov {2}, 1",
        "2:",
        "cmp {0}, 0",
        "je 3f",
        "mov {3}, {1}",
        "add {3}, {2}",
        "mov {1}, {2}",
        "mov {2}, {3}",
        "dec {0}",
        "jmp 2b",
        "3:",
        out(reg) _,
        inout(reg) a,
        out(reg) _,
        out(reg) _,
        );
        println!("asm: {}, {}", a, start.elapsed().as_nanos());
    }
}

const fn fibonacci_compile<const N: usize>() -> u32 {
    let mut cnt = N;
    let mut a = 0;
    let mut b = 1;
    while cnt > 0 {
        let c = a + b;
        a = b;
        b = c;
        cnt -= 1;
    }
    a
}

fn fibonacci_native(n: usize) -> u32 {
    let mut cnt = n;
    let mut a = 0;
    let mut b = 1;
    while cnt > 0 {
        let c = a + b;
        a = b;
        b = c;
        cnt -= 1;
    }
    a
}

const fn fibonacci_less_2048<const N: usize>() -> [u32; 11] {
    use instruct_info::btype::*;
    use instruct_info::itype::*;
    use instruct_info::rtype::*;
    use instruct_info::*;
    use register::alias::*;

    const CNT: u8 = t0;
    const A: u8 = t1;
    const B: u8 = t2;
    const C: u8 = t3;

    [
        li(CNT, N as i32), // int cnt = N; ,
        li(A, 0),          // int a = 0;
        li(B, 1),          // int b = 1;
        // judge
        beq(CNT, 0, 12), // if (cnt == 0) { goto End; }
        // iter
        add(C, A, B),       // c = a + b;
        mv(A, B),           // a = b;
        mv(B, C),           // b = c;
        addi(CNT, CNT, -1), // cnt = cnt - 1;
        j(-12),             // goto Judge
        // end
        mv(a(0), A), // return a
        nop(),
    ]
}

const fn fibonacci_greater_2048<const N: usize>() -> [u32; 12] {
    use instruct_info::btype::*;
    use instruct_info::itype::*;
    use instruct_info::rtype::*;
    use instruct_info::*;
    use register::alias::*;

    const CNT: u8 = t0;
    const A: u8 = t1;
    const B: u8 = t2;
    const C: u8 = t3;

    [
        addi(CNT, CNT, (N & 0xFFF) as i16), // int cnt = N; ,
        lui(CNT, (N >> 12) as i32),         // int cnt = N; ,
        li(A, 0),                           // int a = 0;
        li(B, 1),                           // int b = 1;
        // judge
        beq(CNT, 0, 12), // if (cnt == 0) { goto End; }
        // iter
        add(C, A, B),       // c = a + b;
        mv(A, B),           // a = b;
        mv(B, C),           // b = c;
        addi(CNT, CNT, -1), // cnt = cnt - 1;
        j(-12),             // goto Judge
        // end
        mv(a(0), A), // return a
        nop(),
    ]
}
