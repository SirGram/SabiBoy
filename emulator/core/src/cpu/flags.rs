use crate::cpu::CPU;
use bitflags::bitflags;

bitflags! {
    pub struct Flags: u8 {
        const Z = 0b10000000; // zero
        const N = 0b01000000; // subtraction
        const H = 0b00100000; // half carry
        const C = 0b00010000; // carry
    }
}

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Flags::from_bits_truncate(value)
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Condition {
    NZ,
    Z,
    NC,
    C,
}

impl CPU {
    pub fn set_zn_flags(&mut self, result: u8, set_n: bool) {
        if result == 0 {
            self.f.insert(Flags::Z);
        } else {
            self.f.remove(Flags::Z);
        }
        if set_n {
            self.f.insert(Flags::N);
        } else {
            self.f.remove(Flags::N);
        }
    }

    pub fn set_add_flags(&mut self, original: u8, value: u8, carry: u8) {
        // Helper function for setting H and C flags for addition
        let bit3 = (original & 0xF) + (value & 0xF) + carry > 0xF;
        let bit7 = (original as u16) + (value as u16) + (carry as u16) > 0xFF;

        self.f.set(Flags::H, bit3);
        self.f.set(Flags::C, bit7);
    }

    pub fn set_sub_flags(&mut self, original: u8, value: u8, carry: u8) {
        // Helper function for setting H and C flags for subtraction
        let half_borrow = (original & 0xF) < (value & 0xF) + carry;
        let full_borrow = (original as u16) < (value as u16) + (carry as u16);

        self.f.set(Flags::H, half_borrow);
        self.f.set(Flags::C, full_borrow);
    }
}
