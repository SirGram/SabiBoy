pub mod execute;
pub mod fetch;
pub mod flags;
pub mod instructions;
pub mod registers;

use flags::Flags;

use crate::memory::Memory;
pub use execute::*;

pub struct CPU {
    // Registers
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

    // Memory
    memory: Memory,
}

impl CPU {
    pub fn new() -> Self {
        Self {
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
            memory: Memory::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.fetch_byte();
            self.execute(opcode);
        }
    }
}
