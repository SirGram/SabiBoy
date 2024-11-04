use bitflags::bitflags;

bitflags! {
    struct Flags: u8 {
        const Z = 0b10000000; //zero
        const N = 0b01000000; //substraction
        const H = 0b00100000; //half carry
        const C = 0b00010000; //carry
        }
}
struct Memory {
    ram: [u8; 65536],
}
enum Register16 {
    BC,
    DE,
    HL,
    SP,
}
enum Register8 {
    b,
    c,
    d,
    e,
    h,
    l,
    hlIndirect,
    a
}
enum Register16Mem {
    BC,
    DE,
    HLi,
    HLd,
}
struct CPU {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: Flags,
    sp: u16,
    pc: u16,
    memory: Memory,
}
impl CPU {
    pub fn new() -> CPU {
        CPU {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            f: Flags::empty(),
            sp: 0xFFFE,
            pc: 0x0100,
            memory: Memory { ram: [0; 65536] },
        }
    }
    pub fn run(&mut self) {
        loop {
            let opcode = self.get_imm8();
            match opcode {
                //opcode file
                0x00 => self.nop(),
                0x01 => self.ld_r16_imm16(Register16::BC),
                0x02 => self.ld_r16mem_a(Register16Mem::BC),
                0x03 => self.inc_r16(Register16::BC),
                0x04 => self.inc_r8(Register8::b),
                0x05 => self.dec_r8(Register8::b),
                0x08 => self.ld_imm16_sp(),
                0x09 => self.add_hl_r16(Register16::BC),
                0x0A => self.ld_a_r16mem(Register16Mem::BC),
                0x0B => self.dec_r16(Register16::BC),
                0x0C => self.inc_r8(Register8::c),

                0x11 => self.ld_r16_imm16(Register16::DE),
                0x12 => self.ld_r16mem_a(Register16Mem::DE),
                0x13 => self.inc_r16(Register16::DE),
                0x14 => self.inc_r8(Register8::d),
                0x15 => self.dec_r8(Register8::d),
                0x19 => self.add_hl_r16(Register16::DE),
                0x1A => self.ld_a_r16mem(Register16Mem::DE),
                0x1B => self.dec_r16(Register16::DE),
                0x1C => self.inc_r8(Register8::e),

                0x21 => self.ld_r16_imm16(Register16::HL),
                0x22 => self.ld_r16mem_a(Register16Mem::HLi),
                0x23 => self.inc_r16(Register16::HL),
                0x24 => self.inc_r8(Register8::h),
                0x25 => self.dec_r8(Register8::h),
                0x29 => self.add_hl_r16(Register16::HL),
                0x2A => self.ld_a_r16mem(Register16Mem::HLi),
                0x2B => self.dec_r16(Register16::HL),
                0x2C => self.inc_r8(Register8::l),

                0x31 => self.ld_r16_imm16(Register16::BC),
                0x32 => self.ld_r16mem_a(Register16Mem::HLd),
                0x33 => self.inc_r16(Register16::SP),
                0x34 => self.inc_r8(Register8::hlIndirect),
                0x35 => self.dec_r8(Register8::hlIndirect),
                0x39 => self.add_hl_r16(Register16::SP),
                0x3A => self.ld_a_r16mem(Register16Mem::HLd),
                0x3B => self.dec_r16(Register16::SP),
                0x3C => self.inc_r8(Register8::a),

                _ => {
                    println!("opcode: {:?}", opcode)
                }
            }
        }
    }
    fn fetch(&mut self) -> u8 {
        return self.memory.ram[self.pc as usize];
    }
    fn fetch16(&mut self) -> u16 {
        let imm16_high = self.memory.ram[self.pc as usize];
        let imm16_low = self.memory.ram[self.pc as usize + 1];
        return (imm16_high as u16) << 8 | imm16_low as u16;
    }
    fn get_imm8(&mut self) -> u8 {
        let imm8 = self.fetch();
        self.pc += 1;
        return imm8;
    }
    fn get_imm16(&mut self) -> u16 {
        let imm16 = self.fetch16();
        self.pc += 2;
        return imm16;
    }
    // reg setters
    fn get_r8(&self, register: &Register8) -> u8 {
        match register {
            Register8::b => self.b,
            Register8::c => self.c,
            Register8::d => self.d,
            Register8::e => self.e,
            Register8::h => self.h,
            Register8::l => self.l,
            Register8::hlIndirect => {
                let hl = ((self.h as u16) << 8) | (self.l as u16);
                self.memory.ram[hl as usize]
            }
            Register8::a => self.a,
        }

    }
    fn set_r8(&mut self, register: &Register8, value: u8) {
        match register {
            Register8::b => self.b = value,
            Register8::c => self.c = value,
            Register8::d => self.d = value,
            Register8::e => self.e = value,
            Register8::h => self.h = value,
            Register8::l => self.l = value,
            Register8::hlIndirect => {
                let hl = ((self.h as u16) << 8) | (self.l as u16);
                self.memory.ram[hl as usize] = value;
            }
            Register8::a => self.a = value,
        }
    }
    fn get_r16mem(&mut self, register: Register16Mem) -> u16 {
        // Get address from register16 memory
        match register {
            Register16Mem::BC => ((self.b as u16) << 8) | (self.c as u16),
            Register16Mem::DE => ((self.d as u16) << 8) | (self.e as u16),
            Register16Mem::HLi => {
                let hl = ((self.h as u16) << 8) | (self.l as u16);
                self.set_r16(&Register16::HL, hl.wrapping_add(1));
                hl
            }
            Register16Mem::HLd => {
                let hl = ((self.h as u16) << 8) | (self.l as u16);
                self.set_r16(&Register16::HL, hl.wrapping_sub(1));
                hl
            }
        }
    }
    fn get_r16(&self, register: &Register16) -> u16 {
        match register {
            Register16::BC => ((self.b as u16) << 8) | self.c as u16,
            Register16::DE => ((self.d as u16) << 8) | self.e as u16,
            Register16::HL => ((self.h as u16) << 8) | self.l as u16,
            Register16::SP => self.sp,
        }
    }

