#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
enum Opcode {
    LType = 0x03,
    IType = 0x13,
    SType = 0x23,
    RType = 0x33,
    BType = 0x63,
    JALR = 0x67,
    JType = 0x6F,
    LUI = 0x37,
    AUIPC = 0x17,
}

pub mod rtype {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    #[repr(u32)]
    pub enum RFunct3 {
        AddSub = 0,
        SLL = 1,
        SLT = 2,
        SLTU = 3,
        XOR = 4,
        SrlSra = 5,
        OR = 6,
        AND = 7,
    }

    #[derive(Debug, Clone, Copy)]
    #[repr(u32)]
    pub enum Funct7 {
        BASE = 0,
        ALT = 0x20, // 用于区分 SUB 等指令
    }

    macro_rules! rtype_instructions {
        ($($name:ident: ($funct7:expr, $funct3:expr)),*) => {
            $(pub const fn $name(rd: u8, rs1: u8, rs2: u8)-> u32 {
                encode($funct7, rs2, rs1, $funct3, rd)
            })*
        }
    }

    /// 通用 R-type 指令编码器
    pub const fn encode(funct7: Funct7, rs2: u8, rs1: u8, funct3: RFunct3, rd: u8) -> u32 {
        (funct7 as u32) << 25
            | (rs2 as u32 & 0x1F) << 20
            | (rs1 as u32 & 0x1F) << 15
            | (funct3 as u32) << 12
            | (rd as u32 & 0x1F) << 7
            | Opcode::RType as u32
    }
    rtype_instructions! {
        add: (Funct7::BASE, RFunct3::AddSub),
        sub:  (Funct7::ALT,   RFunct3::AddSub),
        sll:  (Funct7::BASE,  RFunct3::SLL),
        slt:  (Funct7::BASE,  RFunct3::SLT),
        sltu: (Funct7::BASE,  RFunct3::SLTU),
        xor:  (Funct7::BASE,  RFunct3::XOR),
        srl:  (Funct7::BASE,  RFunct3::SrlSra),
        sra:  (Funct7::ALT,   RFunct3::SrlSra),
        or:   (Funct7::BASE,  RFunct3::OR),
        and:  (Funct7::BASE,  RFunct3::AND)
    }
}

pub mod itype {
    use crate::instruct_info::Opcode;
    #[derive(Debug, Clone, Copy)]
    #[repr(u32)]
    pub enum ILFunct3 {
        LB = 0,
        LH = 1,
        LW = 2,
        LBU = 4,
        LHU = 5,
    }

    #[derive(Debug, Clone, Copy)]
    #[repr(u32)]
    pub enum IRFunct3 {
        ADDI = 0,
        SLTI = 2,
        SLTIU = 3,
        XORI = 4,
        ORI = 6,
        ANDI = 7,
    }

    macro_rules! irtype_instructions {
        ($($name:ident: $funct3:expr),*) => {
            $(pub const fn $name(rd: u8, rs1: u8, imm: i16)-> u32 {
                encode_r(rd, rs1, $funct3, imm)
            })*
        };
    }

    const fn encode_r(rd: u8, rs1: u8, funct3: IRFunct3, imm: i16) -> u32 {
        (imm as u32 & 0xFFF) << 20
            | (rs1 as u32 & 0x1F) << 15
            | (funct3 as u32) << 12
            | (rd as u32 & 0x1F) << 7
            | Opcode::IType as u32
    }

    irtype_instructions! {
        addi: IRFunct3::ADDI,
        slti: IRFunct3::SLTI,
        sltiu: IRFunct3::SLTIU,
        xori: IRFunct3::XORI,
        ori: IRFunct3::ORI,
        andi: IRFunct3::ANDI
    }

    macro_rules! iltype_instruction {
        ($($name:ident: $funct3:expr),*) => {
            $(pub const fn $name(rd: u8, rs1: u8, imm: i16)-> u32 {
                encode_l(rd, rs1, $funct3, imm)
            })*
        };
    }
    const fn encode_l(rd: u8, rs1: u8, funct3: ILFunct3, imm: i16) -> u32 {
        ((imm as u32 & 0xFFF) << 20)
            | ((rs1 as u32 & 0x1F) << 15)
            | ((funct3 as u32) << 12)
            | ((rd as u32 & 0x1F) << 7)
            | Opcode::LType as u32
    }

