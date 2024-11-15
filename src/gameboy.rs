use crate::{
    bus::{io_address::IoRegister, Bus},
    cpu::{flags::Flags, CPU},
    debug_window::DebugWindow,
    ppu::PPU,
    timer::Timer,
};
use std::{cell::RefCell, rc::Rc};

pub struct GameBoy {
    pub cpu: CPU,
    pub bus: Rc<RefCell<Bus>>,
    pub timer: Timer,
    pub ppu: PPU,
    pub debug_window: Option<DebugWindow>,
}

impl GameBoy {
    pub fn new(debug: bool) -> Self {
        let bus = Rc::new(RefCell::new(Bus::new()));
        let timer = Timer::new(Rc::clone(&bus));
        let cpu = CPU::new(Rc::clone(&bus));
        let ppu = PPU::new(Rc::clone(&bus));
        let debug_window = if debug {
            Some(DebugWindow::new())
        } else {
            None
        };

        Self {
            cpu,
            timer,
            bus,
            ppu,
            debug_window,
        }
    }
    pub fn set_power_up_sequence(&mut self) {
        // Set initial CPU state to match test expectations
        self.cpu.a = 0x01;
        self.cpu.f.set(Flags::N, false);
        self.cpu.f.set(Flags::H, true);
        self.cpu.f.set(Flags::C, true);
        self.cpu.f.set(Flags::Z, true);
        self.cpu.b = 0x00;
        self.cpu.c = 0x13;
        self.cpu.d = 0x00;
        self.cpu.e = 0xD8;
        self.cpu.h = 0x01;
        self.cpu.l = 0x4D;
        self.cpu.sp = 0xFFFE;
        self.cpu.pc = 0x0100; /*
                              self.bus.borrow_mut().write_byte(IoRegister::Joyp.address(), 0xFF); */
        self.bus.borrow_mut().write_byte(0xFF44, 0x90);
    }

    pub fn reset(&mut self) {}

    pub fn tick(&mut self) {
        self.cpu.tick();
        for _ in 0..self.cpu.cycles {
            /*  self.timer.tick(); */
            self.ppu.tick();
        }
    }

    pub fn run(&mut self) {
        let mut debug_update_counter = 0;
        loop {
            self.tick();

            if let Some(ref mut debug_window) = self.debug_window {
                debug_update_counter += 1;
                if debug_update_counter >= 100 {
                    debug_window.update(&self.cpu, &self.bus);
                    debug_window.render();
                    debug_update_counter = 0;
                }
            }
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.bus.borrow_mut().load_rom(rom);
    }
}
