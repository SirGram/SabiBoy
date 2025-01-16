use crate::bus::MemoryInterface;
use crate::cpu::flags::Flags;
use crate::cpu::registers::Register8;
use crate::cpu::CPU;

impl CPU {
    fn arithmetic_op_r8<M: MemoryInterface>(
        &mut self,
        register: Register8,
        is_subtract: bool,
        use_carry: bool,
        update_register: bool,
        memory: &mut M,
    ) {
        let value = self.get_r8(&register, memory);
        let original_a = self.get_r8(&Register8::A, memory);
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
            self.set_r8(&Register8::A, result, memory);
        }
    }

    // Generic function for logical operations
    fn logical_op_r8<M: MemoryInterface>(
        &mut self,
        register: Register8,
        op: impl Fn(u8, u8) -> u8,
        set_h: bool,
        memory: &mut M,
    ) {
        let original_a = self.get_r8(&Register8::A, memory);
        let value = self.get_r8(&register, memory);
        let result = op(original_a, value);

        self.set_zn_flags(result, false);
        self.f.set(Flags::H, set_h);
        self.f.remove(Flags::C);
        self.set_r8(&Register8::A, result, memory);
    }

    // Public arithmetic operations
    pub fn add_a_r8<M: MemoryInterface>(&mut self, register: Register8, memory: &mut M) {
        self.arithmetic_op_r8(register, false, false, true, memory);
    }

    pub fn adc_a_r8<M: MemoryInterface>(&mut self, register: Register8, memory: &mut M) {
        self.arithmetic_op_r8(register, false, true, true, memory);
    }

    pub fn sub_a_r8<M: MemoryInterface>(&mut self, register: Register8, memory: &mut M) {
        self.arithmetic_op_r8(register, true, false, true, memory);
    }

    pub fn sbc_a_r8<M: MemoryInterface>(&mut self, register: Register8, memory: &mut M) {
        self.arithmetic_op_r8(register, true, true, true, memory);
    }

    // Public logical operations
    pub fn and_a_r8<M: MemoryInterface>(&mut self, register: Register8, memory: &mut M) {
        self.logical_op_r8(register, |a, b| a & b, true, memory);
    }

    pub fn xor_a_r8<M: MemoryInterface>(&mut self, register: Register8, memory: &mut M) {
        self.logical_op_r8(register, |a, b| a ^ b, false, memory);
    }

    pub fn or_a_r8<M: MemoryInterface>(&mut self, register: Register8, memory: &mut M) {
        self.logical_op_r8(register, |a, b| a | b, false, memory);
    }

    // Compare operation: CP A, register
    pub fn cp_a_r8<M: MemoryInterface>(&mut self, register: Register8, memory: &mut M) {
        // Perform a subtraction that only updates flags, not the A register
        self.arithmetic_op_r8(register, true, false, false, memory);
    }
}
