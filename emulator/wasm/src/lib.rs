use gameboy_core::{self as GameboyCore, cartridge::{self, }};
use wasm_bindgen::prelude::*;

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
    pub fn init(&mut self, rom_name: &str) -> Result<(), String> {
        let rom_data: &[u8] = match rom_name {
            "tennis" => include_bytes!("../test/tennis.gb"),
            "tetris" => include_bytes!("../test/tetris.gb"),
            "zelda_awakening" => include_bytes!("../test/zelda_awakening.gb"),
            _ => return Err(format!("Unknown ROM: {}", rom_name)),
        };

        self.gameboy.set_power_up_sequence();
        self.gameboy.load_rom(rom_data);
        Ok(())
    }

    pub fn tick(&mut self) {
        self.gameboy.tick();
    }
    pub fn run_frame(&mut self) {
        self.gameboy.run_frame();
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
   
    pub fn get_cartridge_info(&self) -> CartridgeHeaderState {
        let cartridge_data = self.gameboy.bus.borrow().read_cartridge_header();
        let title = cartridge::cartridge_header::get_title(&cartridge_data);
        let kind = cartridge::cartridge_header::get_cartridge_type(&cartridge_data);
        let rom_size = cartridge::cartridge_header::get_rom_size(&cartridge_data);
        let ram_size = cartridge::cartridge_header::get_ram_size(&cartridge_data);
        let destination = cartridge::cartridge_header::get_destination_code(&cartridge_data);
        let sgb_flag = cartridge::cartridge_header::get_sgb_flag(&cartridge_data);
        let rom_version = cartridge::cartridge_header::get_mask_rom_version(&cartridge_data);
        let licensee_code = cartridge::cartridge_header::get_licensee_code(&cartridge_data);

        CartridgeHeaderState {
            title,
            kind,
            rom_size,
            ram_size,
            destination,
            sgb_flag,
            rom_version,
            licensee_code,
        }
        
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct CartridgeHeaderState {
    title: String,
    kind: String,
    rom_size: String,
    ram_size: String,
    destination: String,
    sgb_flag: String,
    rom_version: String,
    licensee_code: String,
}
#[wasm_bindgen]
impl CartridgeHeaderState {
    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        self.title.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> String {
        self.kind.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn rom_size(&self) -> String {
        self.rom_size.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn ram_size(&self) -> String {
        self.ram_size.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn destination(&self) -> String {
        self.destination.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn sgb_flag(&self) -> String {
        self.sgb_flag.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn rom_version(&self) -> String {
        self.rom_version.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn licensee_code(&self) -> String {
        self.licensee_code.clone()
    }
}