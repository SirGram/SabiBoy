use crate::cpu::registers::{Condition, Register16, Register16Mem, Register8};
use crate::cpu::CPU;

impl CPU {
    pub fn execute(&mut self, opcode: u8) {
        match opcode {
            //opcode file https://izik1.github.io/gbops/
            0x00 => self.nop(),
            0x01 => self.ld_r16_imm16(Register16::BC),
            0x02 => self.ld_r16mem_a(Register16Mem::BC),
            0x03 => self.inc_r16(Register16::BC),
            0x04 => self.inc_r8(Register8::B),
            0x05 => self.dec_r8(Register8::B),
            0x06 => self.ld_r8_imm8(Register8::B),
            0x07 => self.rlca(),
            0x08 => self.ld_imm16_sp(),
            0x09 => self.add_hl_r16(Register16::BC),
            0x0A => self.ld_a_r16mem(Register16Mem::BC),
            0x0B => self.dec_r16(Register16::BC),
            0x0C => self.inc_r8(Register8::C),
            0x0E => self.ld_r8_imm8(Register8::C),
            0x0F => self.rrca(),

            0x10 => self.stop(),
            0x11 => self.ld_r16_imm16(Register16::DE),
            0x12 => self.ld_r16mem_a(Register16Mem::DE),
            0x13 => self.inc_r16(Register16::DE),
            0x14 => self.inc_r8(Register8::D),
            0x15 => self.dec_r8(Register8::D),
            0x16 => self.ld_r8_imm8(Register8::D),
            0x17 => self.rla(),
            0x18 => self.jr_imm8(),
            0x19 => self.add_hl_r16(Register16::DE),
            0x1A => self.ld_a_r16mem(Register16Mem::DE),
            0x1B => self.dec_r16(Register16::DE),
            0x1C => self.inc_r8(Register8::E),
            0x1E => self.ld_r8_imm8(Register8::E),
            0x1F => self.rra(),

            0x20 => self.jr_cond_imm8(Condition::NZ),
            0x21 => self.ld_r16_imm16(Register16::HL),
            0x22 => self.ld_r16mem_a(Register16Mem::HLi),
            0x23 => self.inc_r16(Register16::HL),
            0x24 => self.inc_r8(Register8::H),
            0x25 => self.dec_r8(Register8::H),
            0x26 => self.ld_r8_imm8(Register8::H),
            0x27 => self.daa(),
            0x28 => self.jr_cond_imm8(Condition::Z),
            0x29 => self.add_hl_r16(Register16::HL),
            0x2A => self.ld_a_r16mem(Register16Mem::HLi),
            0x2B => self.dec_r16(Register16::HL),
            0x2C => self.inc_r8(Register8::L),
            0x2E => self.ld_r8_imm8(Register8::L),
            0x2F => self.cpl(),

            0x30 => self.jr_cond_imm8(Condition::NC),
            0x31 => self.ld_r16_imm16(Register16::BC),
            0x32 => self.ld_r16mem_a(Register16Mem::HLd),
            0x33 => self.inc_r16(Register16::SP),
            0x34 => self.inc_r8(Register8::HLIndirect),
            0x35 => self.dec_r8(Register8::HLIndirect),
            0x36 => self.ld_r8_imm8(Register8::HLIndirect),
            0x37 => self.scf(),
            0x38 => self.jr_cond_imm8(Condition::C),
            0x39 => self.add_hl_r16(Register16::SP),
            0x3A => self.ld_a_r16mem(Register16Mem::HLd),
            0x3B => self.dec_r16(Register16::SP),
            0x3C => self.inc_r8(Register8::A),
            0x3E => self.ld_r8_imm8(Register8::A),

            0x40 => self.ld_r8_r8(Register8::B, Register8::B), // NOP
            0x41 => self.ld_r8_r8(Register8::B, Register8::C),
            0x42 => self.ld_r8_r8(Register8::B, Register8::D),
            0x43 => self.ld_r8_r8(Register8::B, Register8::E),
            0x44 => self.ld_r8_r8(Register8::B, Register8::H),
            0x45 => self.ld_r8_r8(Register8::B, Register8::L),
            0x46 => self.ld_r8_r8(Register8::B, Register8::HLIndirect),
            0x47 => self.ld_r8_r8(Register8::B, Register8::A),
            0x48 => self.ld_r8_r8(Register8::C, Register8::B),
            0x49 => self.ld_r8_r8(Register8::C, Register8::C), // NOP
            0x4A => self.ld_r8_r8(Register8::C, Register8::D),
            0x4B => self.ld_r8_r8(Register8::C, Register8::E),
            0x4C => self.ld_r8_r8(Register8::C, Register8::H),
            0x4D => self.ld_r8_r8(Register8::C, Register8::L),
            0x4E => self.ld_r8_r8(Register8::C, Register8::HLIndirect),
            0x4F => self.ld_r8_r8(Register8::C, Register8::A),

            0x50 => self.ld_r8_r8(Register8::D, Register8::B),
            0x51 => self.ld_r8_r8(Register8::D, Register8::C),
            0x52 => self.ld_r8_r8(Register8::D, Register8::D), // NOP
            0x53 => self.ld_r8_r8(Register8::D, Register8::E),
            0x54 => self.ld_r8_r8(Register8::D, Register8::H),
            0x55 => self.ld_r8_r8(Register8::D, Register8::L),
            0x56 => self.ld_r8_r8(Register8::D, Register8::HLIndirect),
            0x57 => self.ld_r8_r8(Register8::D, Register8::A),
            0x58 => self.ld_r8_r8(Register8::E, Register8::B),
            0x59 => self.ld_r8_r8(Register8::E, Register8::C),
            0x5A => self.ld_r8_r8(Register8::E, Register8::D),
            0x5B => self.ld_r8_r8(Register8::E, Register8::E),  // NOP
            0x5C => self.ld_r8_r8(Register8::E, Register8::H),
            0x5D => self.ld_r8_r8(Register8::E, Register8::L),
            0x5E => self.ld_r8_r8(Register8::E, Register8::HLIndirect),
            0x5F => self.ld_r8_r8(Register8::E, Register8::A),

            0x60 => self.ld_r8_r8(Register8::H, Register8::B),
            0x61 => self.ld_r8_r8(Register8::H, Register8::C),
            0x62 => self.ld_r8_r8(Register8::H, Register8::D),
            0x63 => self.ld_r8_r8(Register8::H, Register8::E),
            0x64 => self.ld_r8_r8(Register8::H, Register8::H), // NOP
            0x65 => self.ld_r8_r8(Register8::H, Register8::L),
            0x66 => self.ld_r8_r8(Register8::H, Register8::HLIndirect),
            0x67 => self.ld_r8_r8(Register8::H, Register8::A),
            0x68 => self.ld_r8_r8(Register8::L, Register8::B),
            0x69 => self.ld_r8_r8(Register8::L, Register8::C),
            0x6A => self.ld_r8_r8(Register8::L, Register8::D),
            0x6B => self.ld_r8_r8(Register8::L, Register8::E),
            0x6C => self.ld_r8_r8(Register8::L, Register8::H),
            0x6D => self.ld_r8_r8(Register8::L, Register8::L), // NOP
            0x6E => self.ld_r8_r8(Register8::L, Register8::HLIndirect),
            0x6F => self.ld_r8_r8(Register8::L, Register8::A),

            0x70 => self.ld_r8_r8(Register8::HLIndirect, Register8::B),
            0x71 => self.ld_r8_r8(Register8::HLIndirect, Register8::C),
            0x72 => self.ld_r8_r8(Register8::HLIndirect, Register8::D),
            0x73 => self.ld_r8_r8(Register8::HLIndirect, Register8::E),
            0x74 => self.ld_r8_r8(Register8::HLIndirect, Register8::H),
            0x75 => self.ld_r8_r8(Register8::HLIndirect, Register8::L),
            0x76 => self.halt(),
            0x77 => self.ld_r8_r8(Register8::HLIndirect, Register8::A),
            0x78 => self.ld_r8_r8(Register8::A, Register8::B),
            0x79 => self.ld_r8_r8(Register8::A, Register8::C),
            0x7A => self.ld_r8_r8(Register8::A, Register8::D),
            0x7B => self.ld_r8_r8(Register8::A, Register8::E),
            0x7C => self.ld_r8_r8(Register8::A, Register8::H),
            0x7D => self.ld_r8_r8(Register8::A, Register8::L),
            0x7E => self.ld_r8_r8(Register8::A, Register8::HLIndirect),
            0x7F => self.ld_r8_r8(Register8::A, Register8::A), // NOP

            _ => {
                println!("opcode: {:?}", opcode)
            }
        }
    }
}