    fn set_r16(&mut self, register: &Register16, value: u16) {
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

    // instruction set
    // block 0
    fn nop(&mut self) {}
    fn ld_r16_imm16(&mut self, register: Register16) {
        // Load 16 bit immediate value into register16
        let imm16 = self.get_imm16();
        match register {
            Register16::BC => {
                self.b = (imm16 >> 8) as u8;
                self.c = (imm16 & 0xFF) as u8;
            }
            Register16::DE => {
                self.d = (imm16 >> 8) as u8;
                self.e = (imm16 & 0xFF) as u8;
            }
            Register16::HL => {
                self.h = (imm16 >> 8) as u8;
                self.l = (imm16 & 0xFF) as u8;
            }
            Register16::SP => {
                self.sp = imm16;
            }
        }
    }

    fn ld_r16mem_a(&mut self, register: Register16Mem) {
        // Load A register into memory location pointed to by register16
        // HL register is incremented or decremented after storing

        let address = self.get_r16mem(register);
        self.memory.ram[address as usize] = self.a;
    }
    fn ld_a_r16mem(&mut self, register: Register16Mem) {
        // A register value is loaded from memory location pointed to by register16
        // HL register is incremented or decremented after storing
        let address = self.get_r16mem(register);
        self.a = self.memory.ram[address as usize];
    }
    fn ld_imm16_sp(&mut self) {
        // Load SP register into [imm16]
        let imm16 = self.get_imm16();
        self.memory.ram[imm16 as usize] = (self.sp & 0xFF) as u8;
        self.memory.ram[(imm16 + 1) as usize] = (self.sp >> 8) as u8;
    }

    fn inc_r16(&mut self, register: Register16) {
        // Increment function for 16-bit registers
        let value = self.get_r16(&register).wrapping_add(1);
        self.set_r16(&register, value);
    }

    fn dec_r16(&mut self, register: Register16) {
        // Decrement function for 16-bit registers
        let value = self.get_r16(&register).wrapping_sub(1);
        self.set_r16(&register, value);
    }

    fn add_hl_r16(&mut self, register: Register16) {
        // Add register16 value to HL
        // N= 0, H IF overflow bit 11, C IF overflow bit 15
        let hl = self.get_r16(&Register16::HL);
        let value = self.get_r16(&register);
        self.set_r16(&Register16::HL, hl.wrapping_add(value));
        self.f.remove(Flags::N);
        let overflow_bit11 = hl & 0xFFF + value & 0xFFF > 0xFFF;
        let overflow_bit15 = (hl & 0xFFF) as u32 + (value & 0xFFF )as u32 > 0xFFFF ;
        if overflow_bit11 {
            self.f.insert(Flags::C);
        } else {
            self.f.remove(Flags::C);
        }
        if overflow_bit15 {
            self.f.insert(Flags::H);
        } else {
            self.f.remove(Flags::H);
        }
    }

    fn inc_r8(&mut self, register: Register8) {
        // Increment function for 8-bit registers
        // Z if result= 0, N=0, H if overflow bit 3
        let value = self.get_r8(&register).wrapping_add(1);
        self.set_r8(&register, value);
        if value == 0 {
            self.f.insert(Flags::Z);
        } else {
            self.f.remove(Flags::Z);
        }
        self.f.remove(Flags::N);
        let overflow_bit3 = value & 0xF == 0;
        if overflow_bit3 {
            self.f.insert(Flags::H);
        }
    }

    fn dec_r8(&mut self, register: Register8) {
        // Decrement function for 8-bit registers
        // Z if result= 0, N=1, H if borrow bit 4
        let value = self.get_r8(&register).wrapping_sub(1);
        self.set_r8(&register, value);
        if value == 0 {
            self.f.insert(Flags::Z);
        } else {
            self.f.remove(Flags::Z);
        }
        self.f.insert(Flags::N);
        let overflow_bit4 = value & 0xF == 0xF;
        if overflow_bit4 {
            self.f.insert(Flags::H);
        }
    }
    // block 1

    // block 2

    // block 3
}

//opcodes
