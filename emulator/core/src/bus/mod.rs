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

    #[inline(always)]
    fn read_wave_ram(&self) -> [u8; 16];

    fn gb_mode(&self) -> GameboyMode;
}

#[derive(Clone, Debug)]
pub struct Bus {
    pub joypad: Joypad,
    oam: [u8; 0xA0],
    io_registers: [u8; 0x7F],
    hram: [u8; 0x7F],
    ie_register: u8,
    vram_banks: Vec<[u8; 0x2000]>,
    wram_banks: Vec<[u8; 0x1000]>,
    current_wram_bank: usize,
    debug: [u8; 0x100],
    pub mbc: MbcType,
    pub gb_mode: GameboyMode,

    // CGB-specific
    pub bg_palette_ram: [u8; 64],
    pub obj_palette_ram: [u8; 64],
    bg_palette_index: u8,
    obj_palette_index: u8,
    hdma_active: bool,
    hdma_length: u16,
    hdma_source: u16,
    hdma_dest: u16,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Copy)]
pub enum GameboyMode {
    DMG,
    CGB,
}

impl MemoryInterface for Bus {
    #[inline(always)]
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.mbc.read_byte(address),
            0x8000..=0x9FFF => self.read_vram(address - 0x8000),
            0xC000..=0xCFFF => self.wram_banks[0][(address - 0xC000) as usize],
            0xD000..=0xDFFF => self.wram_banks[self.current_wram_bank][(address - 0xD000) as usize],
            0xE000..=0xFDFF => self.read_byte(address - 0x2000), // Echo RAM
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize],
            0xFEA0..=0xFEFF => self.debug[(address - 0xFEA0) as usize],
            0xFF00 => self.joypad.read(),
            0xFF68..=0xFF6B if self.gb_mode == GameboyMode::CGB => self.read_cgb_registers(address),
            0xFF01..=0xFF7F => self.io_registers[(address - 0xFF01) as usize],
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => self.ie_register,
            _ => 0xFF,
        }
    }

    #[inline(always)]
    fn write_byte(&mut self, address: u16, value: u8) {
        /* if  address == 0xFF02 || address == 0xFF01 {
            print!("{} {}", value, value as char); // debug serial
        } */

        match address {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.mbc.write_byte(address, value),
            0x8000..=0x9FFF => self.write_vram(address - 0x8000, value),
            0xC000..=0xCFFF => self.wram_banks[0][(address - 0xC000) as usize] = value,
            0xD000..=0xDFFF => {
                self.wram_banks[self.current_wram_bank][(address - 0xD000) as usize] = value
            }
            0xE000..=0xFDFF => self.write_byte(address - 0x2000, value),
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize] = value,
            0xFEA0..=0xFEFF => self.debug[(address - 0xFEA0) as usize] = value,
            0xFF00 => self.joypad.write(value),
            0xFF68..=0xFF6B if self.gb_mode == GameboyMode::CGB => {
                self.write_cgb_registers(address, value)
            }

            0xFF01..=0xFF45 => self.io_registers[(address - 0xFF01) as usize] = value,
            0xFF46 => self.dma_oam_transfer(value),
            0xFF47..=0xFF7F => self.io_registers[(address - 0xFF01) as usize] = value,
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = value,
            0xFFFF => self.ie_register = value,
            _ => {}
        }
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

    fn gb_mode(&self) -> GameboyMode {
        self.gb_mode
    }
}

