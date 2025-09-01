#![feature(io_const_error)]
#![allow(dead_code)]
mod alu;
mod arch;
mod const_emulator;
mod emulator;
mod exception;
mod instruct_info;
mod instruction_type;
mod mask;
mod memory;
mod opcode;
mod register;
mod test_code;
mod traits;

use crate::const_emulator::ConstantEmulator;
use crate::emulator::EmulatorContext;
use r32i_asm::riscv_asm;

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
    context.set_data_segment(&nums).set_code_segment(QUICK_SORT);
    context.run_with_thread();
    #[allow(long_running_const_eval)]
    const RES: u32 = ConstantEmulator::run_loop(riscv_asm! {
        main:
        li a0, 0;
        li a1, 1000;
        slli a1, a1, 4;
        L1:
        beq a1, zero, end;
        add a0, a0, a1;
        addi a1, a1, -1;
        j L1;
        end:
    });

    let start = std::time::Instant::now();
    const F: u32 = ConstantEmulator::run_loop(riscv_asm! {
        Main:
        li a0, 12;
        mv s2, a0;
        li a0, 1;
        Loop:
        beq s2, zero, End;
        mv a1, s2;
        call mul;
        addi s2, s2, -1;
        j Loop;
        End:
        stop;
        mul:
        mv t0, a0;
        mv t1, a1;
        li a0, 0;
        mul_loop:
        beq t1, zero, mul_end;
        add a0, a0, t0;
        addi t1, t1, -1;
        j mul_loop;
        mul_end:
        ret;
    });


    println!("const F: {}, time: {:?}", F, start.elapsed());
    #[allow(long_running_const_eval)]
    const A: u32 = ConstantEmulator::run_loop(riscv_asm!{
        main:
        li a0, 2;
        li a1, 1000;
        addi sp, sp, -16;
        sw a1, 4(sp);
        lw a0, 4(sp);
        stop
    });
    println!("{}", A);
    println!("const RES sum: {}", RES);
}
