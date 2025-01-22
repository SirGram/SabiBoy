use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mbc3 {
    current_rom_bank: u8,
    current_ram_bank: u8,
    rom: Vec<u8>,
    ram: Vec<u8>,
    external_ram_enabled: bool,
    previous_latch_value: u8,
    pub rtc: Option<Rtc>,
    current_rtc_register: Option<u8>,
    rom_bank_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mbc3State {
    current_rom_bank: u8,
    current_ram_bank: u8,
    ram: Vec<u8>,
    external_ram_enabled: bool,
    previous_latch_value: u8,
    pub rtc: Option<Rtc>,
    current_rtc_register: Option<u8>,
    rom_bank_count: usize,
}

impl Mbc3 {
    /*
    ROM BANK: Up to 128 banks of 16KB
    RAM BANK: Up to 4 banks of 8KB
    TODO: battery save external ram
     */
    pub fn new(rom: &[u8], ram_size: usize, has_rtc: bool) -> Self {
        let rom_bank_count = rom.len() / 0x4000;
        Self {
            current_rom_bank: 1,
            current_ram_bank: 0,
            rom: rom.to_vec(),
            ram: vec![0; ram_size],
            external_ram_enabled: false,
            previous_latch_value: 0xFF,
            rtc: if has_rtc { Some(Rtc::new()) } else { None },
            current_rtc_register: None,
            rom_bank_count,
        }
    }
    pub fn save_state(&self) -> Mbc3State {
        Mbc3State {
            current_rom_bank: self.current_rom_bank,
            current_ram_bank: self.current_ram_bank,
            ram: self.ram.clone(),
            external_ram_enabled: self.external_ram_enabled,
            previous_latch_value: self.previous_latch_value,
            rtc: self.rtc.clone(),
            current_rtc_register: self.current_rtc_register,
            rom_bank_count: self.rom_bank_count,
        }
    }
    pub fn load_state(&mut self, state: Mbc3State) {
        self.current_rom_bank = state.current_rom_bank;
        self.current_ram_bank = state.current_ram_bank;
        self.ram = state.ram;
        self.external_ram_enabled = state.external_ram_enabled;
        self.previous_latch_value = state.previous_latch_value;
        self.rtc = state.rtc;
        self.current_rtc_register = state.current_rtc_register;
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom[address as usize],
            0x4000..=0x7FFF => {
                let bank = (self.current_rom_bank as usize % self.rom_bank_count).max(1);
                let offset = bank * 0x4000 + (address as usize - 0x4000);
                self.rom[offset]
            }
            0xA000..=0xBFFF => {
                // rtc 1st then ram if no rtc
                if !self.external_ram_enabled {
                    return 0xFF;
                }
                if let Some(rtc_register) = self.current_rtc_register {
                    if let Some(rtc) = &self.rtc {
                        return rtc.read(rtc_register);
                    }
                }
                self.ram[0x2000 * self.current_ram_bank as usize + (address - 0xA000) as usize]
            }
            _ => 0xFF,
        }
    }
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                // RAM and RTC enabling
                self.external_ram_enabled = value & 0x0F == 0x0A;
            }
            0x2000..=0x3FFF => {
                // ROM bank switching
                let new_number = if value & 0x7F == 0 { 1 } else { value & 0x7F }; // 127 mask

                self.current_rom_bank = new_number;
            }
            0x4000..=0x5FFF => {
                // RAM bank switching
                if value <= 0x03 {
                    self.current_ram_bank = value;
                    self.current_rtc_register = None;
                } else if (0x08..=0x0C).contains(&value) {
                    // select rtc register
                    self.current_rtc_register = Some(value);
                }
            }
            0x6000..=0x7FFF => {
                if self.rtc.is_none() {
                    return;
                }

                // RTC Latch mechanism
                if self.previous_latch_value == 0x00 && value == 0x01 {
                    // Latch RTC registers
                    if let Some(rtc) = &mut self.rtc {
                        rtc.latch();
                    }
                }
                self.previous_latch_value = value;
            }
            0xA000..=0xBFFF => {
                if !self.external_ram_enabled {
                    return;
                }

                // Check if we're writing to RTC register
                if let Some(rtc_register) = self.current_rtc_register {
                    if let Some(rtc) = &mut self.rtc {
                        rtc.write(rtc_register, value);
                    }
                } else {
                    // Write to RAM
                    self.ram
                        [0x2000 * self.current_ram_bank as usize + (address - 0xA000) as usize] =
                        value;
                }
            }
            _ => {}
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rtc {
    s_reg: u8,  // seconds
    m_reg: u8,  // minutes
    h_reg: u8,  // hours
    dl_reg: u8, // day counter low
    dh_reg: u8, // day counter high
    cycles: usize,
    s_reg_latched: u8,
    m_reg_latched: u8,
    h_reg_latched: u8,
    dl_reg_latched: u8,
    dh_reg_latched: u8,
    is_latched: bool,
}
impl Rtc {
    pub fn new() -> Self {
        Self {
            s_reg: 0,
            m_reg: 0,
            h_reg: 0,
            dl_reg: 0,
            dh_reg: 0,
            cycles: 0,
            s_reg_latched: 0,
            m_reg_latched: 0,
            h_reg_latched: 0,
            dl_reg_latched: 0,
            dh_reg_latched: 0,
            is_latched: false,
        }
    }
    pub fn latch(&mut self) {
        self.is_latched = true;
        self.s_reg_latched = self.s_reg;
        self.m_reg_latched = self.m_reg;
        self.h_reg_latched = self.h_reg;
        self.dl_reg_latched = self.dl_reg;
        self.dh_reg_latched = self.dh_reg;
    }
    pub fn tick(&mut self) {
        self.cycles += 1;
        if self.cycles < 419433304 {
            return;
        }
        self.cycles = 0;
        self.s_reg = (self.s_reg + 1) & 0x3F;
        if self.s_reg == 60 {
            self.s_reg = 0;
            self.m_reg = (self.m_reg + 1) & 0x3F;
        }
        if self.m_reg == 60 {
            self.m_reg = 0;
            self.h_reg = (self.h_reg + 1) & 0x1F;
        }
        if self.h_reg == 24 {
            self.h_reg = 0;
            let mut day = (((self.dh_reg & 0x01) as u16) << 8) | self.dl_reg as u16;
            day += 1;
            // check overflow
            if day > 0x1FF {
                day = 1;
                self.dh_reg |= 0x80; // carry bit
            }
            self.dl_reg = (day & 0xFF) as u8;
            self.dh_reg = (day >> 8) as u8 & 0x01 | self.dh_reg & 0x7E; // 8th bit
        }
    }

    pub fn read(&self, address: u8) -> u8 {
        if !self.is_latched {
            return 0xFF;
        }
        match address {
            // Upper bits need to read as 1
            0x08 => self.s_reg_latched | 0xC0,
            0x09 => self.m_reg_latched | 0xC0,
            0x0A => self.h_reg_latched | 0xE0,
            0x0B => self.dl_reg_latched,
            0x0C => self.dh_reg_latched | 0x3E,
            _ => 0xFF,
        }
    }
    pub fn write(&mut self, address: u8, value: u8) {
        match address {
            0x08 => self.s_reg_latched = value & 0x3F,
            0x09 => self.m_reg_latched = value & 0x3F,
            0x0A => self.h_reg_latched = value & 0x1F,
            0x0B => self.dl_reg_latched = value,
            0x0C => self.dh_reg_latched = value & 0xC1, // 3 bit flags
            _ => {}
        }
    }
}
