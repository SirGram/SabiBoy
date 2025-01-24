use serde::{Deserialize, Serialize};

use super::{pixelfifo::PixelFifo, Sprite};
use crate::{
    bus::{self, io_address::IoRegister, GameboyMode, MemoryInterface},
    gameboy,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpriteFetcher {
    pub step: u8,
    tile_number: u8,
    tile_data_low: u8,
    tile_data_high: u8,
    pub active: bool,
    pub remaining_pixels: u8,
    pub sprite: Sprite,
}
impl SpriteFetcher {
    pub fn new() -> Self {
        Self {
            step: 0,
            tile_number: 0,
            tile_data_low: 0,
            tile_data_high: 0,
            active: false,
            remaining_pixels: 0,
            sprite: Sprite::new(),
        }
    }
    pub fn scanline_reset(&mut self) {
        self.step = 0;
        self.active = false;
        self.remaining_pixels = 0;
    }
    pub fn start_fetch(&mut self, sprite: &Sprite) {
        self.step = 0;
        self.sprite = sprite.clone();
        self.active = true;
        self.remaining_pixels = if sprite.x_pos >= 8 { 8 } else { sprite.x_pos };
        // Start with tile number fetch
        self.fetch_tile_number(sprite);
    }
    pub fn step(
        &mut self,
        pixel_fifo: &mut PixelFifo,
        ly: u8,
        lcdc: u8,
        gb_mode: GameboyMode,
        vram_banks: &[[u8; 0x2000]],
    ) {
        match self.step {
            0 => {
                self.tile_data_low = self.fetch_tile_data(false, ly, lcdc, gb_mode, vram_banks);
                self.step += 1;
            }
            1 => {
                self.tile_data_high = self.fetch_tile_data(true, ly, lcdc, gb_mode, vram_banks);
                self.step += 1;
            }
            2 => {
                self.push_to_fifo(pixel_fifo, gb_mode);
                self.scanline_reset();
            }
            _ => {
                self.step = 0;
            }
        }
    }
    fn fetch_tile_number(&mut self, sprite: &Sprite) {
        self.tile_number = sprite.tile_number;
    }
    fn fetch_tile_data(
        &mut self,
        is_high_byte: bool,
        ly: u8,
        lcdc: u8,
        gb_mode: GameboyMode,
        vram_banks: &[[u8; 0x2000]],
    ) -> u8 {
        let y_flip = self.sprite.flags & 0x40 != 0;
        let x_flip = self.sprite.flags & 0x20 != 0;
        let sprite_size = if lcdc & 0x04 != 0 { 16 } else { 8 };

        // Calculate the actual Y line within the tile, handling Y-flip
        let relative_y = ly.wrapping_sub(self.sprite.y_pos);
        let y_line = if y_flip {
            (sprite_size as u8 - 1).wrapping_sub(relative_y)
        } else {
            relative_y
        };

        let actual_tile = if sprite_size == 16 {
            // For 8x16 sprites, the top tile is the tile number with bit 0 cleared
            self.tile_number & 0b11111110
        } else {
            self.tile_number
        };
        let y_offset = 2 * (y_line % sprite_size) as u16;

        // only 8000 method for sprites
        let base_address = 0x8000 + (actual_tile as u16 * 16);

        // Get the correct byte of tile data
        let address = base_address + y_offset + if is_high_byte { 1 } else { 0 };
        let mut data;
        match gb_mode {
            GameboyMode::DMG => {
                data = vram_banks[0][(address - 0x8000) as usize];
            }
            GameboyMode::CGB => {
                let selected_bank = self.sprite.flags >> 3 & 0b1;
                data = vram_banks[selected_bank as usize][(address - 0x8000) as usize];
            }
        }

        if x_flip {
            data = data.reverse_bits();
        }
        data
    }

    fn push_to_fifo(&self, pixel_fifo: &mut PixelFifo, gb_mode: GameboyMode) {
        for bit in 0..8 {
            let low_bit = (self.tile_data_low >> (7 - bit)) & 0x1;
            let high_bit = (self.tile_data_high >> (7 - bit)) & 0x1;
            let color = (high_bit << 1) | low_bit;

            // Ensure we have space in the sprite FIFO
            while pixel_fifo.sprite_fifo.len() <= bit {
                pixel_fifo
                    .sprite_fifo
                    .push_back(super::pixelfifo::Pixel::new_sprite(
                        0, // Transparent pixel
                        0, // No flags
                        gb_mode,
                    ));
            }

            // Only override existing pixels if the new pixel is not transparent
            if color != 0 {
                if let Some(existing_pixel) = pixel_fifo.sprite_fifo.get_mut(bit) {
                    let new_pixel =
                        super::pixelfifo::Pixel::new_sprite(color, self.sprite.flags, gb_mode);

                    match gb_mode {
                        bus::GameboyMode::DMG => {
                            if existing_pixel.color == 0 {
                                *existing_pixel = new_pixel;
                            }
                        }
                        bus::GameboyMode::CGB => {
                            *existing_pixel = new_pixel;
                        }
                    }
                }
            }
        }
    }
}
