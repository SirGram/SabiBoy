use super::{fetcher::Fetcher, pixelfifo::PixelFifo, Sprite};
use crate::bus::{self, io_address::IoRegister};
use std::{cell::RefCell, rc::Rc};
pub struct SpriteFetcher {
    step: u8,
    tile_number: u8,
    tile_data_low: u8,
    tile_data_high: u8,
    pub active: bool,
    remaining_pixels: u8,
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
        // Calculate how many pixels to load based on sprite's X position
        self.remaining_pixels = if sprite.x_pos >= 8 { 8 } else { sprite.x_pos };
        // Start with tile number fetch
        self.fetch_tile_number(sprite);
    }
    pub fn step(&mut self, bus: &Rc<RefCell<bus::Bus>>, pixel_fifo: &mut PixelFifo) {
        match self.step {
            0 => {
                self.tile_data_low = self.fetch_tile_data(bus, false);
                self.step += 1;
            }
            1 => {
                self.tile_data_high = self.fetch_tile_data(bus, true);
                self.step += 1;
            }
            2 => {
                // Load sprite pixels while preserving existing sprite pixels in FIFO
                self.push_to_fifo(pixel_fifo);

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
    fn fetch_tile_data(&mut self, bus: &Rc<RefCell<bus::Bus>>, is_high_byte: bool) -> u8 {
        let ly = bus.borrow().read_byte(IoRegister::Ly.address());
        let scy = bus.borrow().read_byte(IoRegister::Scy.address());

        let y_flip = self.sprite.flags & 0x40 != 0;
        let x_flip = self.sprite.flags & 0x20 != 0;
        let sprite_size = if bus.borrow().read_byte(IoRegister::Lcdc.address()) & 0x04 != 0 { 16 } else { 8 };

         // Calculate the actual Y line within the tile, handling Y-flip
         let relative_y = ly.wrapping_sub(self.sprite.y_pos);
         let mut y_line = if y_flip {
            (sprite_size  as u8 - 1).wrapping_sub(relative_y)
        } else {
            relative_y
        };

        // For 8x16 sprites, we need to:
        // 1. Handle which tile we're using (top or bottom)
        // 2. Adjust the y_line for the bottom tile
        let actual_tile = if sprite_size == 16 {
            let is_bottom_half = y_line >= 8;
            if is_bottom_half {
                y_line -= 8;
                // For 8x16 sprites, the bottom tile is the next tile
                self.tile_number | 1
            } else {
                // For 8x16 sprites, the top tile is the tile number with bit 0 cleared
                self.tile_number & !1
            }
        } else {
            self.tile_number
        };
        let y_offset = (y_line as u16 & 7) * 2;


        // only 8000 method for sprites
        let base_address = 0x8000 + (actual_tile as u16 * 16);

        // Get the correct byte of tile data
        let mut data = bus
            .borrow()
            .read_byte(base_address + y_offset + if is_high_byte { 1 } else { 0 });

        if x_flip {
            data = data.reverse_bits();
        }
        data
    }
    fn push_to_fifo(&mut self, pixel_fifo: &mut PixelFifo) {
        let mut pixels = [self.tile_data_low, self.tile_data_high];

        pixel_fifo.push_sprite_pixels(pixels, &self.sprite);
    }
}