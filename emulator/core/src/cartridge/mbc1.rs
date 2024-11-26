pub struct Mbc1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    current_rom_bank: u8,
    current_ram_bank: u8,
    mode: bool,
    ram_enabled: bool,
    is_multicart: bool,
}

impl Mbc1 {
    /*
    ROM BANK: Up to 128 banks of 16KB
    RAM BANK: Up to 4 banks of 8KB
    TODO: maybe MBC1M
    TODO: pass mooneye
     */
    pub fn new(rom: &[u8], ram_size: usize) -> Self {
        Self {
            rom: rom.to_vec(),
            ram: vec![0; ram_size],
            current_rom_bank: 1,
            current_ram_bank: 0,
            ram_enabled: false,
            mode: false,
            is_multicart: false,
        }
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => {
                if !self.mode {
                    self.rom[address as usize]
                } else {
                    let zero_bank_number = self.get_zero_bank_number();               
                    self.rom[0x4000 * zero_bank_number as usize + address as usize]
                }
            }
            0x4000..=0x7FFF => {
                let high_bank_number = self.get_high_bank_number();
           
                self.rom[0x4000 * high_bank_number as usize + (address - 0x4000) as usize]
            }
            0xA000..=0xBFFF => {
                if !self.ram_enabled {
                    return 0xFF;
                };
                let ram_size = self.ram.len();
                if ram_size == 0x2000 * 4 {
                    // 32KB ram
                    let offset = if self.mode {
                        0x2000 * self.current_ram_bank as usize
                    } else {
                        0
                    };
                    self.ram[(address - 0xA000) as usize + offset]
                } else {
                    self.ram[(address - 0xA000) as usize % ram_size]
                }
            }
            _ => 0xFF,
        }
    }
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                // enable ram
                self.ram_enabled = value & 0x0F == 0xA;
            }
            0x2000..=0x3FFF => {
                // select rom bank
                let number_of_rom_banks = self.get_number_of_rom_banks();
                let bit_mask = get_bit_mask(number_of_rom_banks as u8);
                let new_rom_bank_number = value & bit_mask;
                if new_rom_bank_number == 0 {
                    self.current_rom_bank = 1;
                } else {
                    self.current_rom_bank = new_rom_bank_number
                }
            }
            0x4000..=0x5FFF => {
                // select ram bank
                self.current_ram_bank = value & 0x03;
            }
            0x6000..=0x7FFF => {
                // change mode
                self.mode = value & 0x01 == 0x01;
            }
            0xA000..=0xBFFF => {
                if !self.ram_enabled {
                    return;
                }
                // write to ram
                let ram_size = self.ram.len();
                if ram_size == 0x2000 * 4 {
                    // 32KB ram
                    let offset = if self.mode {
                        0x2000 * self.current_ram_bank as usize
                    } else {
                        0
                    };

                    self.ram[(address - 0xA000) as usize + offset] = value;
                } else {
                    // 8|16 KB ram
                    self.ram[(address - 0xA000) as usize % ram_size ] = value;
                }
            }
            _ => {}
        }
    }
    fn get_high_bank_number(&self) -> u8 {
        let base_number =
            self.current_rom_bank & get_bit_mask(self.get_number_of_rom_banks() as u8);
        match self.get_number_of_rom_banks() {
            128 => {
                // 128 Banks: Use 2 bits of RAM bank (b5 and b6)
                let ram_bits = (self.current_ram_bank & 0x03) << 5;
                (base_number & 0b10011111) | ram_bits
            }
            64 => {
                // 64 Banks: Use the lower bit of RAM bank (b5)
                let ram_bit = (self.current_ram_bank & 0x01) << 5;
                (base_number & 0b11011111) | ram_bit
            }
            _ => {
                // <= 32 Banks
                base_number
            }
        }
    }

    fn get_zero_bank_number(&self) -> u8 {
        let number = 0b00000000;
        let number_of_rom_banks = self.get_number_of_rom_banks();

        match number_of_rom_banks {
            128 => {
                // 128 Banks: Use the 2 bits of current RAM bank (b5 and b6)
                number | ((self.current_ram_bank & 0x03) << 5)
            }
            64 => {
                // 64 Banks: Use the lower bit of the RAM bank (b5)
                number | ((self.current_ram_bank & 0x01) << 5)
            }
            _ => {
                // <=32 Banks: Always return 0
                0
            }
        }
    }

    fn get_number_of_rom_banks(&self) -> usize {
        (self.rom.len() + 0x3FFF) / 0x4000
    }
}

fn get_bit_mask(number_rom_banks: u8) -> u8 {
    match number_rom_banks {
        128 => 0b00011111,
        64 =>  0b00011111,
        32 =>  0b00011111,
        16 =>  0b00001111,
        8 =>   0b00000111,
        4 =>   0b00000011,
        2 =>   0b00000001,
        _ => unreachable!(),
    }
}
