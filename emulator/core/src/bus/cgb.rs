use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CgbRegisters {
    vram_bank: u8,
    wram_bank: u8,
    bg_palette_index: u8,
    obj_palette_index: u8,
    #[serde(with = "serde_arrays")]
    bg_palette_ram: [u8; 64],
    #[serde(with = "serde_arrays")]
    obj_palette_ram: [u8; 64],
    speed_switch: u8,
    dma_source: u16,
    dma_dest: u16,
    dma_length: u16,
    dma_active: bool,
    hdma_active: bool,
}

impl Default for CgbRegisters {
    fn default() -> Self {
        Self {
            vram_bank: 0,
            wram_bank: 1,
            bg_palette_index: 0,
            obj_palette_index: 0,
            bg_palette_ram: [0xFF; 64],
            obj_palette_ram: [0xFF; 64],
            speed_switch: 0,
            dma_source: 0,
            dma_dest: 0,
            dma_length: 0,
            dma_active: false,
            hdma_active: false,
        }
    }
}

impl CgbRegisters {
    pub fn read_register(&self, addr: u16) -> u8 {
        match addr {
            0xFF4F => self.vram_bank,
            0xFF4D => self.speed_switch,
            0xFF68 => self.bg_palette_index,
            0xFF69 => self.bg_palette_ram[(self.bg_palette_index & 0x3F) as usize],
            0xFF6A => self.obj_palette_index,
            0xFF6B => self.obj_palette_ram[(self.obj_palette_index & 0x3F) as usize],
            0xFF70 => self.wram_bank,
            0xFF55 => {
                if self.hdma_active {
                    (self.dma_length >> 4) as u8
                } else {
                    0xFF
                }
            }
            _ => 0xFF,
        }
    }

    pub fn write_register(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF4F => self.vram_bank = value & 0x01,
            0xFF4D => self.handle_speed_switch(value),
            0xFF68 => self.bg_palette_index = value,
            0xFF69 => self.write_bg_palette(value),
            0xFF6A => self.obj_palette_index = value,
            0xFF6B => self.write_obj_palette(value),
            0xFF70 => self.set_wram_bank(value),
            0xFF51 => self.dma_source = (self.dma_source & 0x00FF) | ((value as u16) << 8),
            0xFF52 => self.dma_source = (self.dma_source & 0xFF00) | ((value as u16) & 0xF0),
            0xFF53 => self.dma_dest = (self.dma_dest & 0x00FF) | (((value as u16) & 0x1F) << 8),
            0xFF54 => self.dma_dest = (self.dma_dest & 0xFF00) | ((value as u16) & 0xF0),
            0xFF55 => return self.handle_hdma(value),
            _ => {}
        }
    }

    pub fn write_bg_palette(&mut self, value: u8) {
        let index = (self.bg_palette_index & 0x3F) as usize;
        self.bg_palette_ram[index] = value;

        // Debug print when writing a complete color (every 2 bytes)
        if index % 2 == 1 {
            let low = self.bg_palette_ram[index - 1];
            let high = value;
            let (r, g, b) = Self::convert_color(low, high);
            println!(
                "BG Palette {}, Color {}: RGB({}, {}, {})",
                index / 8,       // Palette number
                (index % 8) / 2, // Color number in palette
                r,
                g,
                b
            );
        }

        if self.bg_palette_index & 0x80 != 0 {
            self.bg_palette_index =
                (self.bg_palette_index & 0x80) | ((self.bg_palette_index.wrapping_add(1)) & 0x3F);
        }
    }

    fn write_obj_palette(&mut self, value: u8) {
        let index = (self.obj_palette_index & 0x3F) as usize;
        self.obj_palette_ram[index] = value;
        if self.obj_palette_index & 0x80 != 0 {
            self.obj_palette_index =
                (self.obj_palette_index & 0x80) | ((self.obj_palette_index.wrapping_add(1)) & 0x3F);
        }
    }

    fn set_wram_bank(&mut self, value: u8) {
        self.wram_bank = if value & 0x07 == 0 { 1 } else { value & 0x07 };
    }

    fn handle_speed_switch(&mut self, value: u8) {
        self.speed_switch = (self.speed_switch & 0x80) | (value & 0x7F);
        // Note: Actual speed switch occurs after STOP instruction
    }

    fn handle_hdma(&mut self, value: u8) {
        if self.hdma_active && value & 0x80 == 0 {
            // Stop HDMA
            self.hdma_active = false;
        }

        let length = ((value & 0x7F) as u16 + 1) * 0x10;

        if value & 0x80 != 0 {
            // H-Blank DMA
            self.hdma_active = true;
            self.dma_length = length;
        } else {
            // General Purpose DMA
            self.dma_active = true;
            self.dma_length = length;
        }
    }

    pub fn get_vram_bank(&self) -> usize {
        self.vram_bank as usize
    }

    pub fn get_wram_bank(&self) -> usize {
        self.wram_bank as usize
    }

    pub fn get_bg_color(&self, palette: u8, color_id: u8) -> (u8, u8, u8) {
        let index = ((palette & 0x07) * 8 + (color_id & 0x03) * 2) as usize;
        let low = self.bg_palette_ram[index];
        let high = self.bg_palette_ram[index + 1];
        Self::convert_color(low, high)
    }

    pub fn get_obj_color(&self, palette: u8, color_id: u8) -> (u8, u8, u8) {
        let index = (((palette & 0x07) << 3) | (color_id & 0x03)) << 1;
        let low = self.obj_palette_ram[index as usize];
        let high = self.obj_palette_ram[(index + 1) as usize];
        Self::convert_color(low, high)
    }

  fn convert_color(low: u8, high: u8) -> (u8, u8, u8) {
    let color = ((high as u16) << 8) | (low as u16);
    // CGB uses 5 bits per color channel
    let r = (color & 0x1F) as u8;
    let g = ((color >> 5) & 0x1F) as u8;
    let b = ((color >> 10) & 0x1F) as u8;

    // Convert 5-bit to 8-bit color (multiply by 8)
    (
        (r << 3) | (r >> 2),
        (g << 3) | (g >> 2),
        (b << 3) | (b >> 2),
    )
}
}
