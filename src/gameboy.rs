use crate::{bus::Bus, cpu::CPU, timer::Timer};
use std::{cell::RefCell, rc::Rc};

pub struct GameBoy {
    pub cpu: CPU,
    pub bus: Rc<RefCell<Bus>>,
    pub timer: Timer,
}

impl GameBoy {
    pub fn new() -> Self {
        // CPU with reference to shared bus
        let bus = Rc::new(RefCell::new(Bus::new()));
        let cpu = CPU::new(Rc::clone(&bus));
        let timer =  Timer::new(Rc::clone(&bus));

        Self { cpu, bus, timer }
    }

    pub fn tick(&mut self) {
        self.timer.tick();
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
