use std::collections::VecDeque;

pub struct Pixel {
    color: u8,
    sprite_priority: bool, // CGB relevant
    bg_priority: bool,
    palette: bool, // CGB: 0-7
}
impl Pixel {
    pub fn new(color: u8) -> Self {
        Self {
            color,
            sprite_priority: false,
            bg_priority: false,
            palette: false,
        }
    }
}

pub struct PixelFifo {
    bg_fifo: VecDeque<Pixel>,
    sprite_fifo: VecDeque<Pixel>,
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
        // Push only if it's empty
        if !self.bg_fifo.is_empty() {
            return;
        }

        // Decode 2bpp tile data into 8 pixels
        for bit in 0..8 {
            let low_bit = tile_data[0] >> (7 - bit) & 0x1;
            let high_bit = tile_data[1] >> (7 - bit) & 0x1;
            let color = high_bit << 1 | low_bit;
            self.bg_fifo.push_back(Pixel::new(color));
        }
    }
    pub fn push_sprite_pixels(&mut self) {}
    pub fn pop_pixel(&mut self) -> Option<u8> {
        if self.bg_fifo.is_empty() {
            return None;
        };

        let bg_pixel = self.bg_fifo.pop_front().unwrap();
        let sprite_pixel = self.sprite_fifo.pop_front();

        /*
        Pixel mixing
        1) If the color number of the Sprite Pixel is 0, the Background Pixel is pushed to the LCD.
        2) If the BG-to-OBJ-Priority bit is 1 and the color number of the Background Pixel is anything other than 0, the Background Pixel is pushed to the LCD.
        3) If none of the above conditions apply, the Sprite Pixel is pushed to the LCD.
        */
        match sprite_pixel {
            Some(sprite) => {
                if sprite.color == 0 {
                    Some(bg_pixel.color)
                } else if sprite.bg_priority && bg_pixel.color != 0 {
                    Some(bg_pixel.color)
                } else {
                    Some(sprite.color)
                }
            }
            None => Some(bg_pixel.color),
        }
    }
}