use crate::{bus::io_address::IoRegister, cpu::CPU};

impl CPU {
    pub fn handle_interrupts(&mut self) {
        let ie_register = self.bus.borrow().read_byte(IoRegister::Ie.address());
        let if_register = self.bus.borrow().read_byte(IoRegister::If.address());
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
                // VBlank interrupt
                self.service_interrupt(0x40, 0);
            } else if interrupts & 0x02 != 0 {
                // LCDC interrupt
                self.service_interrupt(0x48, 1);
            } else if interrupts & 0x04 != 0 {
                // Timer interrupt
                self.service_interrupt(0x50, 2);
            } else if interrupts & 0x08 != 0 {
                // Serial interrupt
                self.service_interrupt(0x58, 3);
            } else if interrupts & 0x10 != 0 {
                // Joypad interrupt
                self.service_interrupt(0x60, 4);
            }
        }
    }

    fn service_interrupt(&mut self, address: u16, bit: u8) {
        // Disable the IME flag and IF register bit. Then perform call instruction
        self.ime = false;
        self.ime_scheduled = false;
        let if_register = self.bus.borrow().read_byte(IoRegister::If.address());
        let value = if_register & !(1 << bit);
        self.bus
            .borrow_mut()
            .write_byte(IoRegister::If.address(), value);

        self.push_stack(self.pc);
        self.pc = address;

        // 20 t-cycles
        self.cycles += 20;
    }

    fn push_stack(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(2);
        self.bus.borrow_mut().write_word(self.sp, value);
    }
}
