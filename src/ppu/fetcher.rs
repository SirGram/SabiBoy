use std::{cell::RefCell, rc::Rc};

use crate::bus::{self, io_address::IoRegister, Bus};

use super::pixelfifo::PixelFifo;

pub struct Fetcher {
    pub step: u8,
    tile_number: u8,
    tile_data_low: u8,
    tile_data_high: u8,
    pub is_window_fetch: bool,

    pub x_pos_counter: u16,
    pub window_line_counter: u16,
    pub pause: bool,
    pub delay: usize,
}
impl Fetcher {
    pub fn new() -> Self {
        Self {
            step: 0,
            tile_number: 0,
            tile_data_low: 0,
            tile_data_high: 0,
            is_window_fetch: false,
            x_pos_counter: 0,
            window_line_counter: 0,
            pause: false,
            delay: 0,
        }
    }
    pub fn pause(&mut self) {
        self.pause = true;
        self.step = 0;
    }
    pub fn unpause(&mut self) {
        self.pause = false;
    }

    pub fn step(&mut self, bus: &Rc<RefCell<Bus>>, pixel_fifo: &mut PixelFifo, mode_cycles: usize) {
        if self.pause {
            return;
        }
        match self.step {
            0 => {
                self.fetch_tile_number(bus);
                self.step += 1;
            }
            1 => {
                self.tile_data_low = self.fetch_tile_data(bus, self.tile_number, false);
                self.step += 1;
            }
            2 => {
                self.tile_data_high = self.fetch_tile_data(bus, self.tile_number, true);
                // Delay of 12 T-cycles before the background FIFO is first filled with pixel data
                if mode_cycles <= 12 {
                    self.step = 0;
                } else {
                    self.step += 1;
                }
            }
            3 => {
                self.push_to_fifo(pixel_fifo);
            }
            _ => self.step = 0,
        }
    }

    fn fetch_tile_number(&mut self, bus: &Rc<RefCell<Bus>>) {
        let lcdc = bus.borrow().read_byte(IoRegister::Lcdc.address());
        let scx = bus.borrow().read_byte(IoRegister::Scx.address());
        let scy = bus.borrow().read_byte(IoRegister::Scy.address());
        let ly = bus.borrow().read_byte(IoRegister::Ly.address());

        // Calculate the tile map coordinates with SCX
        let map_x = (scx as u16 + self.x_pos_counter) >> 3;
        let map_y = if self.is_window_fetch {
            self.window_line_counter >> 3
        } else {
            ((ly as u16 + scy as u16) >> 3) & 0x1F
        };

        // Wrap the x-coordinate properly
        let wrapped_x = map_x & 0x1F;

        // Select the correct tile map base address
        let tile_map_base = if self.is_window_fetch {
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
        };

        // Calculate final address in tile map
        let tile_map_addr = tile_map_base + (map_y as u16 * 32) + wrapped_x as u16;
        self.tile_number = bus.borrow().read_byte(tile_map_addr);
    }
    fn fetch_tile_data(
        &mut self,
        bus: &Rc<RefCell<Bus>>,
        tile_number: u8,
        is_high_byte: bool,
    ) -> u8 {
        let ly = bus.borrow().read_byte(IoRegister::Ly.address());
        let scy = bus.borrow().read_byte(IoRegister::Scy.address());
        let lcdc = bus.borrow().read_byte(IoRegister::Lcdc.address());

        // Calculate the offset within the tile (0-7)
        let y_offset = if self.is_window_fetch {
            (self.window_line_counter & 7) * 2
        } else {
            ((ly as u16 + scy as u16) & 7) * 2
        };

        // LCDC Bit4 selects Tile Data method
        let base_address = if (lcdc & 0x10) != 0 {
            // 8000 method: unsigned addressing
            0x8000 + (tile_number as u16 * 16)
        } else {
            // 8800 method: signed addressing
            0x9000u16.wrapping_add((tile_number as i8 as i16 * 16) as u16)
        };

        // Get the correct byte of tile data
        bus.borrow()
            .read_byte(base_address + y_offset + if is_high_byte { 1 } else { 0 })
    }

    fn push_to_fifo(&mut self, pixel_fifo: &mut PixelFifo) {
        // this step repeats every cycle until it succeeds
        if pixel_fifo.push_bg_pixels([self.tile_data_low, self.tile_data_high]) {
            self.step = 0;
        }
    }
}