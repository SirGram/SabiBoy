use std::result;

use crate::bus::MemoryInterface;
use crate::cpu::fetch::*;
use crate::cpu::flags::{Condition, Flags};
use crate::cpu::registers::{Register16, Register16Mem, Register8};
use crate::cpu::CPU;

impl CPU {
    pub fn nop(&mut self) {}

    pub fn ld_r16_imm16<M: MemoryInterface>(&mut self, register: Register16, memory: &mut M) {
        // Load 16 bit immediate value into register16
        let imm16 = self.fetch_word(memory);
        self.set_r16(&register, imm16);
    }

    pub fn ld_r16mem_a<M: MemoryInterface>(&mut self, register: Register16Mem, memory: &mut M) {
        // Load A register into bus location pointed to by register16
        // HL register is incremented or decremented after storing
        let address = self.get_r16mem(&register);
        let value = self.get_r8(&Register8::A, memory);
        memory.write_byte(address, value);
    }

    pub fn ld_a_r16mem<M: MemoryInterface>(&mut self, register: Register16Mem, memory: &mut M) {
        // A register value is loaded from bus location pointed to by register16
        // HL register is incremented or decremented after storing
        let address = self.get_r16mem(&register);
        let value = memory.read_byte(address);
        self.set_r8(&Register8::A, value, memory);
    }

    pub fn ld_imm16_sp<M: MemoryInterface>(&mut self, memory: &mut M) {
        // Load SP register into [imm16]
        let imm16 = self.fetch_word(memory);
        memory.write_word(imm16, self.sp);
    }

    pub fn inc_r16(&mut self, register: Register16) {
        // Increment function for 16-bit registers
        let value = self.get_r16(&register).wrapping_add(1);
        self.set_r16(&register, value);
    }

    pub fn dec_r16(&mut self, register: Register16) {
        // Decrement function for 16-bit registers
        let value = self.get_r16(&register).wrapping_sub(1);
        self.set_r16(&register, value);
    }

    pub fn add_hl_r16(&mut self, register: Register16) {
        // Add register16 value to HL
        // N= 0, H IF overflow bit 11, C IF overflow bit 15
        let hl = self.get_r16(&Register16::HL);
        let value = self.get_r16(&register);
        self.set_r16(&Register16::HL, hl.wrapping_add(value));
        self.f.remove(Flags::N);
        let overflow_bit11 = (hl & 0xFFF) + (value & 0xFFF) > 0xFFF;
        let overflow_bit15 = hl as u32 + value as u32 > 0xFFFF;
        if overflow_bit15 {
            self.f.insert(Flags::C);
        } else {
            self.f.remove(Flags::C);
        }
        if overflow_bit11 {
            self.f.insert(Flags::H);
        } else {
            self.f.remove(Flags::H);
        }
    }

    pub fn inc_r8<M: MemoryInterface>(&mut self, register: Register8, memory: &mut M) {
        // Increment function for 8-bit registers
        // Z if result= 0, N=0, H if overflow bit 3
        let value = self.get_r8(&register, memory).wrapping_add(1);
        self.set_r8(&register, value, memory);
        if value == 0 {
            self.f.insert(Flags::Z);
        } else {
            self.f.remove(Flags::Z);
        }
        self.f.remove(Flags::N);
        let overflow_bit3 = value & 0xF == 0;
        if overflow_bit3 {
            self.f.insert(Flags::H);
        } else {
            self.f.remove(Flags::H);
        }
    }

    pub fn dec_r8<M: MemoryInterface>(&mut self, register: Register8, memory: &mut M) {
        // Decrement function for 8-bit registers
        // Z if result= 0, N=1, H if borrow bit 4
        let value = self.get_r8(&register, memory).wrapping_sub(1);
        self.set_r8(&register, value, memory);
        if value == 0 {
            self.f.insert(Flags::Z);
        } else {
            self.f.remove(Flags::Z);
        }
        self.f.insert(Flags::N);
        let overflow_bit4 = value & 0xF == 0xF;
        if overflow_bit4 {
            self.f.insert(Flags::H);
        } else {
            self.f.remove(Flags::H);
        }
    }

    pub fn ld_r8_imm8<M: MemoryInterface>(&mut self, register: Register8, memory: &mut M) {
        // Load imm8 into r8
        let imm8 = self.fetch_byte(memory);
        self.set_r8(&register, imm8, memory);
    }

