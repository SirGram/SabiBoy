use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Mbc0 {
    rom: Vec<u8>,
    ram: Vec<u8>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mbc0State {
    ram: Vec<u8>,
}

impl Mbc0 {
    /*
    ROM BANK: 2 banks 16KB
    RAM BANK: Up to 1 bank 8KB
     */
    pub fn new(rom: &[u8], ram_size: usize) -> Self {
        Self {
            rom: rom.to_vec(),
            ram: vec![0; ram_size],
        }
    }
    pub fn save_state(&self) -> Mbc0State {
        Mbc0State {
            ram: self.ram.clone(),
        }
    }
    pub fn load_state(&mut self, state: Mbc0State) {
        self.ram = state.ram;
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.rom[address as usize],
            0xA000..=0xBFFF => {
                if !self.ram.is_empty() {
                    self.ram[(address - 0xA000) as usize]
                } else {
                    0xFF // Return 0xFF if no RAM
                }
            }
            _ => 0xFF,
        }
    }
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => {}
            0xA000..=0xBFFF => {
                if !self.ram.is_empty() {
                    self.ram[(address - 0xA000) as usize] = value
                }
            }
            _ => {}
        }
    }
}