    iltype_instruction! {
        lb: ILFunct3::LB,
        lbu: ILFunct3::LBU,
        lw: ILFunct3::LW,
        lh: ILFunct3::LH,
        lhu: ILFunct3::LHU
    }

    const fn encode_s(rd: u8, rs1: u8, funct3: u8, imm: u16) -> u32 {
        (imm as u32 & 0xFFF) << 20
            | (rs1 as u32 & 0x1F) << 15
            | (funct3 as u32) << 12
            | (rd as u32 & 0x1F) << 7
            | Opcode::IType as u32
    }

    pub const fn slli(rd: u8, rs1: u8, imm: u8) -> u32 {
        encode_s(rd, rs1, 1, (imm & 0b11111) as u16)
    }
    pub const fn srli(rd: u8, rs1: u8, imm: u8) -> u32 {
        encode_s(rd, rs1, 5, (imm & 0b11111) as u16)
    }
    pub const fn srai(rd: u8, rs1: u8, imm: u8) -> u32 {
        encode_s(rd, rs1, 5, (imm & 0b11111) as u16 | 0x400)
    }

    // I型指令辅助宏
    macro_rules! i_type {
        ($imm:expr, $rs1:expr, $funct3:expr, $rd:expr, $opcode:expr) => {
            (($imm & 0xfff) << 20) | ($rs1 << 15) | ($funct3 << 12) | ($rd << 7) | $opcode
        };
    }
    pub const fn jalr(rd: u8, rs1: u8, imm: i16) -> u32 {
        i_type!(imm as u16 as u32, rs1 as u32, 0, rd as u32, 0x67)
    }
}

pub mod stype {
    use crate::instruct_info::Opcode;

    #[derive(Debug, Clone, Copy)]
    #[repr(u32)]
    enum Funct3 {
        SB = 0,
        SH = 1,
        SW = 2,
    }
    const fn encode(rs1: u8, rs2: u8, funct3: Funct3, imm: i16) -> u32 {
        let imm11_5 = (imm >> 5) as u32 & 0x7F;
        let imm4_0 = (imm as u32) & 0x1F;

        imm11_5 << 25
            | ((rs2 as u32 & 0x1F) << 20)
            | ((rs1 as u32 & 0x1F) << 15)
            | ((funct3 as u32) << 12)
            | imm4_0 << 7
            | Opcode::SType as u32
    }

    pub const fn sb(rs2: u8, rs1: u8, imm: i16) -> u32 {
        encode(rs1, rs2, Funct3::SB, imm)
    }

    pub const fn sh(rs2: u8, rs1: u8, imm: i16) -> u32 {
        encode(rs1, rs2, Funct3::SH, imm)
    }

    pub const fn sw(rs2: u8, rs1: u8, imm: i16) -> u32 {
        encode(rs1, rs2, Funct3::SW, imm)
    }
}

pub mod btype {
    use crate::instruct_info::Opcode;

    #[derive(Debug, Clone, Copy)]
    #[repr(u32)]
    enum Funct3 {
        BEQ = 0,
        BNE = 1,
        BLT = 4,
        BGE = 5,
        BLTU = 6,
        BGEU = 7,
    }

    const fn encode(rs1: u8, rs2: u8, imm: i16, funct3: Funct3) -> u32 {
        let imm_raw = imm as u32;
        // 提取各分段（小端序）
        let imm12 = (imm_raw >> 12) & 0x1; // bit 12 → 指令位31
        let imm10_5 = (imm_raw >> 5) & 0x3F; // bits 10-5 → 指令位30-25
        let imm4_1 = (imm_raw >> 1) & 0xF; // bits 4-1 → 指令位11-8
        let imm11 = (imm_raw >> 11) & 0x1; // bit 11 → 指令位7

        (imm12 << 31)
            | (imm10_5 << 25)
            | ((rs2 as u32 & 0x1F) << 20)
            | ((rs1 as u32 & 0x1F) << 15)
            | (funct3 as u32) << 12
            | (imm4_1 << 8)
            | (imm11 << 7)
            | Opcode::BType as u32
    }

