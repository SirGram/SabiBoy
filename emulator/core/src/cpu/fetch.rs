use crate::{bus::{Bus, MemoryInterface}, cpu::CPU};

impl CPU {
    pub fn fetch_byte<M: MemoryInterface>(&mut self, memory: &M) -> u8 {
        let byte = memory.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    pub fn fetch_word<M: MemoryInterface>(&mut self, memory: &M) -> u16 {
        let word = memory.read_word(self.pc);
        self.pc = self.pc.wrapping_add(2);
        word
    }
}
