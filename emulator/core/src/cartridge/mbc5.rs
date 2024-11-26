use serde::de::value;

pub struct Mbc5 {
    current_rom_bank: u16,
    current_ram_bank: u8,
    rom: Vec<u8>,
    ram: Vec<u8>,
    external_ram_enabled: bool,
}

impl Mbc5 {
    /*
    ROM BANK: Up to 512 banks of 16KB
    RAM BANK: Up to 16 banks of 8KB
     */
    // TODO: rumble?
    pub fn new(rom: &[u8]) -> Self {
        Self {
            current_rom_bank: 1,
            current_ram_bank: 0,
            rom: rom.to_vec(),
            ram: vec![0; 0x2000 * 16],
            external_ram_enabled: false,
        }
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom[address as usize],
            0x4000..=0x7FFF => {
                self.rom
                    [0x4000 * self.current_rom_bank as usize + (address as usize - 0x4000)]
            }
            0xA000..=0xBFFF => {
                if self.external_ram_enabled {
                    self.ram
                        [0x2000 * self.current_ram_bank as usize + (address as usize - 0xA000)]
                } else {
                    0xFF
                }
            }
            _ => 0xFF,
        }
    }
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                // enable external ram
                let lower_value = value & 0xF;
                if lower_value == 0x0A {
                    self.external_ram_enabled = true;
                } else {
                    self.external_ram_enabled = false;
                }
            }
            0x2000..=0x2FFF => {
                // set lower 8 bits and preserve 9th bit
                self.current_rom_bank = (self.current_rom_bank & 0x100) | value as u16;
            }
            0x3000..=0x3FFF => {
                // set bit0 higher 8 bits and preserve lower 8 bits
                self.current_rom_bank =
                    (self.current_rom_bank & 0xFF) | ((value & 0x01) as u16) << 8;
            }
            0x4000..=0x5FFF => {
                // set ram value 0-15
                self.current_ram_bank = value & 0x0F;
            }
            0xA000..=0xBFFF => {
                // write to external ram
                if !self.external_ram_enabled {
                    return;
                }
                let offset = 0x2000 * self.current_ram_bank as usize + (address - 0xA000) as usize;
                if offset < self.ram.len() {
                    self.ram[offset] = value;
                }
            }
            _ => {}
        }
    }
}
