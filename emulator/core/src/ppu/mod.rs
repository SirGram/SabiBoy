pub mod fetcher;
pub mod fetcher_sprites;
mod helper;
pub mod pixelfifo;

use crate::bus::{io_address::IoRegister, Bus, GameboyMode, MemoryInterface};
use fetcher::Fetcher;
use fetcher_sprites::SpriteFetcher;
use helper::{should_add_sprite, should_fetch_sprite};
use pixelfifo::{ColorValue, PixelFifo};
use serde::{Deserialize, Serialize};
use std::{
    cell::{Ref, RefCell},
    cmp::Ordering,
    rc::Rc,
    vec,
};

const SCREEN_WIDTH: u8 = 160;
const SCREEN_HEIGHT: u8 = 144;
const CYCLES_PER_SCANLINE: usize = 456;
const X_POSITION_COUNTER_MAX: u16 = 160;
const SCANLINE_Y_COUNTER_MAX: u8 = 153;
const VBLANK_START_SCANLINE: u8 = 144;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum PPUMode {
    HBLANK = 0,
    VBLANK = 1,
    OAM_SCAN = 2,
    DRAWING = 3,
}

#[derive(Clone, Debug)]
pub struct PPU {
    pub palette: [u32; 4],
    pub mode: PPUMode,
    pub mode_cycles: usize,

    buffer: Vec<u32>,

    sprite_buffer: Vec<Sprite>,
    fetcher: Fetcher,
    sprite_fetcher: SpriteFetcher,
    pixel_fifo: PixelFifo,
    window_triggered_this_frame: bool,

    previous_stat_conditions: u8,
    x_render_counter: i16,
    window_line_counter_incremented_this_scanline: bool,
    new_frame: bool,
    debug_config: DebugConfig,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PPUState {
    pub mode: PPUMode,
    pub mode_cycles: usize,
    pub sprite_buffer: Vec<Sprite>,
    pub fetcher: Fetcher,
    pub sprite_fetcher: SpriteFetcher,
    pub pixel_fifo: PixelFifo,
    pub window_triggered_this_frame: bool,
    pub previous_stat_conditions: u8,
    pub x_render_counter: i16,
    pub window_line_counter_incremented_this_scanline: bool,
    pub new_frame: bool,
    pub debug_config: DebugConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy)]
