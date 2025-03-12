#![allow(dead_code)]

use std::ops::RangeInclusive;

pub const OPCODE: u32 = 0x7F;
pub const RD: u32 = 0xF80;
pub const FUNCT3: u32 = 0x7000;
pub const RS1: u32 = 0xF8000;
pub const RS2: u32 = 0x1F00000;
pub const FUNCT7: u32 = 0xFE000000;
pub const IMM4_0: u32 = RD;
pub const IMM7: u32 = FUNCT7;
pub const IMM11_5: u32 = FUNCT7;
pub const IMM11_0: u32 = FUNCT7 | RS2;
pub const IMM32_12: u32 = FUNCT7 | RS2 | RS1 | FUNCT3;

pub const BYTE_MASK: u32 = 0xFF;
pub const HALF_WORD_MASK: u32 = 0xFFFF;
pub const WORD_MASK: u32 = 0xFFFFFFFF;

pub const fn range_mask(start: u8, end: u8) -> u32 {
    !0_u32 >> (31 - end) & !0_u32 << start
}

const _: () = assert!(range_mask(0, 1) == 0b11);
const _: () = assert!(range_mask(0, 3) == 0b1111);
const _: () = assert!(range_mask(1, 3) == 0b1110);
const _: () = assert!(range_mask(5, 5) == 0b100000);
const _: () = assert!(range_mask(7, 7) == 0b10000000);


pub fn range(range: RangeInclusive<u8>) -> u32 {
    ((!0_u32) >> (32 - *range.end()) + *range.start()) << range.start()
}

#[cfg(test)]
mod mask_test {
    use crate::mask::range;

    #[test]
    fn test_mask() {
        assert_eq!(range(4..=7) & 0b00101111000, 0b000001110000);
    }
}
