use std::result;

use bitflags::Flag;

use crate::cpu::flags::Flags;
use crate::cpu::registers::{Condition, Register16, Register16Mem, Register8};
use crate::cpu::CPU;

impl CPU {
    pub fn add_a_r8(&mut self, register: Register8) {
        // Add value from register to A
        // Z if result=0, N =0, H if overflow bit3, C if overflow bit7
        let value = self.get_r8(&register);
        let original_a = self.get_r8(&Register8::A);
        let result = self.get_r8(&Register8::A).wrapping_add(value);
        self.set_r8(&Register8::A, result);

        let bit3 = (original_a & 0xF) + (value & 0xF) > 0xF;
        let bit7 = (original_a as u16 & 0xFF) + (value as u16 & 0xFF) > 0xFF;

        if bit3 {
            self.f.insert(Flags::H);
        } else {
            self.f.remove(Flags::H);
        }
        if bit7 {
            self.f.insert(Flags::C);
        } else {
            self.f.remove(Flags::C);
        }

        if result == 0 {
            self.f.insert(Flags::Z);
        } else {
            self.f.remove(Flags::Z);
        }
        self.f.remove(Flags::N);
    }
    pub fn adc_a_r8(&mut self, register: Register8) {
        // Add value from register to A with carry
        // Z if result=0, N =0, H if overflow bit3, C if overflow bit7
        let value = self.get_r8(&register);
        let carry_value = if self.f.contains(Flags::C) { 1 } else { 0 };
        let original_a = self.get_r8(&Register8::A);
        let result = original_a.wrapping_add(value).wrapping_add(carry_value);

        self.set_r8(&Register8::A, result);

        let bit3 = (original_a & 0xF) + (value & 0xF) + carry_value > 0xF;
        let bit7 =
            (original_a as u16 & 0xFF) + (value as u16 & 0xFF) + (carry_value as u16 & 0xFF) > 0xFF;

        if bit3 {
            self.f.insert(Flags::H);
        } else {
            self.f.remove(Flags::H);
        }
        if bit7 {
            self.f.insert(Flags::C);
        } else {
            self.f.remove(Flags::C);
        }

        if result == 0 {
            self.f.insert(Flags::Z);
        } else {
            self.f.remove(Flags::Z);
        }
        self.f.remove(Flags::N);
    }
    pub fn sub_a_r8(&mut self, register: Register8) {
        // Subtract value from register from A
        // Z if result=0, N =1, H if borrow bit4, C if register > A
        let value = self.get_r8(&register);
        let original_a = self.get_r8(&Register8::A);
        let result = original_a.wrapping_sub(value);
        self.set_r8(&Register8::A, result);

        let bit4 = (original_a & 0xF) < (value & 0xF);
        let borrow = (value & 0xF) > original_a & 0xF;

        if bit4 {
            self.f.insert(Flags::H);
        } else {
            self.f.remove(Flags::H);
        }
        if borrow {
            self.f.insert(Flags::C);
        } else {
            self.f.remove(Flags::C);
        }

        if result == 0 {
            self.f.insert(Flags::Z);
        } else {
            self.f.remove(Flags::Z);
        }
        self.f.insert(Flags::N);
    }
    pub fn sbc_a_r8(&mut self, register: Register8) {
        // substract with carry
        // Z=result==0, N=1, H=borrow bit4, C=r8+cy > a
        let original_a = self.get_r8(&Register8::A);
        let value = self.get_r8(&register);
        let carry_value = if self.f.contains(Flags::C) { 1 } else { 0 };
        let result = original_a
            .wrapping_sub(value)
            .wrapping_sub(carry_value);
        self.set_r8(&Register8::A, result);
        // Calculate flags
        let half_borrow = (original_a & 0xF) < (value & 0xF) + carry_value;
        let full_borrow = (original_a as u16) < (value as u16 + carry_value as u16);

        if result == 0 {
            self.f.insert(Flags::Z);
        } else {
            self.f.remove(Flags::Z);
        }
        self.f.insert(Flags::N); 

        if half_borrow {
            self.f.insert(Flags::H);
        } else {
            self.f.remove(Flags::H);
        }

        if full_borrow {
            self.f.insert(Flags::C);
        } else {
            self.f.remove(Flags::C);
        }
    }
    pub fn and_a_r8(&mut self, register:Register8){
        // Bitwise &
        // Z=result==0, N=0, H=1, C=0
        let original_a = self.get_r8(&Register8::A);
        let value = self.get_r8(&register);
        let result = original_a & value;
        if result == 0 {
            self.f.insert(Flags::Z);
        } else {
            self.f.remove(Flags::Z);
        }
        self.f.remove(Flags::N);
        self.f.insert(Flags::H);
        self.f.remove(Flags::C);
        self.set_r8(&Register8::A, result);
    }
    pub fn xor_a_r8(&mut self, register:Register8){
        // Bitwise ^
        // Z=result==0, N=0, H=0, C=0
        let original_a = self.get_r8(&Register8::A);
        let value = self.get_r8(&register);
        let result = original_a ^ value;
        if result == 0 {
            self.f.insert(Flags::Z);
        } else {
            self.f.remove(Flags::Z);
        }
        self.f.remove(Flags::N);
        self.f.remove(Flags::H);
        self.f.remove(Flags::C);
        self.set_r8(&Register8::A, result);
    }
    pub fn or_a_r8(&mut self, register:Register8){
        // Bitwise |
        // Z=result==0, N=0, H=0, C=0
        let original_a = self.get_r8(&Register8::A);
        let value = self.get_r8(&register);
        let result = original_a | value;
        if result == 0 {
            self.f.insert(Flags::Z);
        } else {
            self.f.remove(Flags::Z);
        }
        self.f.remove(Flags::N);
        self.f.remove(Flags::H);
        self.f.remove(Flags::C);
        self.set_r8(&Register8::A, result);
    }
    pub fn cp_a_r8(&mut self, register:Register8){
        // Compare A register with r8
        // Z=result==0, N=1, H=borrow bit4, C=r8+cy > a
        let original_a = self.get_r8(&Register8::A);
        let value = self.get_r8(&register);
        let result = original_a.wrapping_sub(value);
        // Calculate flags
        let half_borrow = (original_a & 0xF) < (value & 0xF);
        let full_borrow = (original_a as u16) < (value as u16);

        if result == 0 {
            self.f.insert(Flags::Z);
        } else {
            self.f.remove(Flags::Z);
        }
        self.f.insert(Flags::N); 

        if half_borrow {
            self.f.insert(Flags::H);
        } else {
            self.f.remove(Flags::H);
        }

        if full_borrow {
            self.f.insert(Flags::C);
        } else {
            self.f.remove(Flags::C);
        }
    }
}
