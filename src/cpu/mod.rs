pub mod execute;
pub mod fetch;
pub mod flags;
pub mod instructions;
pub mod interrupts;
pub mod registers;

use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use flags::Flags;

use crate::bus::Bus;
pub use execute::*;

pub struct CPU {
    // Registers
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub f: Flags,
    pub sp: u16,
    pub pc: u16,

    // Flags
    pub ime: bool,
    ime_scheduled: bool,
    halt: bool,
    halt_bug: bool,
    cb_prefix: bool,

    // cycles
    pub cycles: u64,

    // Shared bus
    bus: Rc<RefCell<Bus>>,
}

impl CPU {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
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
            ime: false,
            halt: false,
            halt_bug: false,
            bus,
            cycles: 0,
            cb_prefix: false,
            ime_scheduled: false,
        }
    }

    pub fn tick(&mut self) {
        // pprinit in hex value
        println!("cpu.pc: {:04X}", self.pc);
        if !self.halt {
            // CB opcode is executed as NOP and is saved for the next tick
            self.ime_instruction();
            let opcode = self.fetch_byte();
            self.check_halt_bug();
            self.execute(opcode, self.cb_prefix);
            self.cycles += self.clock_cycles(opcode, self.cb_prefix);
            self.cb_prefix = opcode == 0xCB;
        }
        self.handle_interrupts();

        println!("cpu.pc: {:04X}", self.pc);
    }

    fn ime_instruction(&mut self) {
        if self.ime_scheduled {
            self.ime = true;
            self.ime_scheduled = false;
        }
    }
    fn check_halt_bug(&mut self) {
        if self.halt_bug {
            self.halt_bug = false;
            self.pc = self.pc.wrapping_sub(1);
        }
    }
}
