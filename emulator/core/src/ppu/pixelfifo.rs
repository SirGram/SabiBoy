use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::bus::{io_address::IoRegister, Bus};

use super::{fetcher_sprites::SpriteFetcher, Sprite};

pub struct Pixel {
    color: u8,
    sprite_priority: bool, // CGB relevant
    bg_priority: bool,
    palette: bool, // CGB: 0-7 | DMG: false = OBP0, true = OBP1
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

pub struct PixelFifo {
    pub bg_fifo: VecDeque<Pixel>,
    pub sprite_fifo: VecDeque<Pixel>,
}

impl PixelFifo {
    pub fn new() -> Self {
        Self {
            bg_fifo: VecDeque::new(),
            sprite_fifo: VecDeque::new(),
        }
    }
    pub fn reset(&mut self) {
        self.bg_fifo.clear();
        self.sprite_fifo.clear();
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
        for bit in 0..8 {
            let low_bit = tile_data[0] >> (7 - bit) & 0x1;
            let high_bit = tile_data[1] >> (7 - bit) & 0x1;
            let color = high_bit << 1 | low_bit;

            let bg_priority = sprite.flags & 0x80 != 0;
            let palette = sprite.flags & 0x10 != 0;

            let pixel = Pixel::new_sprite(color, bg_priority, palette);

            if self.sprite_fifo.len() < 8 {
                self.sprite_fifo.push_back(pixel);
            }
        }
    }
    pub fn pop_pixel(&mut self, bus: &Rc<RefCell<Bus>>) -> Option<u8> {
        /* if self.bg_fifo.is_empty() {
            return None;
        } */
        let mut bg_pixel = self.bg_fifo.pop_front().unwrap();
        let mut sprite_pixel = self.sprite_fifo.pop_front();

        /*
        Pixel mixing
        1) If the color number of the Sprite Pixel is 0, the Background Pixel is pushed to the LCD.
        2) If the BG-to-OBJ-Priority bit is 1 and the color number of the Background Pixel is anything other than 0, the Background Pixel is pushed to the LCD.
        3) If none of the above conditions apply, the Sprite Pixel is pushed to the LCD.
        */

        let lcdc = bus.borrow().read_byte(IoRegister::Lcdc.address());
        if lcdc & 0x01 == 0 {
            // bg/window enable
            bg_pixel.color = 0;
        }

        let bgp = bus.borrow().read_byte(IoRegister::Bgp.address());

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
                    bus.borrow().read_byte(IoRegister::Obp1.address())
                } else {
                    bus.borrow().read_byte(IoRegister::Obp0.address())
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
    pub fn is_paused(&self, sprite_fetcher: &SpriteFetcher) -> bool {
        self.bg_fifo.len() == 0 || sprite_fetcher.active
    }
}
