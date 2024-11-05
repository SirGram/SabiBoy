use crate::cpu::CPU;

impl CPU {
    pub fn fetch_byte(&mut self) -> u8 {
        let byte = self.memory.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    pub fn fetch_word(&mut self) -> u16 {
        let word = self.memory.read_word(self.pc);
        self.pc = self.pc.wrapping_add(2);
        word
    }
}