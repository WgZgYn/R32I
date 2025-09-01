use crate::arch::{Address, PC_DEFAULT_ADDRESS};
use crate::instruction_type::{IInstruction, SInstruction};
use crate::mask::{BYTE_MASK, HALF_WORD_MASK, WORD_MASK};
use crate::register::Registers;

const CODE_DEFAULT_OFFSET: usize = PC_DEFAULT_ADDRESS as usize;
const STACK_BOTTOM_DEFAULT_OFFSET: Address = 0xF0000;
const STACK_DEFAULT_SIZE: usize = 0x100;

pub trait RandomAccess {
    type Output;
    type KeyType;
    fn read(&self, key: &Self::KeyType) -> Option<&Self::Output>;
    fn write(&mut self, key: &Self::KeyType, value: &Self::Output);
    fn get_mut(&mut self, key: &Self::KeyType) -> Option<&mut Self::Output>;
}

#[derive(Default)]
struct MemorySegments {
    data: Vec<u32>,
}

trait Memory {
    fn read(&self, address: Address) -> u32;
    fn write(&mut self, address: Address, value: u32);
}

impl RandomAccess for MemorySegments {
    type Output = u32;
    type KeyType = Address;

    fn read(&self, key: &Self::KeyType) -> Option<&Self::Output> {
        let word_address = *key >> 2;
        match self.data.get(word_address as usize) {
            None => {
                // println!("read to not write address {}", word_address);
                None
            }
            s => s,
        }
    }
    fn write(&mut self, key: &Self::KeyType, value: &Self::Output) {
        let word_address = *key >> 2;
        if word_address as usize >= self.data.len() {
            // println!("write to new word_address {}, enlarge", word_address);
            self.data.resize(word_address as usize + 1, 0);
        }
        self.data[word_address as usize] = *value;
    }
    fn get_mut(&mut self, byte_address: &Self::KeyType) -> Option<&mut Self::Output> {
        let word_address = *byte_address >> 2;
        if word_address as usize >= self.data.len() {
            // println!("get_mut to new word_address {}, enlarge", word_address);
            self.data.resize(word_address as usize + 1, 0);
        }
        self.data.get_mut(word_address as usize)
    }
}

#[derive(Default)]
pub struct MemoryWrapper {
    segments: MemorySegments,
}


impl MemoryWrapper {
    pub fn test_get_memory(&mut self) -> &mut Vec<u32> {
        &mut self.segments.data
    }
    pub fn append(&mut self, data: &[u32]) {
        self.segments.data.extend_from_slice(data);
    }
    pub fn read_byte(&self, byte_address: &Address) -> u8 {
        self.segments
            .read(byte_address)
            .copied()
            .map(|v| {
                ((v >> match byte_address & 0b11 {
                    0b00 => 0,
                    0b01 => 8,
                    0b10 => 16,
                    0b11 => 24,
                    _ => unreachable!(),
                }) & BYTE_MASK) as u8
            })
            .unwrap_or(0)
    }
    pub fn read_word(&self, byte_address: &Address) -> u32 {
        self.segments.read(byte_address).copied().unwrap_or(0)
    }
    pub fn read_halfword(&self, byte_address: &Address) -> u16 {
        self.segments
            .read(byte_address)
            .copied()
            .map(|v| {
                ((v >> match byte_address & 1 {
                    0 => 0,
                    1 => 16,
                    _ => unreachable!(),
                }) & HALF_WORD_MASK) as u16
            })
            .unwrap_or(0)
    }
    pub fn write_word(&mut self, byte_address: &Address, value: u32) {
        self.segments.write(byte_address, &value);
    }
    pub fn write_halfword(&mut self, byte_address: &Address, halfword: u16) {
        self.segments.get_mut(byte_address).map(|v| {
            *v = *v | {
                if byte_address & 1 == 0 {
                    halfword as u32
                } else {
                    (halfword as u32) << 16
                }
            }
        });
    }
    pub fn write_byte(&mut self, byte_address: &Address, value: u8) {
        self.segments.get_mut(byte_address).map(|v| {
            *v = *v | {
                match byte_address & 0b11 {
                    0b00 => value as u32,
                    0b01 => (value as u32) << 8,
                    0b10 => (value as u32) << 16,
                    0b11 => (value as u32) << 24,
                    _ => unreachable!(),
                }
            }
        });
    }

