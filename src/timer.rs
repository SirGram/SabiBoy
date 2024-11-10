use crate::bus::{io_address::IoRegister, Bus};

use std::{cell::RefCell, rc::Rc};
pub struct Timer {
    div: u8,  // Divider Register
    tima: u8, // Timer Counter
    tma: u8,  // Timer Modulo
    tac: u8,  // Timer Control
    div_counter: u16,
    tima_counter: u16,
    
    bus: Rc<RefCell<Bus>>,
}

impl Timer {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        Self {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            div_counter: 0,
            tima_counter: 0,
            bus,
        }
    }
    fn increment_div(&mut self) {
        self.div_counter = self.div_counter.wrapping_add(1);
        if self.div_counter >= 256 {
            self.div_counter = 0;
            self.div = self.div.wrapping_add(1);
            self.bus
                .borrow_mut()
                .write_byte(IoRegister::Div.address(), self.div);
        }
    }
    fn get_tima_frequency(&self) -> u16 {
        match self.tac & 0x03 {
            0 => 1024,
            1 => 16,
            2 => 64,
            3 => 256,
            _ => unreachable!(),
        }
    }
    fn increment_tima(&mut self) {
        //  Update  if TAC bit 2 is set
        let increment_tima_enabled = self.tac & 0b0000_0100 != 0;
        if !increment_tima_enabled {
            return;
        }
        let frequency = self.get_tima_frequency();
        self.tima_counter = self.tima_counter.wrapping_add(1);

        // Increment TIME depending on frequency
        if self.tima_counter >= frequency {
            self.tima_counter = 0;
            self.tima = self.tima.wrapping_add(1);
            // TIMA  overflow
            if self.tima == 0 {
                self.tima = self.tma;
                // Request interrupt
                let if_register = self.bus.borrow().read_byte(IoRegister::If.address());
                self.bus
                    .borrow_mut()
                    .write_byte(IoRegister::If.address(), if_register | 0b0000_0100);
            }
            self.bus
                .borrow_mut()
                .write_byte(IoRegister::Tima.address(), self.tima);
        }
    }
    pub fn tick(&mut self) {
        // Always increment DIV register
        self.increment_div();
        // Increment TIMA register
        self.increment_tima();
    }
}
