use serde::{Deserialize, Serialize};

use crate::bus::{io_address::IoRegister,  MemoryInterface};

#[derive(Clone, Debug)]
pub struct Timer {
    div_counter: usize,
    tima_counter: usize,
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

    // Read timer registers from bus
    fn div<M:MemoryInterface>(&self, memory: &M) -> u8 {
        memory.read_byte(IoRegister::Div.address())
    }

    fn tima<M:MemoryInterface>(&self, memory: &M) -> u8 {
        memory.read_byte(IoRegister::Tima.address())
    }

    fn tma<M:MemoryInterface>(&self, memory: &M) -> u8 {
        memory.read_byte(IoRegister::Tma.address())
    }

    fn tac<M:MemoryInterface>(&self, memory: &M) -> u8 {
        memory.read_byte(IoRegister::Tac.address())
    }

    // Write timer registers to bus
    fn set_div<M:MemoryInterface>(&self, value: u8, memory: &mut M) {
        memory
            .write_byte(IoRegister::Div.address(), value);
    }

    fn set_tima<M:MemoryInterface>(&self, value: u8, memory: &mut M) {
        memory
            .write_byte(IoRegister::Tima.address(), value);
    }

    fn request_timer_interrupt<M:MemoryInterface>(&self, memory: &mut M) {
        let if_reg = memory.read_byte(IoRegister::If.address());
        memory.write_byte(IoRegister::If.address(), if_reg | 0b0000_0100);
    }

    fn increment_div<M:MemoryInterface>(&mut self, memory: &mut M) {
        self.div_counter += 1;
        if self.div_counter >= 256 {
            self.div_counter = 0;
            let new_div = self.div(memory).wrapping_add(1);
            self.set_div(new_div, memory);
        }
    }

    fn get_tima_frequency<M:MemoryInterface>(&self, memory: &M) -> usize {
        match self.tac(memory) & 0x03 {
            0 => 1024,
            1 => 16,
            2 => 64,
            3 => 256,
            _ => unreachable!(),
        }
    }

    fn increment_tima<M:MemoryInterface>(&mut self, memory: &mut M) {
        // Check if timer is enabled (TAC bit 2)
        if self.tac(memory) & 0b0000_0100 == 0 {
            return;
        }

        let frequency = self.get_tima_frequency(memory);
        self.tima_counter += 1;

        // Increment TIMA depending on Hz selected
        if self.tima_counter >= frequency {
            self.tima_counter = 0;
            let new_tima = self.tima(memory).wrapping_add(1);

            // Handle TIMA overflow
            if new_tima == 0 {
                self.set_tima(self.tma(memory),memory); // Reset to TMA value
                self.request_timer_interrupt(memory);
            } else {
                self.set_tima(new_tima, memory);
            }
        }
    }

    pub fn tick <M:MemoryInterface>(&mut self, memory: &mut M) {
        self.increment_div( memory);
        self.increment_tima( memory);
    }
}
