use crate::{
    bus::{io_address::IoRegister, MemoryInterface},
    cpu::CPU,
};

impl CPU {
    pub fn handle_interrupts<M: MemoryInterface>(&mut self, memory: &mut M) {
        let ie_register = memory.read_byte(IoRegister::Ie.address());
        let if_register = memory.read_byte(IoRegister::If.address());
        // Interrupts enabled and requested
        let interrupts = ie_register & if_register;
        /*
        0000 0001 - V-Blank
        0000 0010 - LCDC
        0000 0100 - Timer
        0000 1000 - Serial
        0001 0000 - Joypad
        */
        /* println!("ie: {:02X} if: {:02X}", ie_register, if_register); */

        if interrupts != 0 {
            self.halt = false;
        }
        if !self.ime {
            return;
        }

        if interrupts != 0 {
            if interrupts & 0x01 != 0 {
                self.service_interrupt(0x40, 0, memory);
            } else if interrupts & 0x02 != 0 {
                self.service_interrupt(0x48, 1, memory);
            } else if interrupts & 0x04 != 0 {
                self.service_interrupt(0x50, 2, memory);
            } else if interrupts & 0x08 != 0 {
                self.service_interrupt(0x58, 3, memory);
            } else if interrupts & 0x10 != 0 {
                self.service_interrupt(0x60, 4, memory);
            }
        }
    }

    fn service_interrupt<M: MemoryInterface>(&mut self, address: u16, bit: u8, memory: &mut M) {
        // Disable the IME flag and IF register bit. Then perform call instruction
        self.ime = false;
        self.ime_scheduled = false;
        let if_register = memory.read_byte(IoRegister::If.address());
        let value = if_register & !(1 << bit);
        memory.write_byte(IoRegister::If.address(), value);

        self.push_stack(self.pc, memory);
        self.pc = address;

        // 20 t-cycles
        self.cycles += 20;
    }

    fn push_stack<M: MemoryInterface>(&mut self, value: u16, memory: &mut M) {
        self.sp = self.sp.wrapping_sub(2);
        memory.write_word(self.sp, value);
    }
}
