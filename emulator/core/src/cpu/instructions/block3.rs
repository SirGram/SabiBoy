use crate::bus::MemoryInterface;
use crate::cpu::flags::{Condition, Flags};
use crate::cpu::registers::{Register16, Register16Stk, Register8};
use crate::cpu::{RstVec, CPU};

impl CPU {
    fn arithmetic_op_imm8<M: MemoryInterface>(
        &mut self,
        op: impl Fn(u8, u8) -> u8,
        set_n: bool,
        use_carry: bool,
        memory: &mut M,
    ) {
        let imm8 = self.fetch_byte(memory);
        let original_a = self.get_r8(&Register8::A, memory);
        let carry = if use_carry {
            self.f.contains(Flags::C) as u8
        } else {
            0
        };

        let result = if set_n {
            // Subtraction
            let temp = original_a.wrapping_sub(imm8);
            if use_carry {
                temp.wrapping_sub(carry)
            } else {
                temp
            }
        } else {
            // Addition
            let temp = original_a.wrapping_add(imm8);
            if use_carry {
                temp.wrapping_add(carry)
            } else {
                temp
            }
        };

        if set_n {
            self.set_sub_flags(original_a, imm8, carry);
        } else {
            self.set_add_flags(original_a, imm8, carry);
        }

        self.set_zn_flags(result, set_n);
        self.set_r8(&Register8::A, result, memory);
    }

    pub fn add_a_imm8<M: MemoryInterface>(&mut self, memory: &mut M) {
        self.arithmetic_op_imm8(
            |a, b| a.wrapping_add(b),
            false, // N flag
            false, // don't use carry
            memory,
        );
    }

    pub fn adc_a_imm8<M: MemoryInterface>(&mut self, memory: &mut M) {
        self.arithmetic_op_imm8(
            |a, b| a.wrapping_add(b),
            false, // N flag
            true,  // use carry
            memory,
        );
    }

    pub fn sub_a_imm8<M: MemoryInterface>(&mut self, memory: &mut M) {
        self.arithmetic_op_imm8(
            |a, b| a.wrapping_sub(b),
            true,  // N flag
            false, // don't use carry
            memory,
        );
    }

    pub fn sbc_a_imm8<M: MemoryInterface>(&mut self, memory: &mut M) {
        self.arithmetic_op_imm8(
            |a, b| a.wrapping_sub(b),
            true, // N flag
            true, // use carry
            memory,
        );
    }

    fn logical_op_imm8<M: MemoryInterface>(
        &mut self,
        op: impl Fn(u8, u8) -> u8,
        set_h: bool,
        memory: &mut M,
    ) {
        let imm8 = self.fetch_byte(memory);
        let original_a = self.get_r8(&Register8::A, memory);
        let result = op(original_a, imm8);

        self.set_zn_flags(result, false);
        self.f.set(Flags::H, set_h);
        self.f.remove(Flags::C);
        self.set_r8(&Register8::A, result, memory);
    }

    pub fn and_a_imm8<M: MemoryInterface>(&mut self, memory: &mut M) {
        self.logical_op_imm8(|a, b| a & b, true, memory);
    }

    pub fn xor_a_imm8<M: MemoryInterface>(&mut self, memory: &mut M) {
        self.logical_op_imm8(|a, b| a ^ b, false, memory);
    }

    pub fn or_a_imm8<M: MemoryInterface>(&mut self, memory: &mut M) {
        self.logical_op_imm8(|a, b| a | b, false, memory);
    }

    pub fn cp_a_imm8<M: MemoryInterface>(&mut self, memory: &mut M) {
        let imm8 = self.fetch_byte(memory);
        let original_a = self.get_r8(&Register8::A, memory);
        let result = original_a.wrapping_sub(imm8);

        self.set_sub_flags(original_a, imm8, 0);
        self.set_zn_flags(result, true);
    }

    // returns
    pub fn ret<M: MemoryInterface>(&mut self, memory: &mut M) {
        // return from subroutine
        // pop pc from stack
        /* ld (pc), [sp] ;
        inc sp */
        let word = memory.read_word(self.sp);
        self.pc = word;
        self.sp = self.sp.wrapping_add(2);
    }

    pub fn ret_cc<M: MemoryInterface>(&mut self, condition: Condition, memory: &mut M) {
        let should_jump = self.should_jump(condition);
        if should_jump {
            self.ret(memory);
        }
    }

    pub fn reti<M: MemoryInterface>(&mut self, memory: &mut M) {
        // enable interrupts and return
        self.ei();
        self.ret(memory);
    }

    pub fn jp_cond_imm16<M: MemoryInterface>(&mut self, condition: Condition, memory: &mut M) {
        // Jump to address relative to PC based on condition
        let imm16 = self.fetch_word(memory);
        let should_jump = self.should_jump(condition);
        if should_jump {
            self.pc = imm16;
        }
    }

    pub fn jp_imm16<M: MemoryInterface>(&mut self, memory: &mut M) {
        // Jump to address relative to PC
        let imm16 = self.fetch_word(memory);
        self.pc = imm16;
    }

    pub fn jp_hl<M: MemoryInterface>(&mut self, memory: &mut M) {
        // Jump to address pointed to by HL
        let hl = self.get_r16(&Register16::HL);
        self.pc = hl;
    }

    pub fn call_imm16<M: MemoryInterface>(&mut self, memory: &mut M) {
        // Push next instruction onto stack and jump to address
        let ret_address = self.pc.wrapping_add(2);
        let [low_byte, high_byte] = ret_address.to_le_bytes();
        self.sp = self.sp.wrapping_sub(1);
        memory.write_byte(self.sp, high_byte);
        self.sp = self.sp.wrapping_sub(1);
        memory.write_byte(self.sp, low_byte);

        let address = self.fetch_word(memory);
        self.pc = address;
    }