    macro_rules! btype_instructions {
        ($($name:ident: $funct3:expr,)*) => {
            $(pub const fn $name(rs1: u8, rs2: u8, imm: i16)-> u32 {
                encode(rs1, rs2, imm << 1, $funct3)
            })*
        };
    }

    btype_instructions! {
        beq: Funct3::BEQ,
        bne: Funct3::BNE,
        blt: Funct3::BLT,
        bge: Funct3::BGE,
        bltu: Funct3::BLTU,
        bgeu: Funct3::BGEU,
    }

    pub const fn ble(rs1: u8, rs2: u8, imm: i16) -> u32 {
        bge(rs2, rs1, imm)
    }
}

pub mod jtype {
    const fn encode_j_imm(imm: i32) -> u32 {
        let imm = imm as u32;
        ((imm & 0x80000) >> 20) << 19   // bit 20 -> 19
            | (imm & 0x7fe) << 20           // bits 10:1 -> 30:21
            | ((imm & 0x800) >> 11) << 20   // bit 11 -> 20
            | ((imm & 0xff000) >> 12) << 12 // bits 19:12 -> 19:12
    }

    pub const fn jal(rd: u8, imm: i32) -> u32 {
        encode_j_imm(imm << 1) | (rd as u32) << 7 | 0x6f
    }
}

// B型指令编码
const fn encode_b_imm(imm: i16) -> u32 {
    let imm = imm as u32;
    ((imm & 0x800) >> 11) << 31     // bit 12 -> 31
        | (imm & 0x7e0) << 20           // bits 10:5 -> 30:25
        | (imm & 0x1e) << 7             // bits 4:1 -> 11:8
        | ((imm & 0x1000) >> 12) << 7 // bit 11 -> 7
}

pub mod utype {
    // U型指令辅助宏
    macro_rules! u_type {
        ($imm:expr, $rd:expr, $opcode:expr) => {
            (($imm & 0xfffff) << 12) | ($rd << 7) | $opcode
        };
    }
    pub const fn lui(rd: u8, imm: i32) -> u32 {
        u_type!(imm as u32, rd as u32, 0x37)
    }

    pub const fn auipc(rd: u8, imm: i32) -> u32 {
        u_type!(imm as u32, rd as u32, 0x17)
    }
}

pub mod pseudo {
    use crate::instruct_info::jtype::jal;
    use crate::instruct_info::itype::{addi, jalr};
    use crate::instruct_info::utype::lui;
    use crate::register::alias::ra;

    pub const fn j(imm: i32) -> u32 {
        jal(0, imm)
    }

    pub const fn call(imm: i32) -> u32 {
        jal(1, imm) // ra = x1
    }
    pub const fn jr(re1: u8) -> u32 {
        jalr(0, re1, 0)
    }
    pub const fn ret() -> u32 {
        jr(ra)
    }
    pub const fn mv(rd: u8, rs: u8) -> u32 {
        addi(rd, rs, 0)
    }

    pub const fn nop() -> u32 {
        addi(0, 0, 0)
    }

    pub const fn stop() -> u32 {
        0
    }

    // TODO:
    pub const fn li(rd: u8, imm: i32) -> u32 {
        if -2048 <= imm && imm < 2048 {
            addi(rd, 0, imm as i16)
        } else if imm & 0xFFF == 0 {
            lui(rd, imm)
        } else {
            panic!("not implement")
        }
    }
}

#[allow(unused)]
pub mod prelude {
    pub use crate::instruct_info::itype::*;
    pub use crate::instruct_info::rtype::*;
    pub use crate::instruct_info::stype::*;
    pub use crate::instruct_info::utype::*;
    pub use crate::instruct_info::btype::*;
    pub use crate::instruct_info::jtype::*;
    pub use crate::instruct_info::pseudo::*;
}