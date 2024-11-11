use crate::bus::{io_address::IoRegister, Bus};
use helper::should_add_sprite;
use minifb::{Key, Window, WindowOptions};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
    vec,
};

mod helper;

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;
const CYCLES_PER_SCANLINE: u32 = 456;
const X_POSITION_COUNTER_MAX: u32 = 160;
const SPRITE_HEGHT_TALL: u8 = 16;
const SPRITE_HEIGHT_NORMAL: u8 = 8;

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
    mode_cycles: u32,

    sprite_buffer: Vec<Sprite>,
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
        Self {
            window: Window::new(
                "SabiBoy",
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                WindowOptions::default(),
            )
            .unwrap(),
            buffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT],
            bus: bus,
            mode: Mode::OAM_SCAN,
            mode_cycles: 0,
            sprite_buffer: Vec::new(),
        }
    }
    pub fn tick(&mut self) {
        // Check if LCD is enabled
        let lcdc_enabled = self.get_io_register(IoRegister::Lcdc) & 0b1000_0000 != 0;
        if !lcdc_enabled {
            return;
        }
        // scanline
        self.mode_cycles += 1;
        match self.mode {
            Mode::OAM_SCAN => self.handle_oam(),
            Mode::DRAWING => self.handle_vram(),
            Mode::HBLANK => self.handle_hblank(),
            Mode::VBLANK => self.handle_vblank(),
        }

        self.update_stat();
        
        if self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.window
                .update_with_buffer(&self.buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
                .unwrap();
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
        /* Add sprites  to buffer
        80 T-cycles  / 40 sprites (1 sprite is 4 bytes) = 1 sprite per 2 cycles
        */
        if self.mode_cycles >= 80 {
            self.mode = Mode::DRAWING;
            self.mode_cycles = 0;
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
    fn handle_hblank(&mut self) {}
    fn handle_vblank(&mut self) {}

    fn handle_vram(&mut self) {}
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

        self.set_io_register(IoRegister::Stat, stat);
    }
}
