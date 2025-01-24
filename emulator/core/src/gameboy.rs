use crate::{
    apu::APU,
    bus::{io_address::IoRegister, Bus, BusState, GameboyMode, MemoryInterface},
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
    bus_data: BusState,
}
#[derive(Clone, Debug)]
pub struct Gameboy {
    pub cpu: CPU,
    pub timer: Timer,
    pub bus: Bus,
    pub apu: APU,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Interrupt {
    VBlank,
    LCDStat,
    Timer,
    Serial,
    Joypad,
}

impl Gameboy {
    pub fn new(palette: [u32; 4]) -> Self {
        let bus = Bus::new(palette, GameboyMode::DMG);
        let timer = Timer::new();
        let cpu = CPU::new();
        let apu = APU::new();

        Self {
            cpu,
            timer,
            bus,
            apu,
        }
    }
    pub fn save_state(&self) -> Result<Vec<u8>, std::io::Error> {
        let serializable_state = SerializableGameboy {
            cpu_state: self.cpu.save_state(),
            timer_state: self.timer.save_state(),
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

        Ok(())
    }
    pub fn reset(&mut self) {
        self.set_power_up_sequence();
    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.bus);
        for _ in 0..self.cpu.cycles {
            let mut interrupts = Vec::new();

            interrupts.extend(self.bus.tick());
            self.timer.tick(&mut self.bus);
            self.apu.tick();

            let mut if_reg = self.bus.read_byte(IoRegister::If.address());
            for interrupt in interrupts {
                match interrupt {
                    Interrupt::VBlank => if_reg |= 0b0000_0001,
                    Interrupt::LCDStat => if_reg |= 0b0000_0010,
                    Interrupt::Timer => if_reg |= 0b0000_0100,
                    Interrupt::Serial => if_reg |= 0b0000_1000,
                    Interrupt::Joypad => if_reg |= 0b0001_0000,
                }
            }
            self.bus.write_byte(IoRegister::If.address(), if_reg);
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
        self.bus.check__gb_mode(rom[0x143]);
        self.set_power_up_sequence();
        self.bus.load_rom(rom);
    }
    pub fn set_power_up_sequence(&mut self) {
        match self.bus.gb_mode() {
            GameboyMode::DMG => self.set_power_up_sequence_dmg(),
            GameboyMode::CGB => self.set_power_up_sequence_cgb(),
        }
    }
    fn set_power_up_sequence_dmg(&mut self) {
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
    fn set_power_up_sequence_cgb(&mut self) {
        // Set initial GB state after boot
        self.cpu.a = 0x11;
        self.cpu.f.set(Flags::N, true);
        self.cpu.f.set(Flags::H, false);
        self.cpu.f.set(Flags::C, false);
        self.cpu.f.set(Flags::Z, false);
        self.cpu.b = 0x00;
        self.cpu.c = 0x00;
        self.cpu.d = 0xFF;
        self.cpu.e = 0x56;
        self.cpu.h = 0x00;
        self.cpu.l = 0x0D;
        self.cpu.sp = 0xFFFE;
        self.cpu.pc = 0x0100;
        self.cpu.ime = false;

        // Hardware Registers
        self.bus.write_byte(IoRegister::Joyp.address(), 0xCF);
        self.bus.write_byte(IoRegister::Sb.address(), 0x00);
        self.bus.write_byte(IoRegister::Sc.address(), 0x7F);
        self.bus.write_byte(IoRegister::Div.address(), 0xAB); // ??
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
        self.bus.write_byte(IoRegister::Stat.address(), 0x85); //??
        self.bus.write_byte(IoRegister::Scy.address(), 0x00);
        self.bus.write_byte(IoRegister::Scx.address(), 0x00);
        self.bus.write_byte(IoRegister::Ly.address(), 0x00); //??
        self.bus.write_byte(IoRegister::Lyc.address(), 0x00);
        self.bus.write_byte(IoRegister::Dma.address(), 0xFF);
        self.bus.write_byte(IoRegister::Bgp.address(), 0xFC);
        // Note: OBP0 and OBP1 are left uninitialized as per documentation
        self.bus.write_byte(IoRegister::Wy.address(), 0x00);
        self.bus.write_byte(IoRegister::Wx.address(), 0x00);

        // Interrupt Enable Register
        self.bus.write_byte(IoRegister::Ie.address(), 0x00);

        // Modified CGB Register initialization
        self.bus.write_byte(IoRegister::Key1.address(), 0x7E); // Speed switch register
        self.bus.write_byte(IoRegister::Vbk.address(), 0x00); // Start with VRAM bank 0

        // Initialize HDMA registers to 0xFF (disabled state)
        self.bus.write_byte(IoRegister::Hdma1.address(), 0xFF);
        self.bus.write_byte(IoRegister::Hdma2.address(), 0xFF);
        self.bus.write_byte(IoRegister::Hdma3.address(), 0xFF);
        self.bus.write_byte(IoRegister::Hdma4.address(), 0xFF);
        self.bus.write_byte(IoRegister::Hdma5.address(), 0xFF);

        // Initialize palette registers
        self.bus.write_byte(IoRegister::Bcps.address(), 0x00); // Background palette index
        self.bus.write_byte(IoRegister::Bcpd.address(), 0xFF); // Initialize with white
        self.bus.write_byte(IoRegister::Ocps.address(), 0x00); // Object palette index
        self.bus.write_byte(IoRegister::Ocpd.address(), 0xFF); // Initialize with white

        // WRAM bank register (SVBK)
        self.bus.write_byte(IoRegister::Svbk.address(), 0x01); // Start with WRAM bank 1

        // Initialize both background and object palettes to white
        for i in 0..64 {
            self.bus.cgb.bg_palette_ram[i] = 0xFF;
            self.bus.cgb.obj_palette_ram[i] = 0xFF;
        }

        // Make sure CGB-specific variables are properly initialized
        self.bus.cgb.vram_bank = 0;
        self.bus.cgb.wram_bank = 1;
        self.bus.cgb.bg_palette_index = 0;
        self.bus.cgb.obj_palette_index = 0;
        self.bus.cgb.dma_active = false;
        self.bus.cgb.hdma_active = false;
    }
}
