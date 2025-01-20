use serde::{Deserialize, Serialize};

use super::{pixelfifo::PixelFifo, Sprite};
use crate::{
    bus::{self, io_address::IoRegister, GameboyMode, MemoryInterface},
    gameboy::Gameboy,
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
    pub fn step<M: MemoryInterface>(&mut self, memory: &mut M, pixel_fifo: &mut PixelFifo) {
        match self.step {
            0 => {
                self.tile_data_low = self.fetch_tile_data(memory, false);
                self.step += 1;
            }
            1 => {
                self.tile_data_high = self.fetch_tile_data(memory, true);
                self.step += 1;
            }
            2 => {
                // Load sprite pixels while preserving existing sprite pixels in FIFO
                self.push_to_fifo(memory, pixel_fifo);

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
    fn fetch_tile_data<M: MemoryInterface>(&mut self, memory: &mut M, is_high_byte: bool) -> u8 {
        let ly = memory.read_byte(IoRegister::Ly.address());

        let y_flip = self.sprite.flags & 0x40 != 0;
        let x_flip = self.sprite.flags & 0x20 != 0;
        let sprite_size = if memory.read_byte(IoRegister::Lcdc.address()) & 0x04 != 0 {
            16
        } else {
            8
        };

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
        match memory.gb_mode() {
            GameboyMode::DMG => {
                data = memory.read_byte(address);
            }
            GameboyMode::CGB => {
                let original_bank = memory.read_byte(0xFF4F);
                let selected_bank = self.sprite.flags > 3 & 1;
                memory.write_byte(0xFF4F, selected_bank as u8);
                data = memory.read_byte(address);
                memory.write_byte(0xFF4F, original_bank as u8);
            }
        }

        if x_flip {
            data = data.reverse_bits();
        }
        data
    }
    fn push_to_fifo<M: MemoryInterface>(&self, memory: &M, pixel_fifo: &mut PixelFifo) {
        for bit in 0..8 {
            let low_bit = (self.tile_data_low >> (7 - bit)) & 0x1;
            let high_bit = (self.tile_data_high >> (7 - bit)) & 0x1;
            let color = (high_bit << 1) | low_bit;

            // Ensure we have space in the sprite FIFO
            while pixel_fifo.sprite_fifo.len() <= bit {
                pixel_fifo
                    .sprite_fifo
                    .push_back(super::pixelfifo::Pixel::new_sprite(
                        memory, 0, // Transparent pixel
                        0, // No flags
                    ));
            }

            // Only override existing pixels if the new pixel is not transparent
            if color != 0 {
                if let Some(existing_pixel) = pixel_fifo.sprite_fifo.get_mut(bit) {
                    let new_pixel = match memory.gb_mode() {
                        bus::GameboyMode::DMG => {
                            super::pixelfifo::Pixel::new_sprite(memory, color, self.sprite.flags)
                        }
                        bus::GameboyMode::CGB => {
                            super::pixelfifo::Pixel::new_sprite(memory, color, self.sprite.flags)
                        }
                    };

                    // Replace the pixel only if:
                    // 1. The existing pixel is transparent (color == 0), or
                    // 2. The new sprite has priority (based on x position)
                    if existing_pixel.color == 0 {
                        *existing_pixel = new_pixel;
                    }
                }
            }
        }
    }
}