    fn lb(&self, registers: &mut Registers, i: IInstruction) {
        let base = registers.get(i.rs1()) as u32;
        let offset = i.imm();
        let data = self.read_byte(&((base as i32 + offset) as u32));
        *registers.get_mut(i.rd()) = (data as u32) << 24 >> 24;
    }

    fn lbu(&self, registers: &mut Registers, i: IInstruction) {
        let base = registers.get(i.rs1()) as u32;
        let offset = i.imm();
        let data = self.read_byte(&((base as i32 + offset) as u32));
        *registers.get_mut(i.rd()) = data as u32;
    }

    fn lh(&self, registers: &mut Registers, i: IInstruction) {
        let base = registers.get(i.rs1()) as u32;
        let offset = i.imm();
        let data = self.read_halfword(&((base as i32 + offset) as u32));
        *registers.get_mut(i.rd()) = (data as u32) << 16 >> 16;
    }

    fn lhu(&self, registers: &mut Registers, i: IInstruction) {
        let base = registers.get(i.rs1()) as u32;
        let offset = i.imm();
        let data = self.read_halfword(&((base as i32 + offset) as u32));
        *registers.get_mut(i.rd()) = data as u32;
    }

    fn lw(&self, registers: &mut Registers, i: IInstruction) {
        let base = registers.get(i.rs1()) as u32;
        let offset = i.imm();
        *registers.get_mut(i.rd()) = self.read_word(&((base as i32 + offset) as u32));
    }

    fn sb(&mut self, registers: &mut Registers, s: SInstruction) {
        let base = registers.get(s.rs1()) as u32;
        let offset = s.imm();
        let target = base as i32 + offset;
        assert!(target >= 0);
        self.write_byte(&(target as u32), (registers.get(s.rs2()) & BYTE_MASK) as u8);
    }

    fn sw(&mut self, registers: &mut Registers, s: SInstruction) {
        let base = registers.get(s.rs1()) as u32;
        let offset = s.imm();
        let target = base as i32 + offset;
        assert!(target >= 0);

        // println!("store {} to {}", registers.get(s.rs2()) & WORD_MASK, target);
        self.write_word(&(target as u32), registers.get(s.rs2()) & WORD_MASK);
    }

    fn sh(&mut self, registers: &mut Registers, s: SInstruction) {
        let base = registers.get(s.rs1()) as u32;
        let offset = s.imm();
        let target = base as i32 + offset;
        assert!(target >= 0);
        self.write_halfword(
            &(target as u32),
            (registers.get(s.rs2()) & HALF_WORD_MASK) as u16,
        );
    }

    pub fn load(&self, registers: &mut Registers, i: IInstruction) {
        match i.funct3() {
            0b000 => self.lb(registers, i),
            0b001 => self.lh(registers, i),
            0b010 => self.lw(registers, i),
            0b100 => self.lbu(registers, i),
            0b101 => self.lhu(registers, i),
            _ => {
                eprintln!(
                    "no such instruction {:0X} with funct3 {}",
                    i.0.0,
                    i.funct3()
                );
            }
        }
    }

    pub fn store(&mut self, registers: &mut Registers, s: SInstruction) {
        match s.funct3() {
            0b000 => self.sb(registers, s),
            0b001 => self.sh(registers, s),
            0b010 => self.sw(registers, s),
            _ => {
                eprintln!(
                    "no such instruction {:0X} with funct3 {}",
                    s.0.0,
                    s.funct3()
                );
            }
        }
    }
}
