use crate::{bus::Bus, cpu::CPU};

impl CPU {
    pub fn fetch_byte(&mut self) -> u8 {
        let byte = self.bus.borrow().read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    pub fn fetch_word(&mut self) -> u16 {
        let word = self.bus.borrow().read_word(self.pc);
        self.pc = self.pc.wrapping_add(2);
        word
    }
}
