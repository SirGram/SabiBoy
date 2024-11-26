
pub struct Mbc3 {
    current_rom_bank: u8,
    current_ram_bank: u8,
    rom: Vec<u8>,
    ram: Vec<u8>,
    external_ram_enabled: bool,
}

impl Mbc3 {
    /*
    ROM BANK: Up to 128 banks of 16KB
    RAM BANK: Up to 4 banks of 8KB
     */
    pub fn new(rom: &[u8]) -> Self {
        Self {
            current_rom_bank: 1,
            current_ram_bank: 0,
            rom: rom.to_vec(),
            ram: vec![0; 0x2000 * 4],
            external_ram_enabled: false,
        }       
    }
    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom[address as usize],
            0x4000..=0x7FFF => self.rom[self.current_rom_bank as usize * 0x4000 + (address - 0x4000) as usize],
            0xA000..=0xBFFF => {},
            _=> 0xFF
        }
    }
    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                // RAM and RTC enabling
                self.external_ram_enabled = value & 0x0F == 0x0A;
                
            },
            0x2000..=0x3FFF => {
                // ROM bank switching
            }
            0x4000..=0x5FFF => {


            }
    }
}
}