use crate::{
    bus::{io_address::IoRegister, Bus},
    cpu::{flags::Flags, CPU},
    debug_window::DebugWindow,
    ppu::PPU,
    timer::Timer,
};
use std::io::{self, Write};
use std::{
    cell::RefCell,
    rc::Rc,
    time::{Duration, Instant},
};

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

    pub fn reset(&mut self) {}

    pub fn tick(&mut self) {
        self.cpu.tick();
        for _ in 0..self.cpu.cycles {
            self.timer.tick();
            self.ppu.tick();
        }
    }

    pub fn run(&mut self) {
        let cycles_per_frame = 70_224;
        let target_frame_time = Duration::from_micros(16_667); // 60 fps
        let mut last_fps_check = Instant::now();
        let mut frames = 0;
        let mut current_fps = 0;
        loop {
            let frame_start_time = Instant::now();
            let mut cycles_this_frame = 0;

            while cycles_this_frame < cycles_per_frame {
                self.tick();
                cycles_this_frame += self.cpu.cycles;
            }
            frames += 1;
            if last_fps_check.elapsed() > Duration::from_secs(1) {
                current_fps = frames;
                frames = 0;
                last_fps_check = Instant::now();
            }
            let frame_time = frame_start_time.elapsed();
            if frame_time < target_frame_time {
                std::thread::sleep(target_frame_time - frame_time);
            }

            if let Some(ref mut debug_window) = self.debug_window {
                debug_window.update(&self.cpu, &self.bus, &self.ppu, current_fps);
                debug_window.render();
            }
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.bus.borrow_mut().load_rom(rom);
    }
    pub fn set_power_up_sequence(&mut self) {
        // Set initial GB state after boot
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
        self.cpu.pc = 0x0100;
        self.cpu.ime = false;

        let mut bus = self.bus.borrow_mut();

        // Hardware Registers
        bus.write_byte(IoRegister::Joyp.address(), 0xCF);
        bus.write_byte(IoRegister::Sb.address(), 0x00);
        bus.write_byte(IoRegister::Sc.address(), 0x7E);
        bus.write_byte(IoRegister::Div.address(), 0xAB);
        bus.write_byte(IoRegister::Tima.address(), 0x00);
        bus.write_byte(IoRegister::Tma.address(), 0x00);
        bus.write_byte(IoRegister::Tac.address(), 0xF8);
        bus.write_byte(IoRegister::If.address(), 0xE1);

        // Sound Registers
        bus.write_byte(IoRegister::Nr10.address(), 0x80);
        bus.write_byte(IoRegister::Nr11.address(), 0xBF);
        bus.write_byte(IoRegister::Nr12.address(), 0xF3);
        bus.write_byte(IoRegister::Nr13.address(), 0xFF);
        bus.write_byte(IoRegister::Nr14.address(), 0xBF);
        bus.write_byte(IoRegister::Nr21.address(), 0x3F);
        bus.write_byte(IoRegister::Nr22.address(), 0x00);
        bus.write_byte(IoRegister::Nr23.address(), 0xFF);
        bus.write_byte(IoRegister::Nr24.address(), 0xBF);
        bus.write_byte(IoRegister::Nr30.address(), 0x7F);
        bus.write_byte(IoRegister::Nr31.address(), 0xFF);
        bus.write_byte(IoRegister::Nr32.address(), 0x9F);
        bus.write_byte(IoRegister::Nr33.address(), 0xFF);
        bus.write_byte(IoRegister::Nr34.address(), 0xBF);
        bus.write_byte(IoRegister::Nr41.address(), 0xFF);
        bus.write_byte(IoRegister::Nr42.address(), 0x00);
        bus.write_byte(IoRegister::Nr43.address(), 0x00);
        bus.write_byte(IoRegister::Nr44.address(), 0xBF);
        bus.write_byte(IoRegister::Nr50.address(), 0x77);
        bus.write_byte(IoRegister::Nr51.address(), 0xF3);
        bus.write_byte(IoRegister::Nr52.address(), 0xF1);

        // LCD Registers
        bus.write_byte(IoRegister::Lcdc.address(), 0x91);
        bus.write_byte(IoRegister::Stat.address(), 0x85);
        bus.write_byte(IoRegister::Scy.address(), 0x00);
        bus.write_byte(IoRegister::Scx.address(), 0x00);
        bus.write_byte(IoRegister::Ly.address(), 0x00);
        bus.write_byte(IoRegister::Lyc.address(), 0x00);
        bus.write_byte(IoRegister::Dma.address(), 0xFF);
        bus.write_byte(IoRegister::Bgp.address(), 0xFC);
        // Note: OBP0 and OBP1 are left uninitialized as per documentation
        bus.write_byte(IoRegister::Wy.address(), 0x00);
        bus.write_byte(IoRegister::Wx.address(), 0x00);

        // Interrupt Enable Register
        bus.write_byte(IoRegister::Ie.address(), 0x00);
    }
}
