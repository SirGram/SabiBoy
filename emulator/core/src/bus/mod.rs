use io_address::IoRegister;
use serde::{Deserialize, Serialize};

use crate::{
    cartridge::{mbc0::Mbc0, mbc1::Mbc1, mbc3::Mbc3, mbc5::Mbc5, MbcType, MbcTypeState},
    joyp::Joypad,
};

pub mod io_address;

pub trait MemoryInterface {
    fn read_byte(&self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, value: u8);
    
    // Default implementations for common operations
    #[inline(always)]
    fn read_word(&self, addr: u16) -> u16 {
        let low = self.read_byte(addr);
        let high = self.read_byte(addr.wrapping_add(1));
        u16::from_le_bytes([low, high])
    }

    #[inline(always)]
    fn write_word(&mut self, addr: u16, value: u16) {
        let [low, high] = value.to_le_bytes();
        self.write_byte(addr, low);
        self.write_byte(addr.wrapping_add(1), high);
    }

    // Add direct memory access methods
    #[inline(always)]
    fn get_vram(&self) -> Option<&[u8]> { None }
    
    #[inline(always)]
    fn get_oam(&self) -> Option<&[u8]> { None }
    
    #[inline(always)]
    fn read_wave_ram(&self) -> [u8; 16];
}

#[derive(Clone, Debug)]
pub struct Bus {
    pub joypad: Joypad,
    oam: [u8; 0xA0],
    io_registers: [u8; 0x7F],
    hram: [u8; 0x7F],
    ie_register: u8,
    vram: [u8; 0x2000],
    ram_bank_0: [u8; 0x1000],
    ram_bank_n: [u8; 0x1000],
    debug: [u8; 0x100],
    pub mbc: MbcType,
}

impl MemoryInterface for Bus {
    #[inline(always)]
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.mbc.read_byte(address),
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize],
            0xC000..=0xCFFF => self.ram_bank_0[(address - 0xC000) as usize],
            0xD000..=0xDFFF => self.ram_bank_n[(address - 0xD000) as usize],
            0xE000..=0xFDFF => self.read_byte(address - 0x2000), // Echo RAM
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize],
            0xFEA0..=0xFEFF => self.debug[(address - 0xFEA0) as usize],
            0xFF00 => self.joypad.read(),
            0xFF01..=0xFF7F => self.io_registers[(address - 0xFF01) as usize],
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => self.ie_register,
            _ => 0xFF,
        }
    }

    #[inline(always)]
    fn write_byte(&mut self, address: u16, value: u8) {
        if address == 0xFF01 {
            println!("{}", value as char);
        }
        
        match address {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.mbc.write_byte(address, value),
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize] = value,
            0xC000..=0xCFFF => self.ram_bank_0[(address - 0xC000) as usize] = value,
            0xD000..=0xDFFF => self.ram_bank_n[(address - 0xD000) as usize] = value,
            0xE000..=0xFDFF => self.write_byte(address - 0x2000, value),
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize] = value,
            0xFEA0..=0xFEFF => self.debug[(address - 0xFEA0) as usize] = value,
            0xFF00 => self.joypad.write(value),
            0xFF01..=0xFF45 => self.io_registers[(address - 0xFF01) as usize] = value,
            0xFF46 => self.dma_oam_transfer(value),
            0xFF47..=0xFF7F => self.io_registers[(address - 0xFF01) as usize] = value,
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = value,
            0xFFFF => self.ie_register = value,
            _ => {}
        }
    }

    // Provide direct memory access
    #[inline(always)]
    fn get_vram(&self) -> Option<&[u8]> {
        Some(&self.vram)
    }

    #[inline(always)]
    fn get_oam(&self) -> Option<&[u8]> {
        Some(&self.oam)
    }

    #[inline(always)]
    fn read_wave_ram(&self) -> [u8; 16] {
        let mut wave_ram = [0; 16];
        let start_address = IoRegister::WaveRamStart.address();
        for i in 0..16 {
            wave_ram[i] = self.read_byte(start_address + i as u16);
        }
        wave_ram
    }
}

