pub struct Memory {
    ram: [u8; 65536],
}

impl Memory {
    pub fn new() -> Self {
        Self { ram: [0; 65536] }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value;
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
