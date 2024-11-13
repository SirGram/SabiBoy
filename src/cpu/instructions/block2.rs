use crate::cpu::flags::Flags;
use crate::cpu::registers::Register8;
use crate::cpu::CPU;

impl CPU {
    fn arithmetic_op_r8(
        &mut self,
        register: Register8,
        is_subtract: bool,
        use_carry: bool,
        update_register: bool,
    ) {
        let value = self.get_r8(&register);
        let original_a = self.get_r8(&Register8::A);
        let carry = if use_carry {
            self.f.contains(Flags::C) as u8
        } else {
            0
        };

        let result = match (is_subtract, use_carry) {
            (false, false) => original_a.wrapping_add(value), // ADD
            (false, true) => original_a.wrapping_add(value).wrapping_add(carry), // ADC
            (true, false) => original_a.wrapping_sub(value),  // SUB
            (true, true) => original_a.wrapping_sub(value).wrapping_sub(carry), // SBC
        };

        if is_subtract {
            self.set_sub_flags(original_a, value, carry);
        } else {
            self.set_add_flags(original_a, value, carry);
        }

        self.set_zn_flags(result, is_subtract);

        if update_register {
            self.set_r8(&Register8::A, result);
        }
    }

    // Generic function for logical operations
    fn logical_op_r8(&mut self, register: Register8, op: impl Fn(u8, u8) -> u8, set_h: bool) {
        let original_a = self.get_r8(&Register8::A);
        let value = self.get_r8(&register);
        let result = op(original_a, value);

        self.set_zn_flags(result, false);
        self.f.set(Flags::H, set_h);
        self.f.remove(Flags::C);
        self.set_r8(&Register8::A, result);
    }

    // Public arithmetic operations
    pub fn add_a_r8(&mut self, register: Register8) {
        self.arithmetic_op_r8(register, false, false, true);
    }

    pub fn adc_a_r8(&mut self, register: Register8) {
        self.arithmetic_op_r8(register, false, true, true);
    }

    pub fn sub_a_r8(&mut self, register: Register8) {
        self.arithmetic_op_r8(register, true, false, true);
    }

    pub fn sbc_a_r8(&mut self, register: Register8) {
        self.arithmetic_op_r8(register, true, true, true);
    }

    // Public logical operations
    pub fn and_a_r8(&mut self, register: Register8) {
        self.logical_op_r8(register, |a, b| a & b, true);
    }

    pub fn xor_a_r8(&mut self, register: Register8) {
        self.logical_op_r8(register, |a, b| a ^ b, false);
    }

    pub fn or_a_r8(&mut self, register: Register8) {
        self.logical_op_r8(register, |a, b| a | b, false);
    }

    // Compare operation: CP A, register
    pub fn cp_a_r8(&mut self, register: Register8) {
        // Perform a subtraction that only updates flags, not the A register
        self.arithmetic_op_r8(register, true, false, false);
    }
}
