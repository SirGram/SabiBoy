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
            bg_palette_ram: [0; 64],
            obj_palette_ram: [0; 64],
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
    #[inline(always)]
    pub fn read_register(&self, addr: u16) -> u8 {
        match addr {
            0xFF4F => self.vram_bank | 0xFE, // Reading returns other bits as 1
            0xFF4D => self.speed_switch,
            0xFF68 => self.bg_palette_index,
            0xFF69 => {
                let index = (self.bg_palette_index & 0x3F) as usize;
                self.bg_palette_ram[index]
            }
            0xFF6A => self.obj_palette_index,
            0xFF6B => {
                let index = (self.obj_palette_index & 0x3F) as usize;
                self.obj_palette_ram[index]
            }
            0xFF70 => self.wram_bank,
            0xFF55 => {
                if self.hdma_active {
                    (self.dma_length >> 4) as u8
                } else {
                    0xFF
                }
            }
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    pub fn write_register(&mut self, addr: u16, value: u8) {
        /* println!("CGB write register {:04X} = {:02X}", addr, value); */
        match addr {
            0xFF4F => self.vram_bank = value & 0x01,
            0xFF4D => self.handle_speed_switch(value),
            0xFF68 => {
                println!(
                    "Writing to BG palette index: {:02X}, auto-increment: {}",
                    value & 0x3F,
                    if value & 0x80 != 0 { "yes" } else { "no" }
                );
                self.bg_palette_index = value;
            }
            0xFF69 => {
                println!(
                    "Writing to BG palette data: {:02X} at index {:02X}",
                    value,
                    self.bg_palette_index & 0x3F
                );
                self.write_bg_palette(value);
            }
            0xFF6A => self.obj_palette_index = value,
            0xFF6B => self.write_obj_palette(value),
            0xFF70 => self.set_wram_bank(value),
            0xFF51 => self.dma_source = (self.dma_source & 0x00FF) | ((value as u16) << 8),
            0xFF52 => self.dma_source = (self.dma_source & 0xFF00) | ((value as u16) & 0xF0),
            0xFF53 => self.dma_dest = (self.dma_dest & 0x00FF) | (((value as u16) & 0x1F) << 8),
            0xFF54 => self.dma_dest = (self.dma_dest & 0xFF00) | ((value as u16) & 0xF0),
            0xFF55 => return self.handle_hdma(value),
            _ => unreachable!(),
        }
    }
    #[inline(always)]
    pub fn write_bg_palette(&mut self, value: u8) {
        let index = (self.bg_palette_index & 0x3F) as usize;
       /*  println!(
            "Writing BG palette: value={:02X}, index={}, auto_increment={}",
            value,
            index,
            self.bg_palette_index & 0x80 != 0
        ); */

        self.bg_palette_ram[index] = value;

        // If this completed a color (every 2 bytes), log the full color
        if index % 2 == 1 {
            let palette_num = (index / 8) as u8;
            let color_num = ((index % 8) / 2) as u8;
            let low = self.bg_palette_ram[index - 1];
            let high = value;
            let (r, g, b) = Self::convert_color(low, high);
            println!(
                "Set BG Palette {}, Color {}: bytes={:02X}{:02X} RGB({}, {}, {})",
                palette_num, color_num, low, high, r, g, b
            );
        }

        // Auto-increment if enabled
        if self.bg_palette_index & 0x80 != 0 {
            self.bg_palette_index = 0x80 | ((self.bg_palette_index.wrapping_add(1)) & 0x3F);
        }
    }
    #[inline(always)]
    pub fn write_obj_palette(&mut self, value: u8) {
        let index = (self.obj_palette_index & 0x3F) as usize;
        println!(
            "Writing OBJ palette: value={:02X}, index={}, auto_increment={}",
            value,
            index,
            self.obj_palette_index & 0x80 != 0
        );

        self.obj_palette_ram[index] = value;

        // If this completed a color (every 2 bytes), log the full color
        if index % 2 == 1 {
            let palette_num = (index / 8) as u8;
            let color_num = ((index % 8) / 2) as u8;
            let low = self.obj_palette_ram[index - 1];
            let high = value;
            let (r, g, b) = Self::convert_color(low, high);
            println!(
                "Set OBJ Palette {}, Color {}: bytes={:02X}{:02X} RGB({}, {}, {})",
                palette_num, color_num, low, high, r, g, b
            );
        }

        // Auto-increment if enabled
        if self.obj_palette_index & 0x80 != 0 {
            self.obj_palette_index = 0x80 | ((self.obj_palette_index.wrapping_add(1)) & 0x3F);
        }
    }

    fn set_wram_bank(&mut self, value: u8) {
        self.wram_bank = if value & 0x07 == 0 { 1 } else { value & 0x07 };
    }

    fn handle_speed_switch(&mut self, value: u8) {
        self.speed_switch = (self.speed_switch & 0x80) | (value & 0x01);
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
        // Calculate the byte index into palette RAM
        // Each palette has 8 bytes (4 colors × 2 bytes per color)
        let base_index = (palette & 0x07) * 8;
        // Each color takes 2 bytes
        let color_offset = (color_id & 0x03) * 2;
        let index = (base_index + color_offset) as usize;

        let low = self.bg_palette_ram[index];
        let high = self.bg_palette_ram[index + 1];
        
  /*println!("palette {} id {}", palette, color_id);
       println!(
            "Reading BG Color - Palette: {}, Color ID: {}, Index: {}, Bytes: {:02X}{:02X}",
            palette, color_id, index, low, high
        );  */

        let color = Self::convert_color(low, high);
        color
    }

    pub fn get_obj_color(&self, palette: u8, color_id: u8) -> (u8, u8, u8) {
        // Same indexing scheme as BG colors
        let base_index = (palette & 0x07) * 8;
        let color_offset = (color_id & 0x03) * 2;
        let index = (base_index + color_offset) as usize;

        let low = self.obj_palette_ram[index];
        let high = self.obj_palette_ram[index + 1];

   /*      println!(
            "Reading OBJ Color - Palette: {}, Color ID: {}, Index: {}, Bytes: {:02X}{:02X}",
            palette, color_id, index, low, high
        ); */

        Self::convert_color(low, high)
    }

    fn convert_color(low: u8, high: u8) -> (u8, u8, u8) {
        let color = ((high as u16) << 8) | (low as u16);
        // Extract 5-bit color components (15-bit format: 0RRRRRGGGGGBBBBB)
        let b = (color & 0x1F) as u8;           
        let g = ((color >> 5) & 0x1F) as u8;   
        let r = ((color >> 10) & 0x1F) as u8;  
    
        // Convert 5-bit to 8-bit color depth (e.g., 0b11111 -> 0xFF)
        let r_8bit = (r << 3) | (r >> 2);     
        let g_8bit = (g << 3) | (g >> 2);      
        let b_8bit = (b << 3) | (b >> 2);     
    
        let color = (r_8bit, g_8bit, b_8bit);
        color
    }
    
}
