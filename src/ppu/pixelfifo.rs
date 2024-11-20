use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::bus::{io_address::IoRegister, Bus};

use super::fetcher_sprites::SpriteFetcher;

pub struct Pixel {
    color: u8,
    sprite_priority: bool, // CGB relevant
    bg_priority: bool,
    palette: bool, // CGB: 0-7
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
    pub fn push_bg_pixels(&mut self, tile_data: [u8; 2]) -> bool {
        // Decode 2bpp tile data into 8 pixels
        for bit in 0..8 {
            let low_bit = tile_data[0] >> (7 - bit) & 0x1;
            let high_bit = tile_data[1] >> (7 - bit) & 0x1;
            let color = high_bit << 1 | low_bit;
            let pixel = Pixel::new_bg_sprite(color);
            self.bg_fifo.push_back(pixel);
        }
        true
    }
    pub fn push_sprite_pixels(
        &mut self,
        tile_data: [u8; 2],
        start_pixel: u8,
        remaining_pixels: u8,
        palette: bool,
        bg_priority: bool,
    ) -> bool {
        if start_pixel >= 8 || remaining_pixels == 0 {
            return false;
        }
        // Decode 2bpp tile data into 8 pixels
        for bit in start_pixel..std::cmp::min(8, start_pixel + remaining_pixels) {
            let low_bit = tile_data[0] >> (7 - bit) & 0x1;
            let high_bit = tile_data[1] >> (7 - bit) & 0x1;
            let color = high_bit << 1 | low_bit;
            if self.sprite_fifo.len() <= (bit - start_pixel) as usize {
                let pixel = Pixel::new_sprite(color, bg_priority, palette);
                self.sprite_fifo.push_back(pixel);
            }
        }
        true
    }
    pub fn pop_pixel(&mut self, bus: &Rc<RefCell<Bus>>) -> Option<u8> {
        if self.bg_fifo.is_empty() {
            return None;
        }
        let mut bg_pixel = self.bg_fifo.pop_front().unwrap();
        let sprite_pixel = self.sprite_fifo.pop_front();

        /*
        Pixel mixing
        1) If the color number of the Sprite Pixel is 0, the Background Pixel is pushed to the LCD.
        2) If the BG-to-OBJ-Priority bit is 1 and the color number of the Background Pixel is anything other than 0, the Background Pixel is pushed to the LCD.
        3) If none of the above conditions apply, the Sprite Pixel is pushed to the LCD.
        */
    
        let lcdc = bus.borrow().read_byte(IoRegister::Lcdc.address());
        if lcdc & 0x01 == 0 {
            // bg/window enable
            bg_pixel.color = 4;
        }
        match sprite_pixel {
            Some(sprite) => {
                if sprite.color == 0 {
                    Some(bg_pixel.color)
                } else if sprite.bg_priority && bg_pixel.color != 0 {
                    Some(bg_pixel.color)
                } else {
                   /*  Some(sprite.color) */
                      Some(4) 
                }
            }
            None => Some(bg_pixel.color),
        }
    }
    pub fn bg_pixel_count(&self) -> usize {
        return self.bg_fifo.len();
    }
    pub fn is_paused(&self, sprite_fetcher: &SpriteFetcher) -> bool {
        self.bg_fifo.is_empty() || sprite_fetcher.active
    }
}
