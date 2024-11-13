use crate::{bus::Bus, cpu::CPU, ppu::PPU, timer::Timer};
use std::{cell::RefCell, rc::Rc};

pub struct GameBoy {
    pub cpu: CPU,
    pub bus: Rc<RefCell<Bus>>,
    pub timer: Timer,
    /*
    pub ppu: PPU, */
}

impl GameBoy {
    pub fn new() -> Self {
        // CPU with reference to shared bus
        let bus = Rc::new(RefCell::new(Bus::new()));
        let timer = Timer::new(Rc::clone(&bus));
        let cpu = CPU::new(Rc::clone(&bus)); /*
                                             let ppu = PPU::new(Rc::clone(&bus)); */

        Self {
            cpu,
            timer,
            bus, /*
                 ppu, */
        }
    }

    pub fn reset(&mut self) {
    }

    pub fn tick(&mut self) {
        self.cpu.tick();
        self.timer.tick(self.cpu.cycles);
        /*
        self.ppu.tick(self.cpu.cycles); */
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
