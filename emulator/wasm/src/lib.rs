use wasm_bindgen::prelude::*;
use std::{cell::RefCell, rc::Rc};
use gameboy_core as GameboyCore;

#[wasm_bindgen]
pub struct GameboyWasm {
    gameboy: GameboyCore::gameboy::Gameboy,
}

#[wasm_bindgen]
impl GameboyWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            gameboy: GameboyCore::gameboy::Gameboy::new(),
        }
    }

   /*  pub fn load_rom(&mut self, rom_data: &[u8]) {
        self.gameboy.load_rom(rom_data);
        self.gameboy.set_power_up_sequence();
    } */
   pub fn init(&mut self) {
    
    self.gameboy.set_power_up_sequence();
    self.gameboy.load_rom(include_bytes!("../test/tennis.gb"));

   }

    pub fn tick(&mut self) {    
        self.gameboy.tick();
    }

    pub fn get_frame_buffer(&self) -> Vec<u8> {
        let buffer = self.gameboy.ppu.get_frame_buffer();
        let mut rgba = Vec::with_capacity(160 * 144 * 4);
        
        for &color in buffer {
            // Convert u32 ARGB to RGBA bytes
            let r = ((color >> 16) & 0xFF) as u8;
            let g = ((color >> 8) & 0xFF) as u8;
            let b = (color & 0xFF) as u8;
            rgba.extend_from_slice(&[r, g, b, 255]); // Full alpha
        }
        
        rgba
    }
    pub fn handle_keys(&mut self, keys: u8) {
        self.gameboy.bus.borrow_mut().joypad.update_keys(keys);
    }

    
}

