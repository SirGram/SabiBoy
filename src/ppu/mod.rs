mod helper;
mod pixelfifo;

use crate::bus::{io_address::IoRegister, Bus};
use helper::should_add_sprite;
use minifb::{Key, Scale, Window, WindowOptions};
use pixelfifo::PixelFifo;
use std::{
    cell::{Ref, RefCell},
    char::MAX,
    f32::consts::E,
    io::Write,
    rc::Rc,
    vec,
};

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

    debug_string: String,
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

            debug_string: String::with_capacity(100),
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
    pub fn update_debug_string(&mut self) {
        // Update debug string
        self.debug_string.clear();
        self.debug_string.push_str(&format!(
            "\rMode: {:?} | Cycles: {} | X_pos: {} | LY: {} | Fetcher Step: {} | Sprites: {}{}",
            self.mode,
            self.mode_cycles,
            self.fetcher.x_pos_counter,
            self.get_io_register(IoRegister::Ly),
            self.fetcher.step,
            self.sprite_buffer.len(),
            " ".repeat(20) // Padding to ensure old text is overwritten
        ));

        // Print the debug string
        print!("{}", self.debug_string);
        std::io::stdout().flush().unwrap();
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
        self.update_debug_string();

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

        if self.frame_cycles >= 100 {
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
        /*
        1) Fetch Tile No.: During the first step the fetcher fetches and stores the tile number of the tile which should be used.
        Which Tilemap is used depends on whether the PPU is currently rendering Background or Window pixels and on the bits 3 and 5 of the LCDC register.
        Additionally, the address which the tile number is read from is offset by the fetcher-internal X-Position-Counter, which is incremented each time the last step is completed.
        The value of SCX / 8 is also added if the Fetcher is not fetching Window pixels. In order to make the wrap-around with SCX work, this offset is ANDed with 0x1f.
        An offset of 32 * (((LY + SCY) & 0xFF) / 8) is also added if background pixels are being fetched, otherwise, if window pixels are being fetched, this offset is determined by 32 * (WINDOW_LINE_COUNTER / 8). The Window Line Counter is a fetcher-internal variable which is incremented each time a scanline had any window pixels on it and reset when entering VBlank mode.

        Note: The sum of both the X-POS+SCX and LY+SCY offsets is ANDed with 0x3ff in order to ensure that the address stays within the Tilemap memory regions.
        */
        let lcdc = self.get_io_register(IoRegister::Lcdc);
        let is_window = lcdc & 0x20 != 0;
        self.fetcher.is_window_fetch = is_window;
        let base_address: u16 = if is_window {
            // Bit 6  Window Tile Map Select
            if lcdc & 0x40 != 0 {
                0x9C00
            } else {
                0x9800
            }
        } else {
            // Bit 3 BG Tile Map Select
            if lcdc & 0x08 != 0 {
                0x9C00
            } else {
                0x9800
            }
        };
        let x_offset = if is_window {
            self.fetcher.x_pos_counter
        } else {
            (self.fetcher.x_pos_counter + (self.get_io_register(IoRegister::Scx) as u16 / 8)) & 0x1F
        };

        let y_offset = if is_window {
            32 * (self.fetcher.window_line_counter / 8)
        } else {
            32 * (((self.get_io_register(IoRegister::Ly) + self.get_io_register(IoRegister::Scy))
                & 0xFF)
                / 8) as u16
        };
        let address = (base_address + x_offset + y_offset) & 0x3FF;

        self.fetcher.tile_number = self.bus.borrow().read_byte(address)
    }

    fn fetch_tile_data(&mut self, tile_number: u8) -> u8 {
        let offset = if !self.fetcher.is_window_fetch {
            2 * ((self.get_io_register(IoRegister::Ly) + self.get_io_register(IoRegister::Scy)) % 8)
                as u16
        } else {
            2 * (self.fetcher.window_line_counter % 8) as u16
        };
        // LCDC Bit4 selects Tile Data method 8000 or 8800
        let lcdc = self.get_io_register(IoRegister::Lcdc);
        let use_8000_method = (lcdc & 0x10) != 0;

        let base_address = if use_8000_method { 0x8000 } else { 0x9000 };
        let tile_data_address = if use_8000_method {
            // Unsigned tile number, 8000 method
            base_address + (tile_number as u16 * 16)
        } else {
            // Signed tile number, 8800 method
            let signed_tile_number = tile_number as i8;
            (base_address as i32 + (signed_tile_number as i32 * 16)) as u16
        };

        // Return the specific byte within the tile data based on the offset
        self.bus.borrow().read_byte(tile_data_address + offset)
    }
    fn push_to_fifo(&mut self) {
        if self
            .pixel_fifo
            .push_bg_pixels([self.fetcher.tile_data_high, self.fetcher.tile_data_low])
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
                self.fetcher.tile_data_low = self.fetch_tile_data(self.fetcher.tile_number);
                self.fetcher.step += 1;
            }
            2 => {
                self.fetcher.tile_data_high = self.fetch_tile_data(self.fetcher.tile_number);
                // Delay of 12 T-cycles before the background FIFO is first filled with pixel data.
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
        // 160 pixels per line
        if self.fetcher.x_pos_counter >= X_POSITION_COUNTER_MAX {
            self.mode = Mode::HBLANK;
            return;
        }
        // Pixel fetcher
        // Each step is 2 t-cycles.
        if self.mode_cycles % 2 == 0 {
            self.fetcher_step();
        }

        // Push pixel to LCD
        if let Some(color) = self.pixel_fifo.pop_pixel() {
            // Convert color number to actual RGB color and write to buffer
            let actual_color = match color {
                0 => 0xFFFFFF, // White
                1 => 0xAAAAAA, // Light gray
                2 => 0x555555, // Dark gray
                3 => 0x000000, // Black
                _ => 0xFFFFFF, // Shouldn't happen
            };
            let ly = self.get_io_register(IoRegister::Ly);
            let x_pos = self.fetcher.x_pos_counter as usize;

            if x_pos < SCREEN_WIDTH as usize && (ly as usize) < SCREEN_HEIGHT as usize {
                self.buffer[ly as usize * SCREEN_WIDTH as usize + x_pos] = actual_color;
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
