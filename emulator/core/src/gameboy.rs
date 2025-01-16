use crate::{
    apu::APU,
    bus::{io_address::IoRegister, Bus, BusState, MemoryInterface},
    cpu::{flags::Flags, CPUState, CPU},
    ppu::{self, PPUState, PPU},
    timer::{Timer, TimerState},
};
use bincode;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::{
    rc::Rc,
    time::{Duration, Instant},
};

#[derive(Clone, Serialize, Deserialize)]
struct SerializableGameboy {
    cpu_state: CPUState,
    timer_state: TimerState,
    ppu_state: PPUState,
    bus_data: BusState,
}
#[derive(Clone, Debug)]
pub struct Gameboy {
    pub cpu: CPU,
    pub timer: Timer,
    pub ppu: PPU,
    pub bus: Bus,
    pub apu: APU,
}

impl Gameboy {
    pub fn new(palette: [u32; 4]) -> Self {
        let bus = Bus::new();
        let timer = Timer::new();
        let cpu = CPU::new();
        let ppu = PPU::new(palette);
        let apu = APU::new();

        Self {
            cpu,
            timer,
            bus,
            ppu,
            apu,
        }
    }
    pub fn save_state(&self) -> Result<Vec<u8>, std::io::Error> {
        let serializable_state = SerializableGameboy {
            cpu_state: self.cpu.save_state(),
            timer_state: self.timer.save_state(),
            ppu_state: self.ppu.save_state(),
            bus_data: self.bus.save_state(),
        };

        bincode::serialize(&serializable_state)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Serialization failed"))
    }

    pub fn load_state(&mut self, state: Vec<u8>) -> Result<(), std::io::Error> {
        let serializable_state: SerializableGameboy =
            bincode::deserialize(&state).map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::Other, "Deserialization failed")
            })?;

        self.bus.load_state(serializable_state.bus_data);
        self.cpu.load_state(serializable_state.cpu_state);
        self.timer.load_state(serializable_state.timer_state);
        self.ppu.load_state(serializable_state.ppu_state);

        Ok(())
    }
    pub fn reset(&mut self) {}

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.bus);
        for _ in 0..self.cpu.cycles {
            self.timer.tick(&mut self.bus);
            self.ppu.tick(&mut self.bus);
            self.bus.mbc.tick();
            self.apu.tick(&mut self.bus);
        }
    }
    pub fn run_frame(&mut self) {
        // Run one frame worth of emulation
        let cycles_per_frame = 70224;
        let mut cycles_this_frame = 0;
        while cycles_this_frame < cycles_per_frame {
            self.tick();
            cycles_this_frame += self.cpu.cycles;
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.bus.load_rom(rom);
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

        // Hardware Registers
        self.bus.write_byte(IoRegister::Joyp.address(), 0xCF);
        self.bus.write_byte(IoRegister::Sb.address(), 0x00);
        self.bus.write_byte(IoRegister::Sc.address(), 0x7E);
        self.bus.write_byte(IoRegister::Div.address(), 0xAB);
        self.bus.write_byte(IoRegister::Tima.address(), 0x00);
        self.bus.write_byte(IoRegister::Tma.address(), 0x00);
        self.bus.write_byte(IoRegister::Tac.address(), 0xF8);
        self.bus.write_byte(IoRegister::If.address(), 0xE1);

        // Sound Registers
        self.bus.write_byte(IoRegister::Nr10.address(), 0x80);
        self.bus.write_byte(IoRegister::Nr11.address(), 0xBF);
        self.bus.write_byte(IoRegister::Nr12.address(), 0xF3);
        self.bus.write_byte(IoRegister::Nr13.address(), 0xFF);
        self.bus.write_byte(IoRegister::Nr14.address(), 0xBF);
        self.bus.write_byte(IoRegister::Nr21.address(), 0x3F);
        self.bus.write_byte(IoRegister::Nr22.address(), 0x00);
        self.bus.write_byte(IoRegister::Nr23.address(), 0xFF);
        self.bus.write_byte(IoRegister::Nr24.address(), 0xBF);
        self.bus.write_byte(IoRegister::Nr30.address(), 0x7F);
        self.bus.write_byte(IoRegister::Nr31.address(), 0xFF);
        self.bus.write_byte(IoRegister::Nr32.address(), 0x9F);
        self.bus.write_byte(IoRegister::Nr33.address(), 0xFF);
        self.bus.write_byte(IoRegister::Nr34.address(), 0xBF);
        self.bus.write_byte(IoRegister::Nr41.address(), 0xFF);
        self.bus.write_byte(IoRegister::Nr42.address(), 0x00);
        self.bus.write_byte(IoRegister::Nr43.address(), 0x00);
        self.bus.write_byte(IoRegister::Nr44.address(), 0xBF);
        self.bus.write_byte(IoRegister::Nr50.address(), 0x77);
        self.bus.write_byte(IoRegister::Nr51.address(), 0xF3);
        self.bus.write_byte(IoRegister::Nr52.address(), 0xF1);

        // LCD Registers
        self.bus.write_byte(IoRegister::Lcdc.address(), 0x91);
        self.bus.write_byte(IoRegister::Stat.address(), 0x85);
        self.bus.write_byte(IoRegister::Scy.address(), 0x00);
        self.bus.write_byte(IoRegister::Scx.address(), 0x00);
        self.bus.write_byte(IoRegister::Ly.address(), 0x00);
        self.bus.write_byte(IoRegister::Lyc.address(), 0x00);
        self.bus.write_byte(IoRegister::Dma.address(), 0xFF);
        self.bus.write_byte(IoRegister::Bgp.address(), 0xFC);
        // Note: OBP0 and OBP1 are left uninitialized as per documentation
        self.bus.write_byte(IoRegister::Wy.address(), 0x00);
        self.bus.write_byte(IoRegister::Wx.address(), 0x00);

        // Interrupt Enable Register
        self.bus.write_byte(IoRegister::Ie.address(), 0x00);
    }
}
