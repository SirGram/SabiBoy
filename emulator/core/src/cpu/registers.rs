use crate::bus::MemoryInterface;
use crate::cpu::flags::Flags;
use crate::cpu::CPU;

#[derive(Debug, Clone, Copy)]
pub enum Register16 {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Debug, Clone, Copy)]
pub enum Register8 {
    B,
    C,
    D,
    E,
    H,
    L,
    HLIndirect,
    A,
}

#[derive(Debug, Clone, Copy)]
pub enum Register16Mem {
    BC,
    DE,
    HLi, // HL increment
    HLd, // HL decrement
}

#[derive(Debug, Clone, Copy)]
pub enum Register16Stk {
    BC,
    DE,
    HL,
    AF,
}

impl CPU {
    pub fn get_r8<M: MemoryInterface>(&self, register: &Register8, memory: &M) -> u8 {
        match register {
            Register8::B => self.b,
            Register8::C => self.c,
            Register8::D => self.d,
            Register8::E => self.e,
            Register8::H => self.h,
            Register8::L => self.l,
            Register8::HLIndirect => {
                let hl = self.get_r16(&Register16::HL);
                memory.read_byte(hl)
            }
            Register8::A => self.a,
        }
    }
    pub fn set_r8<M: MemoryInterface>(&mut self, register: &Register8, value: u8, memory: &mut M) {
        match register {
            Register8::B => self.b = value,
            Register8::C => self.c = value,
            Register8::D => self.d = value,
            Register8::E => self.e = value,
            Register8::H => self.h = value,
            Register8::L => self.l = value,
            Register8::HLIndirect => {
                let hl = self.get_r16(&Register16::HL);
                memory.write_byte(hl, value);
            }
            Register8::A => self.a = value,
        }
    }
    pub fn get_r16mem(&mut self, register: &Register16Mem) -> u16 {
        // Get address from register16 bus
        match register {
            Register16Mem::BC => ((self.b as u16) << 8) | (self.c as u16),
            Register16Mem::DE => ((self.d as u16) << 8) | (self.e as u16),
            Register16Mem::HLi => {
                let hl = self.get_r16(&Register16::HL);
                self.set_r16(&Register16::HL, hl.wrapping_add(1));
                hl
            }
            Register16Mem::HLd => {
                let hl = self.get_r16(&Register16::HL);
                self.set_r16(&Register16::HL, hl.wrapping_sub(1));
                hl
            }
        }
    }
    pub fn get_r16(&self, register: &Register16) -> u16 {
        match register {
            Register16::BC => ((self.b as u16) << 8) | self.c as u16,
            Register16::DE => ((self.d as u16) << 8) | self.e as u16,
            Register16::HL => ((self.h as u16) << 8) | self.l as u16,
            Register16::SP => self.sp,
        }
    }

    pub fn set_r16(&mut self, register: &Register16, value: u16) {
        match register {
            Register16::BC => {
                self.b = (value >> 8) as u8;
                self.c = (value & 0xFF) as u8;
            }
            Register16::DE => {
                self.d = (value >> 8) as u8;
                self.e = (value & 0xFF) as u8;
            }
            Register16::HL => {
                self.h = (value >> 8) as u8;
                self.l = (value & 0xFF) as u8;
            }
            Register16::SP => self.sp = value,
        }
    }
    pub fn get_r16stk(&self, register: &Register16Stk) -> u16 {
        match register {
            Register16Stk::BC => self.b as u16 | (self.c as u16) << 8,
            Register16Stk::DE => self.d as u16 | (self.e as u16) << 8,
            Register16Stk::HL => self.h as u16 | (self.l as u16) << 8,
            Register16Stk::AF => self.a as u16 | (self.f.bits() as u16) << 8,
        }
    }
    pub fn set_r16stk(&mut self, register: &Register16Stk, value: u16) {
        match register {
            Register16Stk::BC => {
                self.b = (value >> 8) as u8;
                self.c = (value & 0xFF) as u8;
            }
            Register16Stk::DE => {
                self.d = (value >> 8) as u8;
                self.e = (value & 0xFF) as u8;
            }
            Register16Stk::HL => {
                self.h = (value >> 8) as u8;
                self.l = (value & 0xFF) as u8;
            }
            Register16Stk::AF => {
                self.a = (value >> 8) as u8;
                self.f = Flags::from_bits_truncate(value as u8);
            }
        }
    }
}
