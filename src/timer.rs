use crate::bus::{io_address::IoRegister, Bus};
use std::{cell::RefCell, rc::Rc};

pub struct Timer {
    bus: Rc<RefCell<Bus>>,
    div_counter: u16,
    tima_counter: u16,
}

impl Timer {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        Self {
            bus,
            div_counter: 0,
            tima_counter: 0,
        }
    }

    // Read timer registers from bus
    fn div(&self) -> u8 {
        self.bus.borrow().read_byte(IoRegister::Div.address())
    }

    fn tima(&self) -> u8 {
        self.bus.borrow().read_byte(IoRegister::Tima.address())
    }

    fn tma(&self) -> u8 {
        self.bus.borrow().read_byte(IoRegister::Tma.address())
    }

    fn tac(&self) -> u8 {
        self.bus.borrow().read_byte(IoRegister::Tac.address())
    }

    // Write timer registers to bus
    fn set_div(&self, value: u8) {
        self.bus
            .borrow_mut()
            .write_byte(IoRegister::Div.address(), value);
    }

    fn set_tima(&self, value: u8) {
        self.bus
            .borrow_mut()
            .write_byte(IoRegister::Tima.address(), value);
    }

    fn request_timer_interrupt(&self) {
        let mut bus = self.bus.borrow_mut();
        let if_reg = bus.read_byte(IoRegister::If.address());
        bus.write_byte(IoRegister::If.address(), if_reg | 0b0000_0100);
    }

    fn increment_div(&mut self) {
        self.div_counter = self.div_counter.wrapping_add(1);
        if self.div_counter >= 256 {
            self.div_counter = 0;
            let new_div = self.div().wrapping_add(1);
            self.set_div(new_div);
        }
    }

    fn get_tima_frequency(&self) -> u16 {
        match self.tac() & 0x03 {
            0 => 1024,
            1 => 16,
            2 => 64,
            3 => 256,
            _ => unreachable!(),
        }
    }

    fn increment_tima(&mut self) {
        // Check if timer is enabled (TAC bit 2)
        if self.tac() & 0b0000_0100 == 0 {
            return;
        }

        let frequency = self.get_tima_frequency();
        self.tima_counter = self.tima_counter.wrapping_add(1);

        // Increment TIMA depending on Hz selected
        if self.tima_counter >= frequency {
            self.tima_counter = 0;
            let new_tima = self.tima().wrapping_add(1);

            // Handle TIMA overflow
            if new_tima == 0 {
                self.set_tima(self.tma()); // Reset to TMA value
                self.request_timer_interrupt();
            } else {
                self.set_tima(new_tima);
            }
        }
    }

    pub fn tick(&mut self) {
        self.increment_div();
        self.increment_tima();
    }
}