pub struct DebugConfig {
    pub sprite_debug_enabled: bool,
    pub window_debug_enabled: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sprite {
    pub y_pos: u8,
    pub x_pos: u8,
    pub tile_number: u8,
    pub flags: u8,
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
    pub fn new(palette: [u32; 4]) -> Self {
        let debug_config = DebugConfig {
            sprite_debug_enabled: true,
            window_debug_enabled: true,
        };
        Self {
            palette: palette,
            buffer: vec![0; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize],
            mode: PPUMode::OAM_SCAN,
            mode_cycles: 0,
            sprite_buffer: Vec::with_capacity(10),
            fetcher: Fetcher::new(),
            sprite_fetcher: SpriteFetcher::new(),
            pixel_fifo: PixelFifo::new(),
            window_triggered_this_frame: false,
            previous_stat_conditions: 0,
            x_render_counter: -8,
            window_line_counter_incremented_this_scanline: false,
            new_frame: false,
            debug_config: debug_config,
        }
    }
    pub fn save_state(&self) -> PPUState {
        PPUState {
            mode: self.mode,
            mode_cycles: self.mode_cycles,
            sprite_buffer: self.sprite_buffer.clone(),
            fetcher: self.fetcher.clone(),
            sprite_fetcher: self.sprite_fetcher.clone(),
            pixel_fifo: self.pixel_fifo.clone(),
            window_triggered_this_frame: self.window_triggered_this_frame,
            previous_stat_conditions: self.previous_stat_conditions,
            x_render_counter: self.x_render_counter,
            window_line_counter_incremented_this_scanline: self
                .window_line_counter_incremented_this_scanline,
            new_frame: self.new_frame,
            debug_config: self.debug_config,
        }
    }
    pub fn load_state(&mut self, state: PPUState) {
        self.mode = state.mode;
        self.mode_cycles = state.mode_cycles;
        self.sprite_buffer = state.sprite_buffer;
        self.fetcher = state.fetcher;
        self.sprite_fetcher = state.sprite_fetcher;
        self.pixel_fifo = state.pixel_fifo;
        self.window_triggered_this_frame = state.window_triggered_this_frame;
        self.previous_stat_conditions = state.previous_stat_conditions;
        self.x_render_counter = state.x_render_counter;
        self.window_line_counter_incremented_this_scanline =
            state.window_line_counter_incremented_this_scanline;
        self.new_frame = state.new_frame;
    }
    pub fn toggle_sprite_debug_mode(&mut self, enabled: bool) {
        self.debug_config.sprite_debug_enabled = enabled;
    }

    // New method to enable/disable window debug mode
    pub fn toggle_window_debug_mode(&mut self, enabled: bool) {
        self.debug_config.window_debug_enabled = enabled;
    }

    fn check_window<M: MemoryInterface>(&mut self, memory: &mut M) -> bool {
        if !self.debug_config.window_debug_enabled {
            return false;
        }
        /*
        Bit 5 of the LCDC register is set to 1
        The condition WY = LY has been true at any point in the currently rendered frame.
        The current X-position of the shifter is greater than or equal to WX - 7
        */
        if memory.gb_mode() == GameboyMode::DMG {
            let lcdc = memory.read_byte(IoRegister::Lcdc.address());
            if lcdc & 0b0010_0000 == 0 {
                return false;
            }
        }
        let wy = memory.read_byte(IoRegister::Wy.address());
        let wx = memory.read_byte(IoRegister::Wx.address());
        let ly = memory.read_byte(IoRegister::Ly.address());

        if ly == wy {
            if !self.window_triggered_this_frame {
                self.window_triggered_this_frame = true;
            }
        }
        self.window_triggered_this_frame
            && self.x_render_counter as i16 >= (wx.wrapping_sub(7)) as i16
    }

    pub fn get_frame_buffer(&self) -> &[u32] {
        &self.buffer
    }
    pub fn reset_scanline(&mut self) {
        self.mode_cycles = 0;
        self.pixel_fifo.reset();
        self.sprite_buffer.clear();
        self.fetcher.scanline_reset();
        self.sprite_fetcher.scanline_reset();
        self.x_render_counter = -8;
        if self.window_line_counter_incremented_this_scanline {
            self.fetcher.window_line_counter += 1;
        }
        self.window_line_counter_incremented_this_scanline = false;
    }
    fn reset_frame<M: MemoryInterface>(&mut self, memory: &mut M) {
        self.new_frame = false;
        self.reset_scanline();
        self.mode = PPUMode::OAM_SCAN;
        self.window_triggered_this_frame = false;
        self.fetcher.window_line_counter = 0;
        self.set_io_register(memory, IoRegister::Ly, 0);
    }

    pub fn tick<M: MemoryInterface>(&mut self, memory: &mut M) {
        // Check if LCD is enabled
        if memory.gb_mode() == GameboyMode::DMG {
            let lcdc = self.get_io_register(memory, IoRegister::Lcdc);
            if (lcdc & 0x80) == 0 {
                return;
            }
        }

        let ly = self.get_io_register(memory, IoRegister::Ly);

        /*  if ly ==50 && self.mode_cycles < 200 && self.mode_cycles >80 {

          println!(
            "ly {} cycles {} window {} wcoun {} xpos {} xposren {} scx {} bg_fifo_len {} sprite_fifo_len {} current_sprite_x {} sprftch_act {}",

            ly,
            self.mode_cycles,
            self.fetcher.is_window_fetch,
            self.fetcher.window_line_counter,
            self.fetcher.x_pos_counter,
            self.x_render_counter,
            self.get_io_register(IoRegister::Scx) & 7,
            self.pixel_fifo.bg_fifo.len(),
            self.pixel_fifo.sprite_fifo.len(),
            self.sprite_fetcher.sprite.x_pos,
            self.sprite_fetcher.active
            );
        }  */

        match self.mode {
            PPUMode::OAM_SCAN => self.handle_oam(memory),
            PPUMode::DRAWING => self.handle_drawing(memory),
            PPUMode::HBLANK => self.handle_hblank(),
            PPUMode::VBLANK => {}
        }

        if self.mode_cycles >= CYCLES_PER_SCANLINE {
            // Reset fetcher state for new line
            self.reset_scanline();
            // Increment LY and handle PPUMode transitions
            if ly < VBLANK_START_SCANLINE {
                self.set_io_register(memory, IoRegister::Ly, ly + 1);
                self.mode = PPUMode::OAM_SCAN;
            } else if ly == VBLANK_START_SCANLINE {
                // vblank start
                self.mode = PPUMode::VBLANK;
                self.handle_vblank(memory);
                self.set_io_register(memory, IoRegister::Ly, ly + 1);
            } else if ly >= SCANLINE_Y_COUNTER_MAX {
                self.reset_frame(memory);
            } else {
                self.set_io_register(memory, IoRegister::Ly, ly + 1);
            }
        }

        self.update_stat(memory);
        self.mode_cycles += 1;
    }

    fn read_sprite<M: MemoryInterface>(&self, memory: &mut M, address: u16) -> Sprite {
        Sprite {
            y_pos: memory.read_byte(address),
            x_pos: memory.read_byte(address + 1),
            tile_number: memory.read_byte(address + 2),
            flags: memory.read_byte(address + 3),
        }
    }

    fn handle_oam<M: MemoryInterface>(&mut self, memory: &mut M) {
        if self.mode_cycles % 2 != 0 {
            let current_entry = self.mode_cycles / 2;
            let sprite = self.read_sprite(memory, 0xFE00 + (current_entry as u16 * 4));

            /*  if sprite.tile_number != 0  && sprite.y_pos>=64 && sprite.y_pos <= 72 {  // Only log non-zero sprites
                println!(
                    "Sprite found: y_pos={}, x_pos={}, tile_number={}, flags={:b}",
                    sprite.y_pos, sprite.x_pos, sprite.tile_number, sprite.flags
                );
            } */

            if should_add_sprite(
                &sprite,
                self.get_io_register(memory, IoRegister::Ly),
                self.get_io_register(memory, IoRegister::Lcdc),
                self.sprite_buffer.len(),
            ) {
                self.sprite_buffer.push(sprite);
            }
        }
        if self.mode_cycles >= 80 {
            self.mode = PPUMode::DRAWING;
            self.sprite_buffer.sort_by(|a, b| {
                match a.x_pos.cmp(&b.x_pos) {
                    Ordering::Equal => b.flags.cmp(&a.flags), //  Higher OAM  index
                    ordering => ordering,
                }
            });
        }
    }

    fn handle_drawing<M: MemoryInterface>(&mut self, memory: &mut M) {
        // Exit if we've drawn all pixels for this line
        if self.x_render_counter >= X_POSITION_COUNTER_MAX as i16 {
            self.mode = PPUMode::HBLANK;
            return;
        }

        // Pixel shifting
        if !self
            .pixel_fifo
            .is_paused(self.sprite_fetcher.active, self.fetcher.pause)
        {
            if let Some(color_value) = self.pixel_fifo.pop_pixel(memory, &mut self.fetcher) {
                let ly = self.get_io_register(memory, IoRegister::Ly);

                if self.x_render_counter >= 0
                    && self.x_render_counter < SCREEN_WIDTH as i16
                    && (ly as usize) < SCREEN_HEIGHT as usize
                {
                    let buffer_index =
                        ly as usize * SCREEN_WIDTH as usize + self.x_render_counter as usize;

                        let final_color = match color_value {
                            ColorValue::Dmg(color_index) => self.palette[color_index as usize & 0x03],
                            ColorValue::Cgb((r, g, b)) => {
                                ((r as u32) << 16) | ((g as u32) << 8) | (b as u32) | (0xFF << 24)
                            }
                        };

                    self.buffer[buffer_index] = final_color;
                }
                self.fetcher.x_pos_counter += 1;
                self.x_render_counter += 1;
            }
        }
        // Window check
        if !self.fetcher.is_window_fetch {
            if self.check_window(memory) {
                self.fetcher.window_trigger(&mut self.pixel_fifo);
                if !self.window_line_counter_incremented_this_scanline {
                    self.window_line_counter_incremented_this_scanline = true;
                }
            }
        }

        // Check sprites
        if !self.sprite_fetcher.active && self.debug_config.sprite_debug_enabled {
            /*  println!("sprite fetch "); */

            if let Some(sprite) =
                should_fetch_sprite(self.x_render_counter, &mut self.sprite_buffer)
            {
                // Now start sprite fetch
                self.sprite_fetcher.start_fetch(&sprite);
                /* println!("sprite found x {}", sprite.x_pos); */
            }
        }
        /* if a bg fetch is in progress and you run into the start of a sprite;
            immediately stop popping off pixels and finish up that bg fetch (and save the fetched data),
            perform the sprite fetch and load up the sprite data into the sprite fifo,
            then resume popping off pixels and try to push that saved bg data into the fifo,
            then start the next bg fetch when the data's been pushed
        */

        // Every 2 dots, run fetcher steps
        if self.mode_cycles % 2 == 0 {
            self.fetcher.step(memory, &mut self.pixel_fifo);
            if self.sprite_fetcher.active {
                self.fetcher.pause();
                self.sprite_fetcher.step(memory, &mut self.pixel_fifo);
            } else {
                self.fetcher.unpause();
            }
        }
    }

    fn handle_hblank(&mut self) {
        // pads till 456 cycles
    }

    fn handle_vblank<M: MemoryInterface>(&mut self, memory: &mut M) {
        // pads 10 vertical scanlinesf

        //request vblank interrupt
        let if_register = self.get_io_register(memory, IoRegister::If);
        self.set_io_register(memory, IoRegister::If, if_register | 0b0000_0001);
        // update window per frame
        self.new_frame = true;
    }
    fn get_io_register<M: MemoryInterface>(&self, memory: &mut M, register: IoRegister) -> u8 {
        memory.read_byte(register.address())
    }
    fn set_io_register<M: MemoryInterface>(&self, memory: &mut M, register: IoRegister, value: u8) {
        memory.write_byte(register.address(), value);
    }
    fn update_stat<M: MemoryInterface>(&mut self, memory: &mut M) {
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
        let mut stat = self.get_io_register(memory, IoRegister::Stat);

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
        let ly = self.get_io_register(memory, IoRegister::Ly);
        let lyc = self.get_io_register(memory, IoRegister::Lyc);
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
            let if_reg = self.get_io_register(memory, IoRegister::If);
            self.set_io_register(memory, IoRegister::If, if_reg | 0b0000_0010);
        }

        // Store current conditions for next comparison
        self.previous_stat_conditions = current_conditions;

        // Ensure bit 7 is always set
        stat |= 0b1000_0000;
        self.set_io_register(memory, IoRegister::Stat, stat);
    }
}
