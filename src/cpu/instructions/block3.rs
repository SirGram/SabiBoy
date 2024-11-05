use crate::cpu::flags::Flags;
use crate::cpu::registers::{Condition, Register16, Register8};
use crate::cpu::CPU;

impl CPU {
    fn arithmetic_op_imm8(&mut self, op: impl Fn(u8, u8) -> u8, set_n: bool, use_carry: bool) {
        let imm8 = self.fetch_byte();
        let original_a = self.get_r8(&Register8::A);
        let carry = if use_carry {
            self.f.contains(Flags::C) as u8
        } else {
            0
        };

        let result = if set_n {
            // Subtraction
            let temp = original_a.wrapping_sub(imm8);
            if use_carry {
                temp.wrapping_sub(carry)
            } else {
                temp
            }
        } else {
            // Addition
            let temp = original_a.wrapping_add(imm8);
            if use_carry {
                temp.wrapping_add(carry)
            } else {
                temp
            }
        };

        if set_n {
            self.set_sub_flags(original_a, imm8, carry);
        } else {
            self.set_add_flags(original_a, imm8, carry);
        }

        self.set_zn_flags(result, set_n);
        self.set_r8(&Register8::A, result);
    }

    pub fn add_a_imm8(&mut self) {
        self.arithmetic_op_imm8(
            |a, b| a.wrapping_add(b),
            false, // N flag
            false, // don't use carry
        );
    }

    pub fn adc_a_imm8(&mut self) {
        self.arithmetic_op_imm8(
            |a, b| a.wrapping_add(b),
            false, // N flag
            true,  // use carry
        );
    }

    pub fn sub_a_imm8(&mut self) {
        self.arithmetic_op_imm8(
            |a, b| a.wrapping_sub(b),
            true,  // N flag
            false, // don't use carry
        );
    }

    pub fn sbc_a_imm8(&mut self) {
        self.arithmetic_op_imm8(
            |a, b| a.wrapping_sub(b),
            true, // N flag
            true, // use carry
        );
    }

    fn logical_op_imm8(&mut self, op: impl Fn(u8, u8) -> u8, set_h: bool) {
        let imm8 = self.fetch_byte();
        let original_a = self.get_r8(&Register8::A);
        let result = op(original_a, imm8);

        self.set_zn_flags(result, false);
        self.f.set(Flags::H, set_h);
        self.f.remove(Flags::C);
        self.set_r8(&Register8::A, result);
    }

    pub fn and_a_imm8(&mut self) {
        self.logical_op_imm8(|a, b| a & b, true);
    }

    pub fn xor_a_imm8(&mut self) {
        self.logical_op_imm8(|a, b| a ^ b, false);
    }

    pub fn or_a_imm8(&mut self) {
        self.logical_op_imm8(|a, b| a | b, false);
    }

    pub fn cp_a_imm8(&mut self) {
        let imm8 = self.fetch_byte();
        let original_a = self.get_r8(&Register8::A);
        let result = original_a.wrapping_sub(imm8);

        self.set_sub_flags(original_a, imm8, 0);
        self.set_zn_flags(result, true);
    }

    // returns
    pub fn ret(&mut self) {
        // return from subroutine
        // pop pc from stack
        /* ld (pc), [sp] ;
        inc sp */
        let word = self.memory.read_word(self.sp);
        self.pc = word;
        self.sp = self.sp.wrapping_add(2);
    }
    pub fn ret_cc(&mut self, condition: Condition) {
        let should_jump = self.should_jump(condition);
        if should_jump {
            self.ret();
        }
    }
    pub fn reti(&mut self) {
        // TODO: Implement reti instruction
        // enable interrupts
        self.ret();
    }
}
