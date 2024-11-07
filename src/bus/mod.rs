pub struct Bus {
    rom_bank_0: [u8; 0x4000],
    rom_bank_n: [u8; 0x4000],
    vram: [u8; 0x2000],
    external_ram: [u8; 0x2000],
    ram_bank_0: [u8; 0x1000],
    ram_bank_n: [u8; 0x1000],
    oam: [u8; 0x10A], // object attribute memory
    io_registers: [u8; 0x80],
    hram: [u8; 0x7F],
    ie_register: u8,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            rom_bank_0: [0; 0x4000],
            rom_bank_n: [0; 0x4000],
            vram: [0; 0x2000],
            external_ram: [0; 0x2000],
            ram_bank_0: [0; 0x1000],
            ram_bank_n: [0; 0x1000],
            oam: [0; 0x10A],
            io_registers: [0; 0x80],
            hram: [0; 0x7F],
            ie_register: 0,
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        for (i, &byte) in rom.iter().enumerate() {
            if i < 0x4000 {
                self.rom_bank_0[i] = byte;
            } else if i < 0x8000 {
                self.rom_bank_n[i - 0x4000] = byte;
            }
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom_bank_0[address as usize],
            0x4000..=0x7FFF => self.rom_bank_n[(address - 0x4000) as usize],
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize],
            0xA000..=0xBFFF => self.external_ram[(address - 0xA000) as usize],
            0xC000..=0xCFFF => self.ram_bank_0[(address - 0xC000) as usize],
            0xD000..=0xDFFF => self.ram_bank_n[(address - 0xD000) as usize],
            0xE000..=0xFDFF => self.read_byte(address - 0x2000), // Echo RAM: Map E000-FDFF to C000-DDFF
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize],
            0xFEA0..=0xFEFF => 0xFF, // not usable TODO: implement https://gbdev.io/pandocs/Memory_Map.html#fea0feff-range
            0xFF00..=0xFF7F => self.io_registers[(address - 0xFF00) as usize],
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => self.ie_register,
            _ => 0xFF,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x3FFF => self.rom_bank_0[address as usize] = value,
            0x4000..=0x7FFF => self.rom_bank_n[(address - 0x4000) as usize] = value,
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize] = value,
            0xA000..=0xBFFF => self.external_ram[(address - 0xA000) as usize] = value,
            0xC000..=0xCFFF => self.ram_bank_0[(address - 0xC000) as usize] = value,
            0xD000..=0xDFFF => self.ram_bank_n[(address - 0xD000) as usize] = value,
            0xE000..=0xFDFF => self.write_byte(address - 0x2000, value), // Echo RAM: Map E000-FDFF to C000-DDFF
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize] = value,
            0xFEA0..=0xFEFF => {} // Unusable
            0xFF00..=0xFF7F => self.io_registers[(address - 0xFF00) as usize] = value,
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = value,
            0xFFFF => self.ie_register = value,
            _ => {}
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
}
