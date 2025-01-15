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

use crate::bus::{Bus, MemoryInterface};
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
    pub f: u8,
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
            ime: false,
            halt: false,
            halt_bug: false,
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
            f: self.f.bits(),
            sp: self.sp,
            pc: self.pc,
            ime: self.ime,
            ime_scheduled: self.ime_scheduled,
            halt: self.halt,
            halt_bug: self.halt_bug,
            cycles: self.cycles,
        }
    }
    pub fn load_state(&mut self, state: CPUState) {
        self.a = state.a;
        self.b = state.b;
        self.c = state.c;
        self.d = state.d;
        self.e = state.e;
        self.h = state.h;
        self.l = state.l;
        self.f = Flags::from(state.f);
        self.sp = state.sp;
        self.pc = state.pc;
        self.ime = state.ime;
        self.ime_scheduled = state.ime_scheduled;
        self.halt = state.halt;
        self.halt_bug = state.halt_bug;
        self.cycles = state.cycles;
    }

    pub fn tick<M: MemoryInterface>(&mut self, memory: &mut M) {
        if !self.halt {
            self.ime_instruction();
            let mut opcode = self.fetch_byte(memory);

            self.check_halt_bug();
            if opcode == 0xCB {
                opcode = self.fetch_byte(memory);
                self.execute_cb(opcode, memory);
            } else {
                self.execute(opcode, memory);
            }
            self.cycles = self.get_clock_cycles(opcode, opcode == 0xCB);
        } else {
            self.cycles = 4;
        }
        self.handle_interrupts(memory);
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
