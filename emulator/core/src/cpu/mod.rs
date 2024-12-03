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
use serde::{Deserialize, Serialize};

use crate::bus::Bus;
pub use execute::*;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CPUState {
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
    pub ime: bool,
    ime_scheduled: bool,
    pub halt: bool,
    halt_bug: bool,
    pub cycles: usize,
}
#[derive(Clone, Debug)]
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
    pub halt: bool,
    halt_bug: bool,

    // cycles
    pub cycles: usize,

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
            ime_scheduled: false,
        }
    }
    pub fn save_state(&self) -> CPUState {
        CPUState {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            e: self.e,
            h: self.h,
            l: self.l,
            f: self.f.clone(),
            sp: self.sp,
            pc: self.pc,
            ime: self.ime,
            ime_scheduled: self.ime_scheduled,
            halt: self.halt,
            halt_bug: self.halt_bug,
            cycles: self.cycles,
        }
    }
    pub fn load_state(&mut self, state: CPUState, bus: Rc<RefCell<Bus>>) {
        self.a = state.a;
        self.b = state.b;
        self.c = state.c;
        self.d = state.d;
        self.e = state.e;
        self.h = state.h;
        self.l = state.l;
        self.f = state.f;
        self.sp = state.sp;
        self.pc = state.pc;
        self.ime = state.ime;
        self.ime_scheduled = state.ime_scheduled;
        self.halt = state.halt;
        self.halt_bug = state.halt_bug;
        self.cycles = state.cycles;
        self.bus = bus;
    }

    pub fn tick(&mut self) {
        if !self.halt {
            self.ime_instruction();
            let mut opcode = self.fetch_byte();

            self.check_halt_bug();
            if opcode == 0xCB {
                /*  print!("CB OPCODE: {} ", opcode); */
                opcode = self.fetch_byte();
                self.execute_cb(opcode);
            } else {
                self.execute(opcode);
            }
            self.cycles = self.get_clock_cycles(opcode, opcode == 0xCB);
            /*  println!("OPCODE: {:02X} CYCLES: {}", opcode, self.cycles); */
        } else {
            // 4 t-cycles when halted
            self.cycles += 4;
        }
        self.handle_interrupts();
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
