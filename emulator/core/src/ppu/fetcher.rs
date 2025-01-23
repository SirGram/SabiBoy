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

  
    pub fn step(
        &mut self,
        lcdc: u8,
        scy: u8,
        scx: u8,
        ly: u8,
        wx: u8,
        bgp: u8,
        gb_mode: GameboyMode,
        vram_banks: &[[u8; 0x2000]],
        pixel_fifo: &mut PixelFifo,
    ) {
        if self.pause {
            return;
        }

        match self.step {
            0 => {self.fetch_tile_number(lcdc, scy, scx, ly, gb_mode, vram_banks);
                self.step += 1;
            },
            1 => {
                self.tile_data_low = self.fetch_tile_data(lcdc, ly, scy, gb_mode, vram_banks, false);
                self.step += 1;
            }
            2 => {
                self.tile_data_high = self.fetch_tile_data(lcdc, ly, scy, gb_mode, vram_banks, true);
                self.step += 1;
            }
            3 => {self.push_to_fifo(gb_mode, pixel_fifo);
                self.step += 1;},
            _ => self.step = 0,
        }

        self.step = (self.step + 1) % 4;
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

    fn fetch_tile_number(
        &mut self,
        lcdc: u8,
        scy: u8,
        scx: u8,
        ly: u8,
        gb_mode: GameboyMode,
        vram_banks: &[[u8; 0x2000]],
    ) {
        let tile_map_base = self.get_tile_map_base(lcdc);
        let (tile_y, tile_x) = if self.is_window_fetch {
            let tile_y = self.window_line_counter / 8;
            let tile_x = self.x_pos_counter / 8;
            (tile_y, tile_x)
        } else {
            let y_pos = ly.wrapping_add(scy) as u16;
            let tile_y = y_pos / 8;
            let x_pos = (scx as u16) + self.x_pos_counter;
            let tile_x = x_pos / 8;
            (tile_y, tile_x)
        };

        let tile_address = tile_map_base + (tile_y * 32) + tile_x;
        let vram_addr = (tile_address - 0x8000) as usize;

        self.tile_number = vram_banks[0][vram_addr];
        if gb_mode == GameboyMode::CGB {
            self.tile_attrs = vram_banks[1][vram_addr];
        } else {
            self.tile_attrs = 0;
        }
    }

    fn fetch_tile_data(
        &mut self,
        lcdc: u8,
        ly: u8,
        scy: u8,
        gb_mode: GameboyMode,
        vram_banks: &[[u8; 0x2000]],
        is_high_byte: bool,
    ) -> u8 {
        let mut y_offset = if self.is_window_fetch {
            (self.window_line_counter % 8) * 2
        } else {
            ((ly as u16 + scy as u16) % 8) * 2
        };

        if gb_mode == GameboyMode::CGB && (self.tile_attrs & 0x40) != 0 {
            y_offset = 14 - y_offset;
        }

        let base_address = if (lcdc & 0x10) != 0 {
            0x8000 + (self.tile_number as u16 * 16)
        } else {
            0x9000u16.wrapping_add((self.tile_number as i8 as i16 * 16) as u16)
        };

        let address = base_address + y_offset + if is_high_byte { 1 } else { 0 };
        let vram_addr = (address - 0x8000) as usize;

        let bank = if gb_mode == GameboyMode::CGB {
            (self.tile_attrs >> 3) & 0x01
        } else {
            0
        };

        vram_banks[bank as usize][vram_addr]
    }

    fn push_to_fifo(&mut self, gb_mode: GameboyMode, pixel_fifo: &mut PixelFifo) {
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
        if gb_mode == GameboyMode::CGB && (self.tile_attrs & 0x20) != 0 {
            pixels.reverse();
        }

        // Push pixels to FIFO with appropriate attributes
        for color in pixels {
            let pixel = super::pixelfifo::Pixel::new_bg(color, self.tile_attrs , gb_mode);
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
