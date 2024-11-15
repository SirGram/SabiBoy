mod helper;
mod pixelfifo;

use crate::bus::{io_address::IoRegister, Bus};
use helper::should_add_sprite;
use minifb::{Key, Scale, Window, WindowOptions};
use pixelfifo::PixelFifo;
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
    vec,
};

pub const COLORS: [u32; 4] = [0xA8D08D, 0x6A8E3C, 0x3A5D1D, 0x1F3C06];

const SCREEN_WIDTH: u8 = 160;
const SCREEN_HEIGHT: u8 = 144;
const CYCLES_PER_SCANLINE: usize = 456;
const X_POSITION_COUNTER_MAX: u16 = 160;
const SCANLINE_Y_COUNTER_MAX: u8 = 153;
const VBLANK_START_SCANLINE: u8 = 143;
const FRAME_DURATION: usize = 70224;

#[derive(Debug)]
enum Mode {
    HBLANK = 0,
    VBLANK = 1,
    OAM_SCAN = 2,
    DRAWING = 3,
}
pub struct PPU {
    window: Window,
    buffer: Vec<u32>,
    bus: Rc<RefCell<Bus>>,

    mode: Mode,
    mode_cycles: usize,
    frame_cycles: usize,

    sprite_buffer: Vec<Sprite>,

    fetcher: Fetcher,
    pixel_fifo: PixelFifo,

}

pub struct Fetcher {
    step: u8,
    tile_number: u8,
    tile_data_low: u8,
    tile_data_high: u8,
    is_window_fetch: bool,

    x_pos_counter: u16,
    window_line_counter: u16,
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
        }
    }
}

struct Sprite {
    y_pos: u8,
    x_pos: u8,
    tile_number: u8,
    flags: u8,
}

impl PPU {
    /* https://hacktix.github.io/GBEDG/ppu/
     */
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        let window = Window::new(
            "SabiBoy",
            SCREEN_WIDTH as usize,
            SCREEN_HEIGHT as usize,
            WindowOptions {
                scale: Scale::X2,
                ..WindowOptions::default()
            },
        )
        .unwrap();

