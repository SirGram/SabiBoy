
use crate::cpu::CPU;
use crate::cpu::registers::{Register16, Register8, Register16Mem, Condition};

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

                _ => {
                    println!("opcode: {:?}", opcode)
                }
            }
               
    }
}