impl Bus {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            vram_banks: vec![[0; 0x2000]],
            wram_banks: vec![[0; 0x1000], [0; 0x1000]],
            current_wram_bank: 1,
            oam: [0; 0xA0],
            joypad: Joypad::new(),
            io_registers: [0; 0x7F],
            hram: [0; 0x7F],
            ie_register: 0,
            debug: [0; 0x100],
            mbc: MbcType::None,
            gb_mode: GameboyMode::DMG,
            bg_palette_ram: [0xFF; 64], 
            obj_palette_ram: [0xFF; 64],
            bg_palette_index: 0,
            obj_palette_index: 0,
            hdma_active: false,
            hdma_length: 0,
            hdma_source: 0,
            hdma_dest: 0,
        }
    }

    #[inline]
    pub fn save_state(&self) -> BusState {
        let mut vram_data = Vec::with_capacity(self.vram_banks.len() * 0x2000);
        for bank in &self.vram_banks {
            vram_data.extend_from_slice(bank);
        }

        let mut wram_data = Vec::with_capacity(self.wram_banks.len() * 0x1000);
        for bank in &self.wram_banks {
            wram_data.extend_from_slice(bank);
        }

        BusState {
            joypad: self.joypad.clone(),
            oam: self.oam,
            io_registers: self.io_registers,
            hram: self.hram,
            ie_register: self.ie_register,
            vram_data,
            wram_data,
            current_wram_bank: self.current_wram_bank,
            debug: self.debug,
            mbc: self.mbc.save_state(),
            gb_mode: self.gb_mode.clone(),
        }
    }

    #[inline]
    pub fn load_state(&mut self, state: BusState) {
        self.joypad = state.joypad;
        self.oam = state.oam;
        self.io_registers = state.io_registers;
        self.hram = state.hram;
        self.ie_register = state.ie_register;

        // Reconstruct VRAM banks
        let vram_banks_count = state.vram_data.len() / 0x2000;
        self.vram_banks.clear();
        for bank_idx in 0..vram_banks_count {
            let start = bank_idx * 0x2000;
            let end = start + 0x2000;
            let mut bank = [0u8; 0x2000];
            bank.copy_from_slice(&state.vram_data[start..end]);
            self.vram_banks.push(bank);
        }

        // Reconstruct WRAM banks
        let wram_banks_count = state.wram_data.len() / 0x1000;
        self.wram_banks.clear();
        for bank_idx in 0..wram_banks_count {
            let start = bank_idx * 0x1000;
            let end = start + 0x1000;
            let mut bank = [0u8; 0x1000];
            bank.copy_from_slice(&state.wram_data[start..end]);
            self.wram_banks.push(bank);
        }

        self.current_wram_bank = state.current_wram_bank;
        self.debug = state.debug;
        self.mbc.load_state(state.mbc);
        self.gb_mode = state.gb_mode;
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
        println!("ram_size: {} bytes", ram_size);
        println!("mbctype: {:04X}", rom[0x147]);
        let gb_mode = match rom[0x143] {
            0xC0 => GameboyMode::CGB,
            _ => GameboyMode::DMG,
        };
        self.gb_mode = gb_mode;
        println!("gb_mode: {:?}", self.gb_mode);
        if self.gb_mode != GameboyMode::DMG {
            self.vram_banks = vec![[0; 0x2000], [0; 0x2000]];
            self.wram_banks = vec![[0; 0x1000]; 8];
            self.current_wram_bank = 1;
        }

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
    pub fn read_cartridge_header(&self) -> [u8; 0x50] {
        let mut header = [0; 0x50];
        for i in 0..0x50 {
            header[i] = self.read_byte(i as u16);
        }
        header
    }

    #[inline(always)]
    fn read_vram(&self, address: u16) -> u8 {
        let bank = if self.gb_mode != GameboyMode::DMG {
            (self.io_registers[(IoRegister::Vbk.address() - 0xFF01) as usize] & 0x1) as usize
        } else {
            0
        };
        self.vram_banks[bank][address as usize]
    }

    #[inline(always)]
    fn write_vram(&mut self, address: u16, value: u8) {
        let bank = if self.gb_mode != GameboyMode::DMG {
            (self.io_registers[(IoRegister::Vbk.address() - 0xFF01) as usize] & 0x1) as usize
        } else {
            0
        };
        self.vram_banks[bank][address as usize] = value;
    }
    #[inline(always)]
    fn write_cgb_registers(&mut self, address: u16, value: u8) {
        match address {
            0xFF68 => {
                println!("Writing BCPS: {:02X}", value);
                self.bg_palette_index = value;
            }
            0xFF69 => {
                println!("Writing BCPD[{}]: {:02X}", self.bg_palette_index & 0x3F, value);
                let index = (self.bg_palette_index & 0b111111) as usize;
                self.bg_palette_ram[index] = value;
                // auto-increment enabled
                if self.bg_palette_index & 0b10000000 != 0 {
                    self.bg_palette_index = (self.bg_palette_index + 1) & 0x3F;
                };
            }
            0xFF6A => self.obj_palette_index = value,
            0xFF6B => {
                let index = (self.obj_palette_index & 0b111111) as usize;
                self.obj_palette_ram[index] = value;
                // auto-increment enabled
                if self.obj_palette_index & 0b10000000 != 0 {
                    self.obj_palette_index = (self.obj_palette_index + 1) & 0x3F;
                };
            }
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn read_cgb_registers(&self, address: u16) -> u8 {
        match address {
            0xFF68 => self.bg_palette_index,
            0xFF69 => self.bg_palette_ram[(self.bg_palette_index & 0b111111) as usize],
            0xFF6A => self.obj_palette_index,
            0xFF6B => self.obj_palette_ram[(self.obj_palette_index & 0b111111) as usize],
            _ => unreachable!(),
        }
    }
}

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
    // Use serialized arrays for VRAM/WRAM data
    pub vram_data: Vec<u8>, // Will store flattened VRAM data
    pub wram_data: Vec<u8>, // Will store flattened WRAM data
    pub current_wram_bank: usize,
    #[serde(with = "serde_arrays")]
    pub debug: [u8; 0x100],
    pub mbc: MbcTypeState,
    pub gb_mode: GameboyMode,
}
