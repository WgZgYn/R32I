use crate::alu::ALU;
use crate::arch::{Address, Byte, PC_DEFAULT_ADDRESS, PC_STEP, STACK_DEFAULT_ADDRESS};
use crate::instruction_type::Instruction;
use crate::memory::MemoryWrapper;
use crate::register::Registers;

pub struct EmulatorContext {
    pub(crate) registers: Registers,
    memory: MemoryWrapper,
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
            memory: MemoryWrapper::default(),
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
        use crate::opcode::*;
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

    pub fn run_with_thread(mut self) {
        std::thread::spawn(move || {
            self.run();
            println!(
                "the sorted nums: {:?}",
                &self.memory.test_get_memory()[..10]
            );
        })
        .join()
        .unwrap();
    }
}
