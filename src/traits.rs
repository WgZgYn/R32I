trait Code {
    const CODE: u32;
}

pub trait GetCode {
    fn code(&self) -> u32;
}

impl<T: RType> Code for T {
    const CODE: u32 = T::OPCODE | T::FUNCT7;
}

impl<T: RType> GetCode for T {
    fn code(&self) -> u32 {
        T::CODE
    }
}

pub trait RType: Sized {
    const OPCODE: u32 = 0x33;
    const FUNCT3: u32;
    const FUNCT7: u32;
    const RD: u8;
    const RS1: u8;
    const RS2: u8;
}

pub struct ADD<const RD: u8, const RS1: u8, const RS2: u8>;

impl<const RD: u8, const RS1: u8, const RS2: u8> RType for ADD<{ RD }, { RS1 }, { RS2 }> {
    const FUNCT3: u32 = 0;
    const FUNCT7: u32 = 0;
    const RD: u8 = RD;
    const RS1: u8 = RS1;
    const RS2: u8 = RS2;
}

pub struct SUB<const RD: u8, const RS1: u8, const RS2: u8>;

impl<const RD: u8, const RS1: u8, const RS2: u8> RType for SUB<{ RD }, { RS1 }, { RS2 }> {
    const FUNCT3: u32 = 0;
    const FUNCT7: u32 = 0x20;
    const RD: u8 = RD;
    const RS1: u8 = RS1;
    const RS2: u8 = RS2;
}