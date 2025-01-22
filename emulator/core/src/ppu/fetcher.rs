use super::pixelfifo::PixelFifo;
use crate::bus::{io_address::IoRegister, GameboyMode, MemoryInterface};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fetcher {
    pub step: u8,
    pub tile_number: u8,
    tile_data_low: u8,
    tile_data_high: u8,
    pub is_window_fetch: bool,
    pub x_pos_counter: u16,
    pub window_line_counter: u16,
    pub pause: bool,
    pub delay: usize,

    tile_attrs: u8, // CGB only

    saved_state: Option<FetcherState>, //  save state when pausing
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct FetcherState {
    step: u8,
    tile_number: u8,
    tile_data_low: u8,
    tile_data_high: u8,
    tile_attrs: u8,
}

impl Fetcher {
    pub fn new() -> Self {
        Self {
            step: 0,
            tile_number: 0,
            tile_data_low: 0,
            tile_data_high: 0,
            tile_attrs: 0,
            is_window_fetch: false,
            x_pos_counter: 0,
            window_line_counter: 0,
            pause: false,
            delay: 0,

            saved_state: None,
        }
    }

    pub fn scanline_reset(&mut self) {
        self.step = 0;
        self.is_window_fetch = false;
        self.x_pos_counter = 0;
        self.pause = false;
    }

    pub fn window_trigger(&mut self, pixel_fifo: &mut PixelFifo) {
        self.step = 0;
        self.is_window_fetch = true;
        self.x_pos_counter = 7;
        pixel_fifo.reset();
    }

    pub fn step<M: MemoryInterface>(&mut self, memory: &mut M, pixel_fifo: &mut PixelFifo) {
        match self.step {
            0 => {
                self.fetch_tile_number(memory);
                self.step += 1;
            }
            1 => {
                self.tile_data_low = self.fetch_tile_data(memory, self.tile_number, false);
                self.step += 1;
            }
            2 => {
                self.tile_data_high = self.fetch_tile_data(memory, self.tile_number, true);
                self.step += 1;
            }
            3 => {
                self.push_to_fifo(memory, pixel_fifo);
            }
            _ => self.step = 0,
        }
    }

    fn get_tile_map_base(&self, lcdc: u8) -> u16 {
        if self.is_window_fetch {
            if (lcdc & 0x40) != 0 {
                0x9C00
            } else {
                0x9800
            }
        } else {
            if (lcdc & 0x08) != 0 {
                0x9C00
            } else {
                0x9800
            }
        }
    }

    fn fetch_tile_number<M: MemoryInterface>(&mut self, memory: &mut M) {
        let lcdc = memory.read_byte(IoRegister::Lcdc.address());
        let scx = memory.read_byte(IoRegister::Scx.address());
        let scy = memory.read_byte(IoRegister::Scy.address());
        let ly = memory.read_byte(IoRegister::Ly.address());

        let tile_y = if self.is_window_fetch {
            (self.window_line_counter / 8) & 0x1F
        } else {
            ((ly.wrapping_add(scy)) / 8) as u16 & 0x1F
        };

        let tile_x = if self.is_window_fetch {
            self.x_pos_counter / 8
        } else {
            ((scx as u16 / 8) + (self.x_pos_counter) / 8) & 0x1F
        };

        let tile_map_base = self.get_tile_map_base(lcdc);
        let tile_address = tile_map_base + (tile_y * 32) + tile_x;

        // Read tile number and attributes based on mode
        match memory.gb_mode() {
            GameboyMode::DMG => {
                self.tile_number = memory.read_byte(tile_address);
                self.tile_attrs = 0;
            }
            GameboyMode::CGB => {
                // Tile number from bank 0
                self.tile_number = memory.read_byte_vram_bank(tile_address, 0);
                self.tile_attrs = memory.read_byte_vram_bank(tile_address, 1);
            }
        }
    }

    fn fetch_tile_data<M: MemoryInterface>(
        &mut self,
        memory: &mut M,
        tile_number: u8,
        is_high_byte: bool,
    ) -> u8 {
        let ly = memory.read_byte(IoRegister::Ly.address());
        let scy = memory.read_byte(IoRegister::Scy.address());
        let lcdc = memory.read_byte(IoRegister::Lcdc.address());

        // Calculate y position within tile
        let mut y_offset = if self.is_window_fetch {
            (self.window_line_counter % 8) * 2
        } else {
            ((ly as u16 + scy as u16) % 8) * 2
        };

        // Apply vertical flip if needed in CGB mode
        if memory.gb_mode() == GameboyMode::CGB && (self.tile_attrs & 0x40) != 0 {
            y_offset = 14 - y_offset; // 14 = (8 - 1) * 2
        }

        let base_address = if (lcdc & 0x10) != 0 {
            0x8000 + (tile_number as u16 * 16)
        } else {
            0x9000u16.wrapping_add((tile_number as i8 as i16 * 16) as u16)
        };

        let address = base_address + y_offset + if is_high_byte { 1 } else { 0 };

        let data;
        match memory.gb_mode() {
            GameboyMode::DMG => {
                data = memory.read_byte(address);
            }
            GameboyMode::CGB => {
                let selected_bank = self.tile_attrs >> 3 & 0b1;
                data = memory.read_byte_vram_bank(address, selected_bank as usize);
            }
        }
        data
    }

    fn push_to_fifo<M: MemoryInterface>(&mut self, memory: &M, pixel_fifo: &mut PixelFifo) {
        if pixel_fifo.bg_pixel_count() != 0 || self.pause {
            return;
        }

        // Decode tile data into pixels
        let mut pixels = Vec::with_capacity(8);
        for bit in 0..8 {
            let low_bit = (self.tile_data_low >> (7 - bit)) & 0x1;
            let high_bit = (self.tile_data_high >> (7 - bit)) & 0x1;
            let color = (high_bit << 1) | low_bit;
            pixels.push(color);
        }

        // Apply horizontal flip if needed (CGB mode)
        if memory.gb_mode() == GameboyMode::CGB && (self.tile_attrs & 0x20) != 0 {
            pixels.reverse();
        }

        // Push pixels to FIFO with appropriate attributes
        for color in pixels {
            let pixel = super::pixelfifo::Pixel::new_bg(memory, color, self.tile_attrs);
            pixel_fifo.bg_fifo.push_back(pixel);
        }

        self.step = 0;
    }

    pub fn pause(&mut self) {
        if !self.pause {
            self.saved_state = Some(FetcherState {
                step: self.step,
                tile_number: self.tile_number,
                tile_data_low: self.tile_data_low,
                tile_data_high: self.tile_data_high,
                tile_attrs: self.tile_attrs,
            });
            self.pause = true;
        }
    }

    pub fn unpause(&mut self) {
        if self.pause {
            if let Some(saved) = self.saved_state.take() {
                self.step = saved.step;
                self.tile_number = saved.tile_number;
                self.tile_data_low = saved.tile_data_low;
                self.tile_data_high = saved.tile_data_high;
                self.tile_attrs = saved.tile_attrs;
            }
            self.pause = false;
        }
    }
}
