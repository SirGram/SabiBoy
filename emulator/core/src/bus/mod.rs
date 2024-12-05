use io_address::IoRegister;
use serde::{Deserialize, Serialize};

use crate::{
    cartridge::{mbc0::Mbc0, mbc1::Mbc1, mbc3::Mbc3, mbc5::Mbc5, MbcType, MbcTypeState},
    joyp::Joypad,
};

pub mod io_address;
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
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BusState {
    pub joypad: Joypad,

    #[serde(with = "serde_arrays")]
    oam: [u8; 0xA0],

    #[serde(with = "serde_arrays")]
    io_registers: [u8; 0x7F],

    #[serde(with = "serde_arrays")]
    hram: [u8; 0x7F],

    ie_register: u8,

    #[serde(with = "serde_arrays")]
    vram: [u8; 0x2000],

    #[serde(with = "serde_arrays")]
    ram_bank_0: [u8; 0x1000],

    #[serde(with = "serde_arrays")]
    ram_bank_n: [u8; 0x1000],

    #[serde(with = "serde_arrays")]
    debug: [u8; 0x100],

    pub mbc: MbcTypeState,
}

impl Bus {
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
    pub fn save_state(&self) -> BusState {
        BusState {
            joypad: self.joypad.clone(),
            oam: self.oam.clone(),
            io_registers: self.io_registers.clone(),
            hram: self.hram.clone(),
            ie_register: self.ie_register,
            vram: self.vram.clone(),
            ram_bank_0: self.ram_bank_0.clone(),
            ram_bank_n: self.ram_bank_n.clone(),
            debug: self.debug.clone(),
            mbc: self.mbc.save_state(),
        }
    }

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

    pub fn load_rom(&mut self, rom: &[u8]) {
        let ram_size = match rom[0x149] {
            0x00 => 0,       // No RAM
            0x02 => 0x2000,  // 8 KiB
            0x03 => 0x8000,  // 32 KiB
            0x04 => 0x20000, // 128 KiB
            0x05 => 0x10000, // 64 KiB
            _ => 0x2000,
        };
        println!("ram_size: {} bytes", ram_size);
        println!("mbctype: {:04X}", rom[0x147]);
        // Detect MBC type from ROM header
        self.mbc = match rom[0x147] {
            0x00 => MbcType::Mbc0(Mbc0::new(rom, ram_size)),
            0x01..=0x03 => MbcType::Mbc1(Mbc1::new(rom, ram_size)),
            0x0F | 0x10 => MbcType::Mbc3(Mbc3::new(rom, ram_size, true)), // RTC is present
            0x11..=0x13 => MbcType::Mbc3(Mbc3::new(rom, ram_size, false)), // RTC is absent
            0x19..=0x1E => MbcType::Mbc5(Mbc5::new(rom, ram_size)),

            _ => panic!("Unsupported MBC type"),
        };
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.mbc.read_byte(address),
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize],
            0xC000..=0xCFFF => self.ram_bank_0[(address - 0xC000) as usize],
            0xD000..=0xDFFF => self.ram_bank_n[(address - 0xD000) as usize],
            0xE000..=0xFDFF => self.read_byte(address - 0x2000), // Echo RAM: Map E000-FDFF to C000-DDFF
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize],
            0xFEA0..=0xFEFF => self.debug[(address - 0xFEA0) as usize], // not usable TODO: implement https://gbdev.io/pandocs/Memory_Map.html#fea0feff-range
            0xFF00 => self.joypad.read(),
            0xFF01..=0xFF7F => self.io_registers[(address - 0xFF01) as usize],
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => self.ie_register,
            _ => 0xFF,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        // Handle serial output
        if address == 0xFF01 {
            println!("{}", value as char);
        }
        match address {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.mbc.write_byte(address, value),
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize] = value,
            0xC000..=0xCFFF => self.ram_bank_0[(address - 0xC000) as usize] = value,
            0xD000..=0xDFFF => self.ram_bank_n[(address - 0xD000) as usize] = value,
            0xE000..=0xFDFF => self.write_byte(address - 0x2000, value), // Echo RAM: Map E000-FDFF to C000-DDFF
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize] = value,
            0xFEA0..=0xFEFF => self.debug[(address - 0xFEA0) as usize] = value, // Unusable
            0xFF00 => self.joypad.write(value),
            0xFF01..=0xFF45 => self.io_registers[(address - 0xFF01) as usize] = value,
            0xFF46 => self.dma_oam_transfer(value),

            0xFF47..=0xFF7F => self.io_registers[(address - 0xFF01) as usize] = value,
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = value,
            0xFFFF => self.ie_register = value,
            _ => {}
        }
    }

    fn dma_oam_transfer(&mut self, value: u8) {
        // TODO: maybe make cycle accurate
        let source_base = (value as u16) << 8;
        let destination_base = 0xFE00;
        let oam_size = 0xA0;
        for i in 0..oam_size {
            let address = source_base + i;
            let transfer_value = self.read_byte(address);
            self.write_byte(destination_base + i, transfer_value);
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let low = self.read_byte(address);
        let high = self.read_byte(address.wrapping_add(1));
        u16::from_le_bytes([low, high])
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let [low, high] = value.to_le_bytes();
        self.write_byte(address, low);
        self.write_byte(address.wrapping_add(1), high);
    }
    pub fn read_cartridge_header(&self) -> [u8; 0x50] {
        let mut header = [0; 0x50];
        for i in 0..0x50 {
            header[i] = self.read_byte(0x0100 + i as u16);
        }
        header
    }
    pub fn read_wave_ram(&mut self) -> [u8; 16] {
        let mut wave_ram = [0; 16];
        let start_address = IoRegister::WaveRamStart.address();
        for i in 0..16 {
            wave_ram[i] = self.read_byte(start_address + i as u16);
        }
        wave_ram
    }
}
