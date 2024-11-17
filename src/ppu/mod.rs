mod fetcher;
mod fetcher_sprites;
mod helper;
mod pixelfifo;

use crate::bus::{io_address::IoRegister, Bus};
use fetcher::Fetcher;
use fetcher_sprites::SpriteFetcher;
use helper::{should_add_sprite, should_fetch_sprite};
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
const CYCLES_PER_SCANLINE: usize = 455;
const X_POSITION_COUNTER_MAX: u16 = 160;
const SCANLINE_Y_COUNTER_MAX: u8 = 153;
const VBLANK_START_SCANLINE: u8 = 144;
const FRAME_DURATION: usize = 70224;

#[derive(Debug, Copy, Clone)]
pub enum PPUMode {
    HBLANK = 0,
    VBLANK = 1,
    OAM_SCAN = 2,
    DRAWING = 3,
}
pub struct PPU {
    pub mode: PPUMode,
    pub mode_cycles: usize,
    pub frame_cycles: usize,

    window: Window,
    buffer: Vec<u32>,

    bus: Rc<RefCell<Bus>>,

    sprite_buffer: Vec<Sprite>,
    fetcher: Fetcher,
    sprite_fetcher: SpriteFetcher,
    pixel_fifo: PixelFifo,
}

#[derive(Clone)]
struct Sprite {
    y_pos: u8,
    x_pos: u8,
    tile_number: u8,
    flags: u8,
}
impl Sprite {
    pub fn new() -> Self {
        Self {
            y_pos: 0,
            x_pos: 0,
            tile_number: 0,
            flags: 0,
        }
    }
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
            mode: PPUMode::OAM_SCAN,
            mode_cycles: 0,
            sprite_buffer: Vec::new(),
            fetcher: Fetcher::new(),
            sprite_fetcher: SpriteFetcher::new(),
            pixel_fifo: PixelFifo::new(),
            frame_cycles: 0,
        }
    }
    pub fn reset_scanline(&mut self) {
        self.mode = PPUMode::OAM_SCAN;
        self.mode_cycles = 0;
        self.sprite_buffer.clear();
        self.fetcher.x_pos_counter = 0;
        self.fetcher.is_window_fetch = false;
    }

    pub fn tick(&mut self) {
        self.frame_cycles = self.frame_cycles.wrapping_add(1);
        // Check if LCD is enabled
        let lcdc = self.get_io_register(IoRegister::Lcdc);
        if (lcdc & 0x80) == 0 {
            return;
        }

        // scanline

        match self.mode {
            PPUMode::OAM_SCAN => self.handle_oam(),
            PPUMode::DRAWING => self.handle_drawing(),
            PPUMode::HBLANK => self.handle_hblank(),
            PPUMode::VBLANK => self.handle_vblank(),
        }

        if self.mode_cycles >= CYCLES_PER_SCANLINE {
            let ly = self.get_io_register(IoRegister::Ly);
            self.mode_cycles = 0;

            // Increment LY and handle PPUMode transitions
            if ly < 143 {
                self.set_io_register(IoRegister::Ly, ly + 1);
                self.mode = PPUMode::OAM_SCAN;
            } else if ly == VBLANK_START_SCANLINE {
                self.set_io_register(IoRegister::Ly, ly + 1);
                self.mode = PPUMode::VBLANK;
            } else if ly >= SCANLINE_Y_COUNTER_MAX {
                self.set_io_register(IoRegister::Ly, 0);
                self.mode = PPUMode::OAM_SCAN; /*
                                               self.window_line_counter = 0;
                                               self.window_triggered_this_frame = false; */
                self.sprite_buffer.clear();
            } else {
                self.set_io_register(IoRegister::Ly, ly + 1);
            }
            println!("IF: {:02X}", self.get_io_register(IoRegister::If));


            // Reset fetcher state for new line
            self.reset_scanline();
        }

        self.update_stat();

        if self.frame_cycles % 1000 == 0 {
            if self.window.is_open() && !self.window.is_key_down(Key::Escape) {
                self.window
                    .limit_update_rate(Some(std::time::Duration::from_micros(16600))); // ~60fps

                self.window
                    .update_with_buffer(&self.buffer, SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize)
                    .unwrap();
                self.bus.borrow_mut().joypad.update(&mut self.window);
            }
        }
    
        self.mode_cycles += 1;
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

        if self.mode_cycles % 2 != 0 {
            let current_entry = self.mode_cycles / 2;
            let sprite = self.read_sprite(0xFE00 + current_entry as u16);
            if should_add_sprite(
                &sprite,
                self.get_io_register(IoRegister::Ly),
                self.get_io_register(IoRegister::Lcdc),
                self.sprite_buffer.len(),
            ) {
                self.sprite_buffer.push(sprite);
            }
        }
        if self.mode_cycles >= 79 {
            self.mode = PPUMode::DRAWING;
            self.fetcher.x_pos_counter = 0;
            self.fetcher.step = 0;
            self.pixel_fifo.reset();
        }
    }
    fn handle_drawing(&mut self) {
        // Exit if we've drawn all pixels for this line
        if self.fetcher.x_pos_counter >= X_POSITION_COUNTER_MAX {
            self.mode = PPUMode::HBLANK;
            return;
        }

        // Check sprites
        if let Some(sprite) = should_fetch_sprite(self.fetcher.x_pos_counter, &self.sprite_buffer) {
            self.sprite_fetcher.start_fetch(&sprite, &mut self.fetcher);
        }

        if self.mode_cycles % 2 == 0 {
            if self.fetcher.pause {
                self.sprite_fetcher
                    .step(&self.bus, &mut self.pixel_fifo, &mut self.fetcher);
            } else if self.fetcher.delay == 0 {
                // only step if there are no BG pixels
                if self.pixel_fifo.bg_pixel_count() == 0 {
                    self.fetcher
                        .step(&self.bus, &mut self.pixel_fifo, self.mode_cycles);
                }
            } else {
                self.fetcher.delay -= 1;
            }
        }

        // Process pixels from FIFO
            if let Some(color) = self.pixel_fifo.pop_pixel() {
                let ly = self.get_io_register(IoRegister::Ly);
                let x_pos = self.fetcher.x_pos_counter as usize;

                // Only draw if within screen bounds
                if x_pos < SCREEN_WIDTH as usize && (ly as usize) < SCREEN_HEIGHT as usize {
                    let buffer_index = ly as usize * SCREEN_WIDTH as usize + x_pos;
                    let color = COLORS[color as usize];
                    self.buffer[buffer_index] = color;
                }

                self.fetcher.x_pos_counter += 1;
            }
    }
    fn handle_hblank(&mut self) {
        // pads till 456 cycles
    }

    fn handle_vblank(&mut self) {
        // pads 10 vertical scanlines
         // request vblank interrupt
         let if_register = self.get_io_register(IoRegister::If);
         self.set_io_register(IoRegister::If, if_register | 0b0000_0001);
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
        Bit 5   PPUMode 2 STAT Interrupt Enable
                 Setting this bit to 1 enables the "PPUMode 2 condition" to trigger a STAT interrupt.
        Bit 4   PPUMode 1 STAT Interrupt Enable
                 Setting this bit to 1 enables the "PPUMode 1 condition" to trigger a STAT interrupt.
        Bit 3   PPUMode 0 STAT Interrupt Enable
                 Setting this bit to 1 enables the "PPUMode 0 condition" to trigger a STAT interrupt.
        Bit 2   Coincidence Flag
                 This bit is set by the PPU if the value of the LY register is equal to that of the LYC register.
        Bit 1-0 PPU PPUMode
                 These two bits are set by the PPU depending on which PPUMode it is in.
                  * 0 : H-Blank
                  * 1 : V-Blank
                  * 2 : OAM Scan
                  * 3 : Drawing
        */
        let mut stat = self.get_io_register(IoRegister::Stat);

        let PPUMode = match self.mode {
            PPUMode::HBLANK => 0b00,
            PPUMode::VBLANK => 0b01,
            PPUMode::OAM_SCAN => 0b10,
            PPUMode::DRAWING => 0b11,
        };
        stat &= 0b11111100;
        stat |= PPUMode;

        // Update coincidence flag
        let ly = self.get_io_register(IoRegister::Ly);
        let lyc = self.get_io_register(IoRegister::Lyc);
        let coincidence_flag = if ly == lyc { 1 } else { 0 };
        stat &= 0b11111011;
        stat |= coincidence_flag << 2;

        // Update interrupt enable bits
        let interrupt_enable = self.get_io_register(IoRegister::Stat) & 0b11110000;

        // Set interrupt flag based on enabled interrupts and current PPUMode
        let mut interrupt_flag = 0;
        if (interrupt_enable & 0b10000) != 0 && PPUMode == 0b00 {
            interrupt_flag |= 0b10;
        }
        if (interrupt_enable & 0b01000) != 0 && PPUMode == 0b01 {
            interrupt_flag |= 0b10;
        }
        if (interrupt_enable & 0b00100) != 0 && PPUMode == 0b10 {
            interrupt_flag |= 0b10;
        }
        if (interrupt_enable & 0b00010) != 0 && coincidence_flag == 1 {
            interrupt_flag |= 0b10;
        }

        self.set_io_register(IoRegister::Stat, stat | interrupt_enable | interrupt_flag);
    }
}