    pub fn rlca(&mut self) {
        // Rotate a register left
        // Z=0, N=0, H=0, C= bit rotated
        let original_a = self.a;
        self.a = self.a.rotate_left(1);
        if original_a & 0x80 != 0 {
            self.f.insert(Flags::C);
        } else {
            self.f.remove(Flags::C);
        }
        self.f.remove(Flags::Z);
        self.f.remove(Flags::N);
        self.f.remove(Flags::H);
    }

    pub fn rrca(&mut self) {
        let original_a = self.a;
        self.a = self.a.rotate_right(1);
        if original_a & 0x01 != 0 {
            self.f.insert(Flags::C);
        } else {
            self.f.remove(Flags::C);
        }
        self.f.remove(Flags::Z);
        self.f.remove(Flags::N);
        self.f.remove(Flags::H);
    }

    pub fn rla(&mut self) {
        // shifts a to the left 1 bit and stores in c flag
        // old c flag is moved to bit 0
        let bit7 = self.a & 0x80 != 0;
        self.a = self.a << 1;
        if self.f.contains(Flags::C) {
            self.a |= 0x01;
        }
        if bit7 {
            self.f.insert(Flags::C);
        } else {
            self.f.remove(Flags::C);
        }
        self.f.remove(Flags::Z);
        self.f.remove(Flags::N);
        self.f.remove(Flags::H);
    }

    pub fn rra(&mut self) {
        // shifts a to the right 1 bit and stores in c flag
        // old c flag is moved to bit 7
        let bit0 = self.a & 0x01 != 0;
        self.a = self.a >> 1;
        if self.f.contains(Flags::C) {
            self.a |= 0x80;
        }
        if bit0 {
            self.f.insert(Flags::C);
        } else {
            self.f.remove(Flags::C);
        }
        self.f.remove(Flags::Z);
        self.f.remove(Flags::N);
        self.f.remove(Flags::H);
    }

    pub fn daa<M: MemoryInterface>(&mut self, memory: &mut M) {
        // Decimal adjustment for register A
        // e.g. 0x4A => 0x50
        // half carry lower nibble. carry upper nibble
        let original_a = self.get_r8(&Register8::A, memory);
        let mut result = original_a;
        let mut carry = self.f.contains(Flags::C);
        if self.f.contains(Flags::N) {
            if self.f.contains(Flags::C) {
                result = result.wrapping_sub(0x60);
            }
            if self.f.contains(Flags::H) {
                result = result.wrapping_sub(0x06);
            }
        } else {
            if self.f.contains(Flags::C) || result > 0x99 {
                result = result.wrapping_add(0x60);
                carry = true;
            }
            if self.f.contains(Flags::H) || (result & 0x0F) > 0x09 {
                result = result.wrapping_add(0x06);
            }
        }
        self.set_r8(&Register8::A, result, memory);
        self.f.set(Flags::Z, result == 0);
        self.f.set(Flags::H, false);
        self.f.set(Flags::C, carry);
    }

    pub fn cpl(&mut self) {
        // Complement register A
        self.a = !self.a;
        self.f.insert(Flags::N);
        self.f.insert(Flags::H);
    }

    pub fn scf(&mut self) {
        // Set carry flag
        self.f.insert(Flags::C);
        self.f.remove(Flags::H);
        self.f.remove(Flags::N);
    }

    pub fn ccf(&mut self) {
        // Complement carry flag
        self.f.remove(Flags::H);
        self.f.remove(Flags::N);
        if self.f.contains(Flags::C) {
            self.f.remove(Flags::C);
        } else {
            self.f.insert(Flags::C);
        }
    }

    pub fn jr_imm8<M: MemoryInterface>(&mut self, memory: &mut M) {
        // Jump to address relative to PC
        let imm8: i8 = self.fetch_byte(memory) as i8;
        self.pc = self.pc.wrapping_add(imm8 as u16);
    }

    pub fn jr_cond_imm8<M: MemoryInterface>(&mut self, condition: Condition, memory: &mut M) {
        let imm8: i8 = self.fetch_byte(memory) as i8;

        // Determine if we should jump based on the condition
        let should_jump = self.should_jump(condition);

        if should_jump {
            self.pc = self.pc.wrapping_add(imm8 as u16);
        }
    }

    pub fn stop(&mut self) {
        // TODO: Implement stop instruction
        // This instruction is used in GBC
    }
}
