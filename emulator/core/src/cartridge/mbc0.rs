use serde::de::value;

pub struct Mbc0 {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl Mbc0 {
    /*
    ROM BANK: 2 banks 16KB
    RAM BANK: Up to 1 bank 8KB
     */
    // TODO: rumble?
    pub fn new(rom: &[u8]) -> Self {
        Self {
            rom: rom.to_vec(),
            ram: vec![0; 0x2000],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.rom[address as usize],
            0xA000..=0xBFFF => self.ram[(address - 0xA000) as usize],
            _ => 0xFF,
        }
    }
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.rom[address as usize] = value,
            0xA000..=0xBFFF => self.ram[(address - 0xA000) as usize] = value,
            _ => {}
        }
    }
}
