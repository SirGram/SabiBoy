use crate::gameboy::Interrupt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Timer {
    div_counter: usize,
    tima_counter: usize,
    pub rgs: TimerRegisters,
}

#[derive(Clone, Debug)]
pub struct TimerRegisters {
    pub div: u8,
    pub tima: u8,
    pub tma: u8,
    pub tac: u8,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TimerState {
    pub div_counter: usize,
    pub tima_counter: usize,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div_counter: 0,
            tima_counter: 0,
            rgs: TimerRegisters {
                div: 0,
                tima: 0,
                tma: 0,
                tac: 0,
            },
        }
    }
    pub fn read_timer_register(&self, address: u16) -> u8 {
        match address {
            0xFF04 => self.rgs.div,
            0xFF05 => self.rgs.tima,
            0xFF06 => self.rgs.tma,
            0xFF07 => self.rgs.tac,
            _ => 0xFF,
        }
    }
    pub fn write_timer_register(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.rgs.div = value,
            0xFF05 => self.rgs.tima = value,
            0xFF06 => self.rgs.tma = value,
            0xFF07 => self.rgs.tac = value,
            _ => {}
        }
    }

    pub fn save_state(&self) -> TimerState {
        TimerState {
            div_counter: self.div_counter,
            tima_counter: self.tima_counter,
        }
    }

    pub fn load_state(&mut self, state: TimerState) {
        self.div_counter = state.div_counter;
        self.tima_counter = state.tima_counter;
    }

    fn increment_div(&mut self) {
        self.div_counter += 1;
        if self.div_counter >= 256 {
            self.div_counter = 0;
            self.rgs.div = self.rgs.div.wrapping_add(1);
        }
    }

    fn get_tima_frequency(&self) -> usize {
        match self.rgs.tac & 0x03 {
            0 => 1024,
            1 => 16,
            2 => 64,
            3 => 256,
            _ => unreachable!(),
        }
    }

    fn increment_tima(&mut self) -> bool {
        // Check if timer is enabled (TAC bit 2)
        if (self.rgs.tac & 0b0000_0100) == 0 {
            return false;
        }

        let frequency = self.get_tima_frequency();
        self.tima_counter += 1;

        if self.tima_counter >= frequency {
            self.tima_counter = 0;
            self.rgs.tima = self.rgs.tima.wrapping_add(1);

            // Handle TIMA overflow
            if self.rgs.tima == 0 {
                self.rgs.tima = self.rgs.tma;
                return true;
            }
        }
        false
    }

    pub fn tick(&mut self) -> u8 {
        let mut interrupts = 0;

        self.increment_div();

        if self.increment_tima() {
            interrupts |= 0b0000_0100;
        }

        interrupts
    }
}