        Self {
            window,
            buffer: vec![0; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize],
            bus: bus,
            mode: Mode::OAM_SCAN,
            mode_cycles: 0,
            sprite_buffer: Vec::new(),
            fetcher: Fetcher::new(),
            pixel_fifo: PixelFifo::new(),

            frame_cycles: 0,
        }
    }
    pub fn reset(&mut self) {
        self.mode = Mode::OAM_SCAN;
        self.mode_cycles = 0;
        self.sprite_buffer.clear();
        self.pixel_fifo.reset();
        self.set_io_register(IoRegister::Ly, 0);
    }


    pub fn tick(&mut self) {
        self.frame_cycles += 1;
        // Check if LCD is enabled
        let lcdc = self.get_io_register(IoRegister::Lcdc);
        if (lcdc & 0x80) == 0 {
            return;
        }

        // scanline
        self.mode_cycles += 1;

        match self.mode {
            Mode::OAM_SCAN => self.handle_oam(),
            Mode::DRAWING => self.handle_drawing(),
            Mode::HBLANK => self.handle_hblank(),
            Mode::VBLANK => self.handle_vblank(),
        }

        if self.mode_cycles >= CYCLES_PER_SCANLINE {
            self.mode_cycles = 0;
            let ly = self.get_io_register(IoRegister::Ly);

            // Increment LY and handle mode transitions
            if ly < 143 {
                self.set_io_register(IoRegister::Ly, ly + 1);
                self.mode = Mode::OAM_SCAN;
            } else if ly == VBLANK_START_SCANLINE {
                self.set_io_register(IoRegister::Ly, ly + 1);
                self.mode = Mode::VBLANK;
            } else if ly >= SCANLINE_Y_COUNTER_MAX {
                self.set_io_register(IoRegister::Ly, 0);
                self.mode = Mode::OAM_SCAN; /*
                                            self.window_line_counter = 0;
                                            self.window_triggered_this_frame = false; */
            } else {
                self.set_io_register(IoRegister::Ly, ly + 1);
            }

            // Reset fetcher state for new line
            self.fetcher.x_pos_counter = 0;
            self.fetcher.is_window_fetch = false;
            self.fetcher.step = 0; /*
                                   self.pixel_fifo.clear(); */
        }

        self.update_stat();

        if self.frame_cycles >= 1000 {
            self.frame_cycles = 0;
            if self.window.is_open() && !self.window.is_key_down(Key::Escape) {
                self.window
                    .update_with_buffer(&self.buffer, SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize)
                    .unwrap();
            }
        }
    }
    fn read_sprite(&self, address: u16) -> Sprite {
        Sprite {
            y_pos: self.bus.borrow().read_byte(address),
            x_pos: self.bus.borrow().read_byte(address + 1),
            tile_number: self.bus.borrow().read_byte(address + 2),
            flags: self.bus.borrow().read_byte(address + 3),
        }
    }

    fn handle_oam(&mut self) {
        /* Add sprites to buffer
        80 T-cycles  / 40 sprites (1 sprite is 4 bytes) = 1 sprite per 2 cycles
        */
        if self.mode_cycles >= 80 {
            self.mode = Mode::DRAWING;
            self.fetcher.x_pos_counter = 0;
            return;
        }
        if self.mode_cycles == 0 {
            self.sprite_buffer.clear();
        }
        if self.mode_cycles % 2 != 0 {
            let current_entry = self.mode_cycles / 2;
            let sprite = self.read_sprite(0xFE00 + current_entry as u16);
            if should_add_sprite(
                &sprite,
                self.get_io_register(IoRegister::Ly),
                self.sprite_buffer.len(),
            ) {
                self.sprite_buffer.push(sprite);
            }
        }
    }
    fn handle_hblank(&mut self) {
        // pads till 456 cycles
    }

    fn handle_vblank(&mut self) {
        // pads 10 vertical scanlines
    }

    fn fetch_tile_number(&mut self) {
        let lcdc = self.get_io_register(IoRegister::Lcdc);
        let scx = self.get_io_register(IoRegister::Scx);
        let scy = self.get_io_register(IoRegister::Scy);
        let ly = self.get_io_register(IoRegister::Ly);
        let wx = self.get_io_register(IoRegister::Wx);
        let wy = self.get_io_register(IoRegister::Wy);
/* 
        // Check if window is enabled and should be drawn
        let window_enabled = (lcdc & 0x20) != 0;
        let window_visible = window_enabled && wx <= 166 && wy <= ly;
        
        self.fetcher.is_window_fetch = window_visible && 
            (self.fetcher.x_pos_counter + 7) >= (wx as u16 - 7); */

        // Calculate the actual pixel position
        let pixel_x = self.fetcher.x_pos_counter.wrapping_add(scx as u16);
        let pixel_y = if self.fetcher.is_window_fetch {
            self.fetcher.window_line_counter
        } else {
            ly as u16 + scy as u16
        };

        // Calculate tile map position (32x32 tiles)
        let tile_x = (pixel_x / 8) & 0x1F; // Wrap around at 32 tiles
        let tile_y = (pixel_y / 8) & 0x1F;

        // Select the correct tile map base address
        let tile_map_base = if self.fetcher.is_window_fetch {
            if (lcdc & 0x40) != 0 { 0x9C00 } else { 0x9800 }
        } else {
            if (lcdc & 0x08) != 0 { 0x9C00 } else { 0x9800 }
        };

        // Calculate final address in tile map
        let tile_map_addr = tile_map_base + tile_y as u16 * 32 + tile_x as u16;
        self.fetcher.tile_number = self.bus.borrow().read_byte(tile_map_addr);
    }

    fn fetch_tile_data(&mut self, tile_number: u8, is_high_byte: bool) -> u8 {
        let ly = self.get_io_register(IoRegister::Ly);
        let scy = self.get_io_register(IoRegister::Scy);
        let lcdc = self.get_io_register(IoRegister::Lcdc);

        // Calculate the offset within the tile
        let offset = if self.fetcher.is_window_fetch {
            (self.fetcher.window_line_counter % 8) * 2
        } else {
            ((ly as u16 + scy as u16) % 8) * 2
        };
        // LCDC Bit4 selects Tile Data method 8000 or 8800
        let use_8000_method = (lcdc & 0x10) != 0;

        let base_address = if use_8000_method {
            // 8000 method: unsigned addressing
            0x8000 + (tile_number as u16 * 16)
        } else {
            // 8800 method: signed addressing
            let signed_tile_number = tile_number as i8;
            0x9000u16.wrapping_add((signed_tile_number as i16 * 16) as u16)
        };

        // Return the specific byte within the tile data based on the offset
        if is_high_byte {
            self.bus.borrow().read_byte(base_address + offset + 1)
        }else{

            self.bus.borrow().read_byte(base_address + offset)
        }
    }
    fn push_to_fifo(&mut self) {
        if self
            .pixel_fifo
            .push_bg_pixels([ self.fetcher.tile_data_low,self.fetcher.tile_data_high])
        {
            self.fetcher.step = 0;
            
        }
    }
    fn fetcher_step(&mut self) {
        match self.fetcher.step {
            0 => {
                self.fetch_tile_number();
                self.fetcher.step += 1;
            }
            1 => {
                self.fetcher.tile_data_low = self.fetch_tile_data(self.fetcher.tile_number, false);
                self.fetcher.step += 1;
            }
            2 => {
                self.fetcher.tile_data_high = self.fetch_tile_data(self.fetcher.tile_number, true );
                // Delay of 12 T-cycles before the background FIFO is first filled with pixel data
                if self.mode_cycles < 12 {
                    self.fetcher.step = 0;
                } else {
                    self.fetcher.step += 1;
                }
            }
            3 => {
                self.push_to_fifo();
            }
            _ => self.fetcher.step = 0,
        }
    }

    fn handle_drawing(&mut self) {
        // Exit if we've drawn all pixels for this line
        if self.fetcher.x_pos_counter >= X_POSITION_COUNTER_MAX {
            self.mode = Mode::HBLANK;
            return;
        }

        // Fetcher runs every 2 cycles
        if self.mode_cycles % 2 == 0 {
            self.fetcher_step();
        }

        // Process pixels from FIFO
        if let Some(color) = self.pixel_fifo.pop_pixel() {
            let bgp = self.get_io_register(IoRegister::Bgp);
            // Apply BGP palette transformation
            let palette_color = (bgp >> (color * 2)) & 0x3;
            
            let ly = self.get_io_register(IoRegister::Ly);
            let x_pos = self.fetcher.x_pos_counter as usize;

            // Only draw if within screen bounds
            if x_pos < SCREEN_WIDTH as usize && (ly as usize) < SCREEN_HEIGHT as usize {
                let buffer_index = ly as usize * SCREEN_WIDTH as usize + x_pos;
                self.buffer[buffer_index] = COLORS[palette_color as usize];
            }

            self.fetcher.x_pos_counter += 1;
        }
    }
    fn get_io_register(&self, register: IoRegister) -> u8 {
        self.bus.borrow().read_byte(register.address())
    }
    fn set_io_register(&self, register: IoRegister, value: u8) {
        self.bus.borrow_mut().write_byte(register.address(), value);
    }
    fn update_stat(&mut self) {
        /*
        Bit 7   Unused (Always 1)
        Bit 6   LYC=LY STAT Interrupt Enable
                 Setting this bit to 1 enables the "LYC=LY condition" to trigger a STAT interrupt.
        Bit 5   Mode 2 STAT Interrupt Enable
                 Setting this bit to 1 enables the "mode 2 condition" to trigger a STAT interrupt.
        Bit 4   Mode 1 STAT Interrupt Enable
                 Setting this bit to 1 enables the "mode 1 condition" to trigger a STAT interrupt.
        Bit 3   Mode 0 STAT Interrupt Enable
                 Setting this bit to 1 enables the "mode 0 condition" to trigger a STAT interrupt.
        Bit 2   Coincidence Flag
                 This bit is set by the PPU if the value of the LY register is equal to that of the LYC register.
        Bit 1-0 PPU Mode
                 These two bits are set by the PPU depending on which mode it is in.
                  * 0 : H-Blank
                  * 1 : V-Blank
                  * 2 : OAM Scan
                  * 3 : Drawing
        */
        let mut stat = self.get_io_register(IoRegister::Stat);

        let mode = match self.mode {
            Mode::HBLANK => 0b00,
            Mode::VBLANK => 0b01,
            Mode::OAM_SCAN => 0b10,
            Mode::DRAWING => 0b11,
        };
        stat &= 0b11111100;
        stat |= mode;

        // Update coincidence flag
        let ly = self.get_io_register(IoRegister::Ly);
        let lyc = self.get_io_register(IoRegister::Lyc);
        let coincidence_flag = if ly == lyc { 1 } else { 0 };
        stat &= 0b11111011;
        stat |= coincidence_flag << 2;

        // Update interrupt enable bits
        let interrupt_enable = self.get_io_register(IoRegister::Stat) & 0b11110000;

        // Set interrupt flag based on enabled interrupts and current mode
        let mut interrupt_flag = 0;
        if (interrupt_enable & 0b10000) != 0 && mode == 0b00 {
            interrupt_flag |= 0b10;
        }
        if (interrupt_enable & 0b01000) != 0 && mode == 0b01 {
            interrupt_flag |= 0b10;
        }
        if (interrupt_enable & 0b00100) != 0 && mode == 0b10 {
            interrupt_flag |= 0b10;
        }
        if (interrupt_enable & 0b00010) != 0 && coincidence_flag == 1 {
            interrupt_flag |= 0b10;
        }

        self.set_io_register(IoRegister::Stat, stat | interrupt_enable | interrupt_flag);
    }
}
