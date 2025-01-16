use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::bus::{io_address::IoRegister, Bus, MemoryInterface};

use super::{
    fetcher::{self, Fetcher},
    Sprite,
};
#[derive(Clone, Debug, Serialize, Deserialize, Copy)]
pub struct Pixel {
    pub color: u8,
    pub sprite_priority: bool, // CGB relevant
    pub bg_priority: bool,
    pub palette: bool, // CGB: 0-7 | DMG: false = OBP0, true = OBP1
}
impl Pixel {
    pub fn new_bg_sprite(color: u8) -> Self {
        Self {
            color,
            sprite_priority: false,
            bg_priority: false,
            palette: false,
        }
    }
    pub fn new_sprite(color: u8, bg_priority: bool, palette: bool) -> Self {
        Self {
            color,
            sprite_priority: false,
            bg_priority: bg_priority,
            palette: palette,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PixelFifo {
    pub bg_fifo: VecDeque<Pixel>,
    pub sprite_fifo: VecDeque<Pixel>,

    fine_scroll_applied: bool,
}

impl PixelFifo {
    pub fn new() -> Self {
        Self {
            bg_fifo: VecDeque::new(),
            sprite_fifo: VecDeque::new(),

            fine_scroll_applied: false,
        }
    }
    pub fn reset(&mut self) {
        self.bg_fifo.clear();
        self.sprite_fifo.clear();
        self.fine_scroll_applied = false;
    }
    pub fn push_bg_pixels(&mut self, tile_data: [u8; 2]) {
        // Decode 2bpp tile data into 8 pixels
        for bit in 0..8 {
            let low_bit = tile_data[0] >> (7 - bit) & 0x1;
            let high_bit = tile_data[1] >> (7 - bit) & 0x1;
            let color = high_bit << 1 | low_bit;
            let pixel = Pixel::new_bg_sprite(color);
            self.bg_fifo.push_back(pixel);
        }
    }
    pub fn push_sprite_pixels(&mut self, tile_data: [u8; 2], sprite: &Sprite) {
        // Instead of clearing, we'll overlay the new sprite pixels
        let current_len = self.sprite_fifo.len();

        for bit in 0..8 {
            let low_bit = tile_data[0] >> (7 - bit) & 0x1;
            let high_bit = tile_data[1] >> (7 - bit) & 0x1;
            let color = high_bit << 1 | low_bit;

            let bg_priority = sprite.flags & 0x80 != 0;
            let palette = sprite.flags & 0x10 != 0;

            let pixel = Pixel::new_sprite(color, bg_priority, palette);

            if bit < current_len {
                // Only replace existing sprite pixel if:
                // 1. New sprite pixel is not transparent (color != 0)
                // 2. The existing sprite pixel is transparent
                let existing_pixel = &self.sprite_fifo[bit];
                if color != 0 && existing_pixel.color == 0 {
                    self.sprite_fifo[bit] = pixel;
                }
            } else if self.sprite_fifo.len() < 8 {
                self.sprite_fifo.push_back(pixel);
            }
        }
    }
    pub fn apply_fine_scroll(&mut self, scx: u8, fetcher: &mut Fetcher) {
        if !self.fine_scroll_applied {
            let fine_scroll_offset = (scx & 0x07) as usize;

            for _ in 0..fine_scroll_offset {
                if !self.bg_fifo.is_empty() {
                    self.bg_fifo.pop_front();
                    fetcher.x_pos_counter += 1;
                }
            }
            self.fine_scroll_applied = true;
        }
    }
    pub fn pop_pixel<M: MemoryInterface>(
        &mut self,
        memory: &M,
        fetcher: &mut Fetcher,
    ) -> Option<u8> {
        if self.bg_fifo.is_empty() {
            return None;
        }
        self.apply_fine_scroll(memory.read_byte(IoRegister::Scx.address()), fetcher);
        let mut bg_pixel = self.bg_fifo.pop_front().unwrap();
        let mut sprite_pixel = self.sprite_fifo.pop_front();

        /*
        Pixel mixing
        1) If the color number of the Sprite Pixel is 0, the Background Pixel is pushed to the LCD.
        2) If the BG-to-OBJ-Priority bit is 1 and the color number of the Background Pixel is anything other than 0, the Background Pixel is pushed to the LCD.
        3) If none of the above conditions apply, the Sprite Pixel is pushed to the LCD.
        */

        let lcdc = memory.read_byte(IoRegister::Lcdc.address());
        if lcdc & 0x01 == 0 {
            // bg/window enable
            bg_pixel.color = 0;
        }

        let bgp = memory.read_byte(IoRegister::Bgp.address());

        let final_color = if let Some(mut sprite) = sprite_pixel {
            // object enable
            if lcdc & 0x02 == 0 {
                sprite.color = 0;
            }
            if sprite.color == 0 {
                (bgp >> (bg_pixel.color * 2)) & 0x03
            } else if sprite.bg_priority && bg_pixel.color != 0 {
                (bgp >> (bg_pixel.color * 2)) & 0x03
            } else {
                // Bit 4 = 1: Use OBP1 | Bit 4 = 0: Use OBP0
                let palette = if sprite.palette {
                    memory.read_byte(IoRegister::Obp1.address())
                } else {
                    memory.read_byte(IoRegister::Obp0.address())
                };
                (palette >> (sprite.color * 2)) & 0x03
            }
        } else {
            (bgp >> (bg_pixel.color * 2)) & 0x03
        };
        Some(final_color)
    }
    pub fn bg_pixel_count(&self) -> usize {
        return self.bg_fifo.len();
    }
    pub fn sprite_pixel_count(&self) -> usize {
        return self.sprite_fifo.len();
    }
    pub fn is_paused(&self, sprite_fetcher_active: bool, fetcher_active: bool) -> bool {
        self.bg_fifo.len() == 0 || sprite_fetcher_active || fetcher_active
    }
}
