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

    pub fn start_fetch(&mut self, sprite: &Sprite) {
        self.step = 0;
        self.sprite = sprite.clone();
        self.active = true;
        self.remaining_pixels = 8;
    }

    pub fn step(
        &mut self,
        bus: &Rc<RefCell<bus::Bus>>,
        pixel_fifo: &mut PixelFifo,
        fetcher: &mut Fetcher,
    ) {
        match self.step {
            0 => {
                self.fetch_tile_number();
                self.step += 1;
            }
            1 => {
                self.tile_data_low = self.fetch_tile_data(bus, false);
                self.step += 1;
            }
            2 => {
                self.tile_data_high = self.fetch_tile_data(bus, true);
                self.step += 1;
            }
            3 => {
                self.push_to_fifo(pixel_fifo, bus);
                self.active = false;
            }
            _ => {}
        }
    }

    fn fetch_tile_number(&mut self) {
        self.tile_number = self.sprite.tile_number;
    }

    fn fetch_tile_data(&mut self, bus: &Rc<RefCell<bus::Bus>>, is_high_byte: bool) -> u8 {
        let ly = bus.borrow().read_byte(IoRegister::Ly.address());
        let lcdc = bus.borrow().read_byte(IoRegister::Lcdc.address());
        let y_flip = self.sprite.flags & 0x40 != 0;
        let x_flip = self.sprite.flags & 0x20 != 0;
        
        // Calculate sprite height (8 or 16 based on LCDC)
        let sprite_height = if lcdc & 0x04 != 0 { 16 } else { 8 };
        
        // Calculate Y offset within the sprite
        let mut y_offset = ly.wrapping_add(16).wrapping_sub(self.sprite.y_pos);
        
        // Handle Y-flip
        if y_flip {
            y_offset = ((sprite_height - 1) as u8).wrapping_sub(y_offset as u8) as u8;
        }
        
        // For 8x16 sprites, adjust tile number and y_offset
        let adjusted_tile_number = if sprite_height == 16 {
            let tile_base = self.tile_number & 0xFE; // Clear lowest bit
            if y_offset >= 8 {
                tile_base + 1 // Use next tile for bottom half
            } else {
                tile_base // Use base tile for top half
            }
        } else {
            self.tile_number
        };
        
        // Ensure y_offset is within the current tile (0-7)
        y_offset &= 7;
        
        // Calculate tile data address (sprites always use 8000 addressing mode)
        let tile_address = 0x8000 + ((adjusted_tile_number as u16) * 16);
        let data_address = tile_address + ((y_offset as u16) * 2) + (if is_high_byte { 1 } else { 0 });
        
        // Fetch the data
        let mut data = bus.borrow().read_byte(data_address);
        
        // Handle X-flip
        if x_flip {
            data = data.reverse_bits();
        }
        
        data
    }

    fn push_to_fifo(&mut self, pixel_fifo: &mut PixelFifo, bus: &Rc<RefCell<bus::Bus>>) {
        let x_pos = self.sprite.x_pos;
        let start_pixel = if x_pos < 8 { 8 - x_pos } else { 0 };
        let bg_priority = self.sprite.flags & 0x80 != 0;
        let palette_number = self.sprite.flags & 0x10 != 0;
        
        pixel_fifo.push_sprite_pixels(
            [self.tile_data_low, self.tile_data_high],
            start_pixel,
            self.remaining_pixels,
            palette_number,
            bg_priority,
        );
    }
}
