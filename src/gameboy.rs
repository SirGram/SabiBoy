use crate::{bus::Bus, cpu::CPU};
use std::{cell::RefCell, rc::Rc};

pub struct GameBoy {
    cpu: CPU,
    bus: Rc<RefCell<Bus>>,
}

impl GameBoy {
    pub fn new() -> Self {
        // CPU with reference to shared bus
        let bus = Rc::new(RefCell::new(Bus::new()));
        let cpu = CPU::new(Rc::clone(&bus));

        Self { cpu, bus }
    }

    pub fn tick(&mut self) {
        self.cpu.tick();
    }

    pub fn run(&mut self) {
        loop {
            self.tick();
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.bus.borrow_mut().load_rom(rom);
    }
}
