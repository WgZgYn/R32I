use crate::arch::{R32I, RISC_V_32_REGISTERS};
pub const ZERO: Register = 0;
pub type Register = R32I;

// TODO: Add CsrRegisters

#[derive(Default, Debug)]
pub struct Registers {
    registers: [Register; RISC_V_32_REGISTERS],
    dirty: Register,
}

impl Registers {
    pub const fn default() -> Self {
        Self {
            registers: [0; RISC_V_32_REGISTERS],
            dirty: 0,
        }
    }
    pub const fn get(&self, i: u8) -> Register {
        assert!(i < RISC_V_32_REGISTERS as u8);
        self.registers[i as usize]
    }
    pub const fn get_mut(&mut self, i: u8) -> &mut Register {
        if i == 0 {
            return &mut self.dirty;
        }
        assert!(i < RISC_V_32_REGISTERS as u8);
        &mut self.registers[i as usize]
    }
    pub const fn write(&mut self, i: u8, v: u32) {
        assert!(i < RISC_V_32_REGISTERS as u8);
        if i == 0 {
            return;
        }
        self.registers[i as usize] = v;
    }
}

impl Registers {
    pub const fn zero(&self) -> Register {
        self.registers[0]
    }

    pub const fn ra(&mut self) -> &mut Register {
        &mut self.registers[1]
    }

    pub const fn sp(&mut self) -> &mut Register {
        &mut self.registers[2]
    }

    pub const fn gp(&mut self) -> &mut Register {
        &mut self.registers[3]
    }

    pub const fn tp(&mut self) -> &mut Register {
        &mut self.registers[4]
    }

    pub const fn t(&mut self, n: usize) -> &mut Register {
        if n <= 2 {
            &mut self.registers[n + 5]
        } else {
            &mut self.registers[n + 25]
        }
    }

    pub const fn fp(&mut self) -> &mut Register {
        &mut self.registers[8]
    }

    pub const fn s(&mut self, n: usize) -> &mut Register {
        if n <= 2 {
            &mut self.registers[n + 8]
        } else {
            &mut self.registers[n + 16]
        }
    }

    pub const fn a(&mut self, n: usize) -> &mut Register {
        &mut self.registers[n + 10]
    }
}

pub mod alias {
    use register_aliases::register_aliases;
    register_aliases! {}

    macro_rules! register_alias {
        ($($name:ident: $index:expr),*) => {
            $(
                #[allow(non_upper_case_globals)]
                pub const $name: u8 = $index;
            )*
        };
    }

    register_alias! {
        zero: 0,
        ra: 1,
        sp: 2,
        gp: 3,
        tp: 4,
        t0: 5,
        t1: 6,
        t2: 7,
        fp: 8,
        s0: 8,
        s1: 9,
        a0: 10,
        a1: 11,
        a2: 12,
        a3: 13,
        a4: 14,
        a5: 15,
        a6: 16,
        a7: 17,
        s2: 18,
        s3: 19,
        s4: 20,
        s5: 21,
        s6: 22,
        s7: 23,
        s8: 24,
        s9: 25,
        s10: 26,
        s11: 27,
        t3: 28,
        t4: 29,
        t5: 30,
        t6: 31
    }

    pub const fn t(n: u8) -> u8 {
        if n <= 2 { n + 5 } else { n + 25 }
    }

    pub const fn s(n: u8) -> u8 {
        if n <= 2 { n + 8 } else { n + 16 }
    }

    pub const fn a(n: u8) -> u8 {
        n + 10
    }
}
