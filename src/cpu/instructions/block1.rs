use std::result;

use crate::cpu::flags::Flags;
use crate::cpu::registers::{Register16, Register16Mem, Register8};
use crate::cpu::CPU;

impl CPU {
    pub fn ld_r8_r8(&mut self, register1: Register8, register2: Register8) {
        self.set_r8(&register1, self.get_r8(&register2));
    }
    pub fn halt(&mut self) {
        /*
        TODO: Implement HALT instruction
        Enter CPU low-power consumption mode until an interrupt occurs. The exact behavior of this instruction depends on the state of the IME flag.

        IME set
        The CPU enters low-power mode until after an interrupt is about to be serviced. The handler is executed normally, and the CPU resumes execution after the HALT when that returns.
        IME not set
        The behavior depends on whether an interrupt is pending (i.e. ‘[IE] & [IF]’ is non-zero).
        None pending
        As soon as an interrupt becomes pending, the CPU resumes execution. This is like the above, except that the handler is not called.
        Some pending
        The CPU continues execution after the HALT, but the byte after it is read twice in a row (PC is not incremented, due to a hardware bug). */
    }

    // $CB prefix instructions
    pub fn rlc_r8(&mut self, register: Register8) {
        // rotate left with carry
        let value = self.get_r8(&register);
        let result = value.rotate_left(1);
        self.set_r8(&register, result);
        let carry = value as u16 & 0x80 != 0;
        self.set_zn_flags(result, false);
        self.f.set(Flags::C, carry);
        self.f.set(Flags::H, false);
    }
    pub fn rrc_r8(&mut self, register: Register8) {
        // rotate right with carry
        let value = self.get_r8(&register);
        let result = value.rotate_right(1);
        self.set_r8(&register, result);
        let carry = value as u16 & 0x01 != 0;
        self.set_zn_flags(result, false);
        self.f.set(Flags::C, carry);
        self.f.set(Flags::H, false);
    }
    pub fn rl_r8(&mut self, register: Register8) {
        // shifts a to the left 1 bit and stores in c flag
        // old c flag is moved to bit 0
        let original_value = self.get_r8(&register);
        let bit7 = original_value & 0x80 != 0;
        let mut result = original_value << 1;
        if self.f.contains(Flags::C) {
            result |= 0x01;
        }
        if bit7 {
            self.f.insert(Flags::C);
        } else {
            self.f.remove(Flags::C);
        }
        self.set_zn_flags(result, false);
        self.f.remove(Flags::H);
        self.set_r8(&register, result);
    }
    pub fn rr_r8(&mut self, register: Register8) {
        // shifts a to the right 1 bit and stores in c flag
        // old c flag is moved to bit 7
        let original_value = self.get_r8(&register);
        let bit0 = original_value & 0x01 != 0;
        let mut result = original_value >> 1;
        if self.f.contains(Flags::C) {
            result |= 0x80;
        }
        if bit0 {
            self.f.insert(Flags::C);
        } else {
            self.f.remove(Flags::C);
        }
        self.set_zn_flags(result, false);
        self.f.remove(Flags::H);
        self.set_r8(&register, result);
    }
    pub fn sla_r8(&mut self, register: Register8) {
        // shift left 1 bit. carry = bit7, bit0 = 0
        let original_value = self.get_r8(&register);
        let bit7 = original_value & 0x80 != 0;
        let result = original_value << 1;
        if bit7 {
            self.f.insert(Flags::C);
        }
        self.set_zn_flags(result, false);
        self.f.remove(Flags::H);
        self.set_r8(&register, result);
    }
    pub fn sra_r8(&mut self, register: Register8) {
        // shift right 1 bit. carry = bit0, bit7 = unchanged
        let original_value = self.get_r8(&register);
        let bit0 = original_value & 0x01 != 0;
        let bit7 = original_value & 0x80 != 0;
        let mut result = original_value >> 1;
        if bit7 {
            result |= 0x80;
        }

        self.f.set(Flags::C, bit0);
        self.set_zn_flags(result, false);
        self.f.remove(Flags::H);
        self.set_r8(&register, result);
    }
    pub fn swap_r8(&mut self, register: Register8) {
        // swap nibbles
        let original_value = self.get_r8(&register);
        let result = (original_value & 0xF0) >> 4 | (original_value & 0x0F) << 4;
        self.set_r8(&register, result);
    }
    pub fn srl_r8(&mut self, register: Register8) {
        // shift right 1 bit. carry = bit0, bit7 = 0
        let original_value = self.get_r8(&register);
        let bit0 = original_value & 0x01 != 0;
        let result = original_value >> 1;
        self.f.set(Flags::C, bit0);
        self.set_zn_flags(result, false);
        self.f.remove(Flags::H);
        self.set_r8(&register, result);
    }
    pub fn bit_b3_r8(&mut self, register: Register8, selected_bit: u8) {
        // Z=0 if bit selected is set, otherwise Z=1
        let value = self.get_r8(&register);
        let bit_zero = value & (1 << selected_bit) == 0;
        self.f.set(Flags::Z, bit_zero);
        self.f.remove(Flags::N);
        self.f.remove(Flags::H);
    }
    pub fn res_b3_r8(&mut self, register: Register8, selected_bit: u8) {
        // reset bit selected
        let value = self.get_r8(&register);
        let result = value & !(1 << selected_bit);
        self.set_r8(&register, result);
    }
    pub fn set_b3_r8(&mut self, register: Register8, selected_bit: u8) {
        // set bit selected
        let value = self.get_r8(&register);
        let result = value | (1 << selected_bit);
        self.set_r8(&register, result);
    }
}