    pub fn call_cond_imm16<M: MemoryInterface>(&mut self, condition: Condition, memory: &mut M) {
        let should_jump = self.should_jump(condition);
        if should_jump {
            self.call_imm16(memory);
        } else {
            self.fetch_word(memory);
        }
    }

    pub fn rst_tgt3<M: MemoryInterface>(&mut self, tgt3: RstVec, memory: &mut M) {
        // Similar to call, but address has 8 options
        let ret_address = self.pc;
        let [low_byte, high_byte] = ret_address.to_le_bytes();
        self.sp = self.sp.wrapping_sub(1);
        memory.write_byte(self.sp, high_byte);
        self.sp = self.sp.wrapping_sub(1);
        memory.write_byte(self.sp, low_byte);

        self.pc = tgt3 as u16;
    }

    pub fn pop_r16stk<M: MemoryInterface>(&mut self, register: Register16Stk, memory: &mut M) {
        // Pop value from stack into register16stk
        let low_byte = memory.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);
        let high_byte = memory.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);
        let value = u16::from_le_bytes([low_byte, high_byte]);
        self.set_r16stk(&register, value);
    }
    pub fn push_r16stk<M: MemoryInterface>(&mut self, register: Register16Stk, memory: &mut M) {
        // Push value from register16stk into stack
        // High value then Low value
        let value = self.get_r16stk(&register);
        let [low_value, high_value] = value.to_le_bytes();
        self.sp = self.sp.wrapping_sub(1);
        memory.write_byte(self.sp, low_value);
        self.sp = self.sp.wrapping_sub(1);
        memory.write_byte(self.sp, high_value);
    }
    pub fn ldh_c_a<M: MemoryInterface>(&mut self, memory: &mut M) {
        // Store value in register A into the byte at address $FF00+C
        let value = self.get_r8(&Register8::A, memory);
        let address = 0xFF00 + self.get_r8(&Register8::C, memory) as u16;
        memory.write_byte(address, value);
    }

    pub fn ldh_imm8_a<M: MemoryInterface>(&mut self, memory: &mut M) {
        // Load value in register A into byte at address $FF00 and $FFFF
        let value = self.get_r8(&Register8::A, memory);
        let imm8 = self.fetch_byte(memory);
        let address = imm8 as u16 | 0xFF00;
        memory.write_byte(address, value);
    }

    pub fn ld_imm16_a<M: MemoryInterface>(&mut self, memory: &mut M) {
        // Load value in register A in [imm16]
        let value = self.get_r8(&Register8::A, memory);
        let imm16 = self.fetch_word(memory);
        memory.write_byte(imm16, value);
    }

    pub fn ldh_a_c<M: MemoryInterface>(&mut self, memory: &mut M) {
        // Load value in register A from the byte at address $FF00+C
        let address = 0xFF00 + self.get_r8(&Register8::C, memory) as u16;
        let value = memory.read_byte(address);
        self.set_r8(&Register8::A, value, memory);
    }

    pub fn ldh_a_imm8<M: MemoryInterface>(&mut self, memory: &mut M) {
        // Load value in register A from the byte at address $FF00+imm8
        let imm8 = self.fetch_byte(memory);
        let address = 0xFF00 + imm8 as u16;
        let value = memory.read_byte(address);
        self.set_r8(&Register8::A, value, memory);
    }

    pub fn ld_a_imm16<M: MemoryInterface>(&mut self, memory: &mut M) {
        // Load value in register A from [imm16]
        let imm16 = self.fetch_word(memory);
        let value = memory.read_byte(imm16);
        self.set_r8(&Register8::A, value, memory);
    }

    pub fn add_sp_imm8<M: MemoryInterface>(&mut self, memory: &mut M) {
        // Add signed immediate value to SP
        // Z=0, N=0, H=bit3, C=bit7
        let imm8 = self.fetch_byte(memory) as i8;
        let original_sp = self.sp;
        self.sp = self.sp.wrapping_add(imm8 as u16);
        self.set_zn_flags(1 as u8, false);

        let half_carry = (original_sp & 0xF) + (imm8 as u16 & 0xF) > 0xF;
        let carry = (original_sp & 0xFF) + (imm8 as u16 & 0xFF) > 0xFF;
        self.f.set(Flags::C, carry);
        self.f.set(Flags::H, half_carry);
    }

    pub fn ld_hl_sp_plus_imm8<M: MemoryInterface>(&mut self, memory: &mut M) {
        // Load value in HL from SP + signed immediate value
        let imm8 = self.fetch_byte(memory) as i8;
        let original_sp = self.sp;
        let value = self.sp.wrapping_add(imm8 as u16);
        self.set_r16(&Register16::HL, value);

        let half_carry = (original_sp & 0xF) + (imm8 as u16 & 0xF) > 0xF;
        let carry = (original_sp & 0xFF) + (imm8 as u16 & 0xFF) > 0xFF;
        self.f.set(Flags::C, carry);
        self.f.set(Flags::H, half_carry);
        self.set_zn_flags(1, false);
    }

    pub fn ld_sp_hl<M: MemoryInterface>(&mut self, memory: &mut M) {
        // Load value in SP from HL
        let value = self.get_r16(&Register16::HL);
        self.sp = value;
    }

    pub fn di(&mut self) {
        // Disable IME flag
        self.ime = false;
    }

    pub fn ei(&mut self) {
        // The flag is only set after the instruction following EI
        self.ime_scheduled = true;
    }
}
