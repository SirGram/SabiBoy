use std::{cell::RefCell, rc::Rc};

use crate::bus::{self, io_address::IoRegister};

use super::{fetcher::Fetcher, pixelfifo::PixelFifo, Sprite};

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
    pub fn start_fetch(&mut self, sprite: &Sprite, fetcher: &mut Fetcher) {
        self.step = 0;
        self.sprite = sprite.clone();
        self.active = true;

        // Calculate how many pixels to load based on sprite's X position
        self.remaining_pixels = if sprite.x_pos >= 8 { 8 } else { sprite.x_pos };

        // Reset and pause background fetcher
        fetcher.step = 1; // Reset to step 1 specifically
        fetcher.pause = true;

        // Start with tile number fetch
        self.fetch_tile_number(sprite);
    }

    pub fn step(
        &mut self,
        bus: &Rc<RefCell<bus::Bus>>,
        pixel_fifo: &mut PixelFifo,
        fetcher: &mut Fetcher,
    ) {
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

                // Resume background fetching
                fetcher.pause = false;

                // Calculate delay based on remaining background pixels
                let bg_pixels_remaining = pixel_fifo.bg_pixel_count();
                if bg_pixels_remaining < 6 {
                    fetcher.delay = 6 - bg_pixels_remaining;
                }

                self.step = 0;
                self.active = false;
            }
            _ => {
                fetcher.pause = false;
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

        // Calculate the offset within the tile (0-7)
        let y_offset = ((ly as u16 + scy as u16) & 7) * 2;

        // Sprite always 8000 method
        let base_address = 0x8000 + (self.tile_number as u16 * 16);

        // Get the correct byte of tile data
        bus.borrow()
            .read_byte(base_address + y_offset + if is_high_byte { 1 } else { 0 })
    }
    fn push_to_fifo(&mut self, pixel_fifo: &mut PixelFifo) {
        let mut pixels = [self.tile_data_low, self.tile_data_high];
        let start_pixel = if self.sprite.x_pos < 8 {
            8 - self.sprite.x_pos
        } else {
            0
        };

        let bg_priority = self.sprite.flags & 0b1000_0000 != 0;
        let palette_number = self.sprite.flags & 0b0001_0000 != 0;

        pixel_fifo.push_sprite_pixels(pixels, start_pixel, self.remaining_pixels, palette_number, bg_priority);
    }
}
