use crate::bus::io_address::IoRegister;
use crate::bus::MemoryInterface;
use crate::cpu::flags::Flags;
use crate::cpu::registers::Register8;
use crate::cpu::CPU;

impl CPU {
    pub fn ld_r8_r8<M: MemoryInterface>(
        &mut self,
        register1: Register8,
        register2: Register8,
        memory: &mut M,
    ) {
        self.set_r8(&register1, self.get_r8(&register2, memory), memory);
    }

    pub fn halt<M: MemoryInterface>(&mut self, memory: &mut M) {
        // Read the IE (interrupt enable) and IF (interrupt flag) registers
        let ie_register = memory.read_byte(IoRegister::Ie.address());
        let if_register = memory.read_byte(IoRegister::If.address());
        let interrupts = ie_register & if_register;

        if !self.ime && interrupts != 0 {
            // HALT bug occurs when IME is disabled and an interrupt is pending
            self.halt_bug = true;
        } else {
            // Either IME is enabled, or no interrupts are pending
            self.halt = true;
        }
    }

    // $CB prefix instructions
    pub fn rlc_r8<M: MemoryInterface>(&mut self, register: Register8, memory: &mut M) {
        // rotate left with carry
        let value = self.get_r8(&register, memory);
        let result = value.rotate_left(1);
        self.set_r8(&register, result, memory);
        let carry = value as u16 & 0x80 != 0;
        self.set_zn_flags(result, false);
        self.f.set(Flags::C, carry);
        self.f.set(Flags::H, false);
    }

    pub fn rrc_r8<M: MemoryInterface>(&mut self, register: Register8, memory: &mut M) {
        // rotate right with carry
        let value = self.get_r8(&register, memory);
        let result = value.rotate_right(1);
        self.set_r8(&register, result, memory);
        let carry = value as u16 & 0x01 != 0;
        self.set_zn_flags(result, false);
        self.f.set(Flags::C, carry);
        self.f.set(Flags::H, false);
    }

    pub fn rl_r8<M: MemoryInterface>(&mut self, register: Register8, memory: &mut M) {
        // shifts a to the left 1 bit and stores in c flag
        // old c flag is moved to bit 0
        let original_value = self.get_r8(&register, memory);
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
        self.set_r8(&register, result, memory);
    }

    pub fn rr_r8<M: MemoryInterface>(&mut self, register: Register8, memory: &mut M) {
        // shifts a to the right 1 bit and stores in c flag
        // old c flag is moved to bit 7
        let original_value = self.get_r8(&register, memory);
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
        self.set_r8(&register, result, memory);
    }

    pub fn sla_r8<M: MemoryInterface>(&mut self, register: Register8, memory: &mut M) {
        // shift left 1 bit. carry = bit7, bit0 = 0
        let original_value = self.get_r8(&register, memory);
        let bit7 = original_value & 0x80 != 0;
        let result = original_value << 1;
        if bit7 {
            self.f.insert(Flags::C);
        } else {
            self.f.remove(Flags::C);
        }
        self.set_zn_flags(result, false);
        self.f.remove(Flags::H);
        self.set_r8(&register, result, memory);
    }

    pub fn sra_r8<M: MemoryInterface>(&mut self, register: Register8, memory: &mut M) {
        // shift right 1 bit. carry = bit0, bit7 = unchanged
        let original_value = self.get_r8(&register, memory);
        let bit0 = original_value & 0x01 != 0;
        let bit7 = original_value & 0x80 != 0;
        let mut result = original_value >> 1;
        if bit7 {
            result |= 0x80;
        }

        self.f.set(Flags::C, bit0);
        self.set_zn_flags(result, false);
        self.f.remove(Flags::H);
        self.set_r8(&register, result, memory);
    }

    pub fn swap_r8<M: MemoryInterface>(&mut self, register: Register8, memory: &mut M) {
        // swap nibbles
        let original_value = self.get_r8(&register, memory);
        let result = (original_value & 0xF0) >> 4 | (original_value & 0x0F) << 4;
        self.set_r8(&register, result, memory);
        self.set_zn_flags(result, false);
        self.f.remove(Flags::H);
        self.f.remove(Flags::C);
    }

    pub fn srl_r8<M: MemoryInterface>(&mut self, register: Register8, memory: &mut M) {
        // shift right 1 bit. carry = bit0, bit7 = 0
        let original_value = self.get_r8(&register, memory);
        let bit0 = original_value & 0x01 != 0;
        let result = original_value >> 1;
        self.f.set(Flags::C, bit0);
        self.set_zn_flags(result, false);
        self.f.remove(Flags::H);
        self.set_r8(&register, result, memory);
    }

    pub fn bit_b3_r8<M: MemoryInterface>(
        &mut self,
        register: Register8,
        selected_bit: u8,
        memory: &mut M,
    ) {
        // Z=0 if bit selected is set, otherwise Z=1
        let value = self.get_r8(&register, memory);
        let bit_zero = value & (1 << selected_bit) == 0;
        self.f.set(Flags::Z, bit_zero);
        self.f.remove(Flags::N);
        self.f.insert(Flags::H);
    }

    pub fn res_b3_r8<M: MemoryInterface>(
        &mut self,
        register: Register8,
        selected_bit: u8,
        memory: &mut M,
    ) {
        // reset bit selected
        let value = self.get_r8(&register, memory);
        let result = value & !(1 << selected_bit);
        self.set_r8(&register, result, memory);
    }

    pub fn set_b3_r8<M: MemoryInterface>(
        &mut self,
        register: Register8,
        selected_bit: u8,
        memory: &mut M,
    ) {
        // set bit selected
        let value = self.get_r8(&register, memory);
        let result = value | (1 << selected_bit);
        self.set_r8(&register, result, memory);
    }
}
