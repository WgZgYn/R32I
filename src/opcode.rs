#![allow(dead_code)]

use crate::Byte;

// I 类型指令
pub const JAL: Byte = 0x6F; // Jump and Link
pub const JALR: Byte = 0x67; // Jump and Link Register
// U 类型指令
pub const LUI: Byte = 0x37; // Load Upper Immediate
pub const AUIPC: Byte = 0x17; // Add Upper Immediate to PC
// 其他指令
pub const NOP: Byte = 0x00; // No Operation

pub const I_TYPE: Byte = 0x03;
pub const RI_TYPE:Byte = 0x13;
pub const S_TYPE: Byte = 0x23;
pub const R_TYPE: Byte = 0x33;
pub const B_TYPE: Byte = 0x63;
pub const J_TYPE: Byte = 0x6F;

mod b {
    use crate::Byte;

    pub const BEQ: Byte = 0x63; // Branch Equal
    pub const BNE: Byte = 0x63; // Branch Not Equal
    pub const BLT: Byte = 0x63; // Branch Less Than
    pub const BGE: Byte = 0x63; // Branch Greater Than or Equal
    pub const BLTU: Byte = 0x63; // Branch Less Than Unsigned
    pub const BGEU: Byte = 0x63; // Branch Greater Than or Equal Unsigned
}

mod l {
    use crate::Byte;

    pub const LB: Byte = 0x03; // Load Byte
    pub const LH: Byte = 0x03; // Load Halfword
    pub const LW: Byte = 0x03; // Load Word
    pub const LBU: Byte = 0x03; // Load Byte Unsigned
    pub const LHU: Byte = 0x03; // Load Halfword Unsigned
}

mod r {
    use crate::Byte;

    pub const ADD: Byte = 0x33; // Add
    pub const SUB: Byte = 0x33; // Subtract
    pub const AND: Byte = 0x33; // And
    pub const OR: Byte = 0x33; // Or
    pub const XOR: Byte = 0x33; // Exclusive OR
    pub const SLL: Byte = 0x33; // Shift Left Logical
    pub const SRL: Byte = 0x33; // Shift Right Logical
    pub const SRA: Byte = 0x33; // Shift Right Arithmetic
}

mod s {
    use crate::Byte;

    pub const SB: Byte = 0x23; // Store Byte
    pub const SH: Byte = 0x23; // Store Halfword
    pub const SW: Byte = 0x23; // Store Word
}
