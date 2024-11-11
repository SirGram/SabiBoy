use crate::{bus::Bus, cpu::CPU, ppu::PPU, timer::Timer};
use std::{cell::RefCell, rc::Rc};

pub struct GameBoy {
    pub cpu: CPU,
    pub bus: Rc<RefCell<Bus>>,
    pub timer: Timer,
    pub ppu: PPU,
}

impl GameBoy {
    pub fn new() -> Self {
        // CPU with reference to shared bus
        let bus = Rc::new(RefCell::new(Bus::new()));
        let cpu = CPU::new(Rc::clone(&bus));
        let timer = Timer::new(Rc::clone(&bus));
        let ppu = PPU::new(Rc::clone(&bus));

        Self {
            cpu,
            bus,
            timer,
            ppu,
        }
    }

    pub fn tick(&mut self) {
        self.timer.tick();
        self.cpu.tick();
        self.ppu.tick();
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
