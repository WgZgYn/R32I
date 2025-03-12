#[cfg(test)]
mod test_code {
    use crate::EmulatorContext;
    use crate::register::alias::{a0, a1, t0, t1, t2};
    use proc_macro::riscv_asm;

    #[test]
    fn test_j() {
        let mut c = EmulatorContext::default();
        let code = riscv_asm! {
        _start:
            li a0, 0;
            li a1, 0;
            j L_target;
            addi a0, a0, 1;
        L_target:
            addi a1, a1, 2;
            stop;
        };
        c.set_code_segment(code).run();
        println!("a0: {}, a1: {}", c.registers.get(a0), c.registers.get(a1));
        assert!(c.registers.a(0) == &0 && c.registers.a(1) == &2)
    }

    #[test]
    fn test_jr() {
        let mut c = EmulatorContext::default();
        let code = riscv_asm! {
        _start:
            addi ra, ra, L_return;
            jr ra;
            li a0, 1;
        L_return:
            li a0, 42;
            stop;
        };
        c.set_code_segment(code).run();
        println!("a0: {}", c.registers.get(a0));
        assert_eq!(c.registers.a(0), &42)
    }

    #[test]
    fn test_sw_lw() {
        let mut c = EmulatorContext::default();

        let code = riscv_asm! {
        _start:
            addi t0, t0, 24;      // t0 = value地址
            lw t1, 0(t0);      // t1 = 0x12345678
            addi t1, t1, 1;    // t1 += 1 → 0x12345679
            sw t1, 0(t0);      // 写回内存
            lw t2, 0(t0);      // t2 = 0x12345679
            stop;
        };

        c.set_code_segment(code).run();
        println!(
            "t0: {}, t1: {}, t2: {}",
            c.registers.get(t0),
            c.registers.get(t1),
            c.registers.get(t2)
        );
        assert_eq!(c.registers.get(t1), c.registers.get(t2));
    }

    #[test]
    fn test_slli() {
        let mut c = EmulatorContext::default();
        let code = riscv_asm! {
        _start:
            li a0, 1;          // a0 = 1 (0b0001)
            slli a1, a0, 2;    // a1 = a0 << 2 → 0b0100 (4)
            // 预期结果：a1=4
            stop;
        };
        c.set_code_segment(code).run();
        assert_eq!(c.registers.a(1), &4)
    }

    #[test]
    fn test_blt() {
        let mut c = EmulatorContext::default();
        let code = riscv_asm! {
        _start:
            li a0, 5;
            li a1, 10;
            blt a0, a1, L_less;
            li a2, 0;
            j L_end;
        L_less:
            li a2, 1;
        L_end:
            stop;
        };
        c.set_code_segment(code).run();
        assert_eq!(c.registers.a(2), &1)
    }

    #[test]
    fn test_beq() {
        let mut c = EmulatorContext::default();
        let code = riscv_asm! {
        _start:
            li a0, 7;
            li a1, 7;
            beq a0, a1, L_equal;
            li a2, 0;
            j L_end;
        L_equal:
            li a2, 1;
        L_end:
        };
        c.set_code_segment(code).run();
        assert_eq!(c.registers.a(2), &1)
    }

    #[test]
    fn test_call() {
        let mut c = EmulatorContext::default();
        let code = riscv_asm! {
        _start:
            call my_func;       // 调用函数
            j L_end;

        my_func:
            li a0, 1;        // a0 = 100
            ret;               // 返回到call的下一条指令

        L_end:
            addi a0, a0, 1;
            // 预期结果：a0=2
            stop;
        };
        c.set_code_segment(code).run();
        assert_eq!(c.registers.a(0), &2)
    }
}
