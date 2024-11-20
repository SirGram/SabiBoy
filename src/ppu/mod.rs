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

pub const COLORS: [u32; 5] = [0xA8D08D, 0x6A8E3C, 0x3A5D1D, 0x1F3C06, 0xFF0000];

const SCREEN_WIDTH: u8 = 160;
const SCREEN_HEIGHT: u8 = 144;
const CYCLES_PER_SCANLINE: usize = 456;
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
    window_triggered_this_frame: bool,

    previous_stat_conditions: u8,
    x_render_counter: i16,
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
            window_triggered_this_frame: false,
            previous_stat_conditions: 0,
            x_render_counter: -8,
        }
    }
    
    fn check_window(&mut self) -> bool {
        /*
        Bit 5 of the LCDC register is set to 1
        The condition WY = LY has been true at any point in the currently rendered frame.
        The current X-position of the shifter is greater than or equal to WX - 7
        */
        let lcdc = self.bus.borrow().read_byte(IoRegister::Lcdc.address());
        if lcdc & 0b0010_0000 == 0 {
            return false;
        } else { /*
             println!("window lcdc {}", lcdc); */
        }
        let wy = self.bus.borrow().read_byte(IoRegister::Wy.address());
        let wx = self.bus.borrow().read_byte(IoRegister::Wx.address());
        let ly = self.bus.borrow().read_byte(IoRegister::Ly.address());
        /*
        if self.window_triggered_this_frame {
            println!("window {} {}", ly, wy);
        } */
        if ly == wy {
            if !self.window_triggered_this_frame {
                self.window_triggered_this_frame = true;
            }
        }
        self.window_triggered_this_frame
            && self.fetcher.x_pos_counter as u16 >= (wx as u16).wrapping_sub(7)
    }

  
    pub fn reset_scanline(&mut self) {
        self.mode_cycles = 0;
        self.sprite_buffer.clear();        
        self.fetcher.x_pos_counter = 0;
        self.x_render_counter = -8;
    }
    fn reset_frame(&mut self) {
        self.reset_scanline();
        self.set_io_register(IoRegister::Ly, 0);
        self.mode = PPUMode::OAM_SCAN;
        self.window_triggered_this_frame = false;
        
    }



    pub fn tick(&mut self) {
        self.frame_cycles = self.frame_cycles.wrapping_add(1);
        // Check if LCD is enabled
        let lcdc = self.get_io_register(IoRegister::Lcdc);
        if (lcdc & 0x80) == 0 {
            return;
        }

        // Window check
        /* if self.check_window() {
            /*   self.pixel_fifo.bg_fifo.clear(); */
            self.fetcher.window_trigger();
        } */

        // scanline

        match self.mode {
            PPUMode::OAM_SCAN => self.handle_oam(),
            PPUMode::DRAWING => self.handle_drawing(),
            PPUMode::HBLANK => self.handle_hblank(),
            PPUMode::VBLANK => {}
        }
        
        self.update_stat();
        
        self.mode_cycles += 1;
        

        if self.mode_cycles >= CYCLES_PER_SCANLINE {
            let ly = self.get_io_register(IoRegister::Ly);

            // Increment LY and handle PPUMode transitions
            if ly < VBLANK_START_SCANLINE {
                self.set_io_register(IoRegister::Ly, ly + 1);
                self.mode = PPUMode::OAM_SCAN;
            } else if ly == VBLANK_START_SCANLINE {
                // vblank start
                self.set_io_register(IoRegister::Ly, ly + 1);
                self.mode = PPUMode::VBLANK;
                self.handle_vblank();
            } else if ly > SCANLINE_Y_COUNTER_MAX {
                self.reset_frame();
            } else {
                self.set_io_register(IoRegister::Ly, ly + 1);
            }

            // Reset fetcher state for new line
            self.reset_scanline();
        }
        let ly = self.get_io_register(IoRegister::Ly);
        if ly < 10 {
            println!("  cycles: {}  ly: {} x: {}",self.mode_cycles, ly,  self.fetcher.x_pos_counter,   );
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

        if self.mode_cycles % 2 == 0 {
            let current_entry = self.mode_cycles / 2;
            let sprite = self.read_sprite(0xFE00 + (current_entry as u16 * 4));
            if should_add_sprite(
                &sprite,
                self.get_io_register(IoRegister::Ly),
                self.get_io_register(IoRegister::Lcdc),
                self.sprite_buffer.len(),
            ) {
                self.sprite_buffer.push(sprite);
            }
        }
        if self.mode_cycles >= 80 {
            self.mode = PPUMode::DRAWING;
            self.fetcher.step = 0;
            self.sprite_buffer.sort_by(|a, b| a.x_pos.cmp(&b.x_pos));
        }
    }

    fn handle_drawing(&mut self) {
        // Exit if we've drawn all pixels for this line
        if self.x_render_counter  >= X_POSITION_COUNTER_MAX as i16 {
            self.mode = PPUMode::HBLANK;
            return;
        }
        /* if a bg fetch is in progress and you run into the start of a sprite;
            immediately stop popping off pixels and finish up that bg fetch (and save the fetched data),
            perform the sprite fetch and load up the sprite data into the sprite fifo,
            then resume popping off pixels and try to push that saved bg data into the fifo,
            then start the next bg fetch when the data's been pushed
        */

        // Check sprites
        // First handle sprite fetching if needed

       /*  if let Some(sprite) = should_fetch_sprite(self.fetcher.x_pos_counter, &self.sprite_buffer) {
            // Now start sprite fetch
            self.sprite_fetcher.start_fetch(&sprite);
        } */

        // Every 2 dots, run fetcher steps
        if self.mode_cycles % 2 == 0 {
            // If sprite fetch is active, prioritize it
            if self.sprite_fetcher.active {
                self.sprite_fetcher
                    .step(&self.bus, &mut self.pixel_fifo, &mut self.fetcher);
            }
            self.fetcher
                .step(&self.bus, &mut self.pixel_fifo, self.mode_cycles);
        }

        if self.pixel_fifo.is_paused(&self.sprite_fetcher) {
            return;
        }
        if let Some(color) = self.pixel_fifo.pop_pixel(&self.bus) {
            let ly = self.get_io_register(IoRegister::Ly);

            // Only draw if within screen bounds. Discard 1st tile.
            if self.x_render_counter >= 0 && self.x_render_counter < SCREEN_WIDTH as i16  && (ly as usize) < SCREEN_HEIGHT as usize {
                let buffer_index = ly as usize * SCREEN_WIDTH as usize + self.x_render_counter as usize;
                let color = COLORS[color as usize];
                self.buffer[buffer_index] = color;
            }
            self.fetcher.x_pos_counter += 1;
            self.x_render_counter += 1;

           
        }
    }
    fn handle_hblank(&mut self) {
        // pads till 456 cycles
        if self.mode_cycles == 456 && self.window_triggered_this_frame {
            self.fetcher.window_line_counter += 1;
        }
    }

    fn handle_vblank(&mut self) {
        // pads 10 vertical scanlines

        //request vblank interrupt
        let if_register = self.get_io_register(IoRegister::If);
        self.set_io_register(IoRegister::If, if_register | 0b0000_0001);
        // update window per frame
        self.update_window();
        self.fetcher.window_line_counter = 0;
    }
    fn update_window(&mut self) {
        if self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.window
                .update_with_buffer(&self.buffer, SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize)
                .unwrap();
            let should_trigger = self.bus.borrow_mut().joypad.update(&mut self.window);
            if should_trigger {
                self.set_io_register(
                    IoRegister::If,
                    self.get_io_register(IoRegister::If) | 0b0001_0000, // request joypad interrupt
                );
            }
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

        // Determine the current PPU mode
        let ppu_mode = match self.mode {
            PPUMode::HBLANK => 0b00,
            PPUMode::VBLANK => 0b01,
            PPUMode::OAM_SCAN => 0b10,
            PPUMode::DRAWING => 0b11,
        };

        // Clear and set mode bits
        stat &= 0b11111100; // Clear the mode bits
        stat |= ppu_mode; // Set the current mode bits

        // Update the coincidence flag
        let ly = self.get_io_register(IoRegister::Ly);
        let lyc = self.get_io_register(IoRegister::Lyc);
        let coincidence_flag = if ly == lyc { 1 } else { 0 };
        stat &= 0b11111011; // Clear the coincidence flag bit
        stat |= coincidence_flag << 2; // Set the current coincidence flag bit

        // Key change: Track the current interrupt conditions more precisely
        let mut current_conditions = 0;

        // Check LYC=LY condition
        if coincidence_flag == 1 && (stat & 0b01000000 != 0) {
            current_conditions |= 0b0001;
        }

        // Check mode-specific conditions
        match ppu_mode {
            0b00 if stat & 0b00001000 != 0 => current_conditions |= 0b0010, // Mode 0 (H-Blank)
            0b01 if stat & 0b00010000 != 0 => current_conditions |= 0b0100, // Mode 1 (V-Blank)
            0b10 if stat & 0b00100000 != 0 => current_conditions |= 0b1000, // Mode 2 (OAM)
            _ => {}
        }

        // Trigger STAT interrupt only if conditions are met and were not previously met
        if current_conditions != 0 && self.previous_stat_conditions == 0 {
            let if_reg = self.get_io_register(IoRegister::If);
            self.set_io_register(IoRegister::If, if_reg | 0b0000_0010);
        }

        // Store current conditions for next comparison
        self.previous_stat_conditions = current_conditions;

        // Ensure bit 7 is always set
        stat |= 0b1000_0000;
        self.set_io_register(IoRegister::Stat, stat);
    }
}