impl Bus {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            vram: [0; 0x2000],
            ram_bank_0: [0; 0x1000],
            ram_bank_n: [0; 0x1000],
            oam: [0; 0xA0],
            joypad: Joypad::new(),
            io_registers: [0; 0x7F],
            hram: [0; 0x7F],
            ie_register: 0,
            debug: [0; 0x100],
            mbc: MbcType::None,
        }
    }

    #[inline]
    pub fn save_state(&self) -> BusState {
        BusState {
            joypad: self.joypad.clone(),
            oam: self.oam,
            io_registers: self.io_registers,
            hram: self.hram,
            ie_register: self.ie_register,
            vram: self.vram,
            ram_bank_0: self.ram_bank_0,
            ram_bank_n: self.ram_bank_n,
            debug: self.debug,
            mbc: self.mbc.save_state(),
        }
    }

    #[inline]
    pub fn load_state(&mut self, state: BusState) {
        self.joypad = state.joypad;
        self.oam = state.oam;
        self.io_registers = state.io_registers;
        self.hram = state.hram;
        self.ie_register = state.ie_register;
        self.vram = state.vram;
        self.ram_bank_0 = state.ram_bank_0;
        self.ram_bank_n = state.ram_bank_n;
        self.debug = state.debug;
        self.mbc.load_state(state.mbc);
    }

    #[inline]
    pub fn load_rom(&mut self, rom: &[u8]) {
        let ram_size = match rom[0x149] {
            0x00 => 0,       // No RAM
            0x02 => 0x2000,  // 8 KiB
            0x03 => 0x8000,  // 32 KiB
            0x04 => 0x20000, // 128 KiB
            0x05 => 0x10000, // 64 KiB
            _ => 0x2000,
        };
        
        self.mbc = match rom[0x147] {
            0x00 => MbcType::Mbc0(Mbc0::new(rom, ram_size)),
            0x01..=0x03 => MbcType::Mbc1(Mbc1::new(rom, ram_size)),
            0x0F | 0x10 => MbcType::Mbc3(Mbc3::new(rom, ram_size, true)),
            0x11..=0x13 => MbcType::Mbc3(Mbc3::new(rom, ram_size, false)),
            0x19..=0x1E => MbcType::Mbc5(Mbc5::new(rom, ram_size)),
            _ => panic!("Unsupported MBC type"),
        };
    }

    #[inline]
    fn dma_oam_transfer(&mut self, value: u8) {
        let source_base = (value as u16) << 8;
        for i in 0..0xA0 {
            let source_addr = source_base + i;
            let value = self.read_byte(source_addr);
            self.oam[i as usize] = value;
        }
    }
    
    #[inline(always)]
    pub fn read_cartridge_header(&self) -> [u8; 0x50]{
        let mut header = [0; 0x50];
        for i in 0..0x50 {
            header[i] = self.read_byte(i as u16);
        }
        header
    }
}

// Keep the existing BusState definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BusState {
    pub joypad: Joypad,
    #[serde(with = "serde_arrays")]
    pub oam: [u8; 0xA0],
    #[serde(with = "serde_arrays")]
    pub io_registers: [u8; 0x7F],
    #[serde(with = "serde_arrays")]
    pub hram: [u8; 0x7F],
    pub ie_register: u8,
    #[serde(with = "serde_arrays")]
    pub vram: [u8; 0x2000],
    #[serde(with = "serde_arrays")]
    pub ram_bank_0: [u8; 0x1000],
    #[serde(with = "serde_arrays")]
    pub ram_bank_n: [u8; 0x1000],
    #[serde(with = "serde_arrays")]
    pub debug: [u8; 0x100],
    pub mbc: MbcTypeState,
}