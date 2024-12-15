use std::collections::VecDeque;

use gameboy_core::{
    self as GameboyCore,
    cartridge::{self}, ppu::{fetcher::Fetcher, PPUMode}, 
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GameboyWasm {
    gameboy: GameboyCore::gameboy::Gameboy,
    is_paused: bool,
}

#[wasm_bindgen]
impl GameboyWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(palette: Vec<u32>) -> Self {
        let palette_array: [u32; 4] = match palette.len() {
            4 => [palette[0], palette[1], palette[2], palette[3]],
            _ => [0xA8D08D, 0x6A8E3C, 0x3A5D1D, 0x1F3C06], // Default green palette
        };
        Self {
            gameboy: GameboyCore::gameboy::Gameboy::new(palette_array),
            is_paused: false,
        }
    }

    pub fn init(&mut self, rom: &[u8], state: Option<Vec<u8>>) -> Result<(), String> {
        self.gameboy.set_power_up_sequence();
        self.gameboy.load_rom(rom);
    
        if let Some(state) = state {
            match self.gameboy.load_state(state) {
                Ok(_) => {}
                Err(err) => {
                    println!("Failed to load state: {:?}", err);
                    // If there's an error loading the save state, just continue without it
                }
            }
        }
    
        Ok(())
    }
    pub fn save_state(&self) -> Vec<u8> {
        match self.gameboy.save_state() {
            Ok(state) => state,
            Err(_) => {
                // Return an empty vector on error
                vec![]
            }
        }
    }

    pub fn load_state(&mut self, state: Vec<u8>) -> Result<(), String> {
        self.gameboy
            .load_state(state)
            .map_err(|_| "Failed to load state".to_string())
    }

    pub fn tick(&mut self) {
        self.gameboy.tick();
    }
    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    pub fn resume(&mut self) {
        self.is_paused = false;
    }

    pub fn run_frame(&mut self) {
        if !self.is_paused {
            self.gameboy.run_frame();
        }
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
    pub fn get_audio_buffer(&mut self) -> Vec<f32> {
        self.gameboy.apu.get_samples()
    }
    pub fn toggle_audio(&mut self) {
        self.gameboy.apu.toggle_audio();
    }
    pub fn toggle_channel(&mut self, channel: u8) {
        self.gameboy.apu.toggle_channel(channel);
    }

    pub fn get_apu_state(&self) -> WasmApuState {    
        WasmApuState {
            apu_enabled: self.gameboy.apu.enabled,
            ch1_enabled: self.gameboy.apu.ch1_enabled,
            ch2_enabled: self.gameboy.apu.ch2_enabled,
            ch3_enabled: self.gameboy.apu.ch3_enabled,
            ch4_enabled: self.gameboy.apu.ch4_enabled,
            current_ch1_output: self.gameboy.apu.current_ch1_output,
            current_ch2_output: self.gameboy.apu.current_ch2_output,
            current_ch3_output: self.gameboy.apu.current_ch3_output,
            current_ch4_output: self.gameboy.apu.current_ch4_output,
        }
    }
    pub fn get_cpu_state(&self) -> WasmCPUState {
        let cpu_state = self.gameboy.cpu.save_state();
        WasmCPUState {
            a: cpu_state.a,
            b: cpu_state.b,
            c: cpu_state.c,
            d: cpu_state.d,
            e: cpu_state.e,
            h: cpu_state.h,
            l: cpu_state.l,
            f: cpu_state.f,
            sp: cpu_state.sp,
            pc: cpu_state.pc,
            ime: cpu_state.ime,
            halt: cpu_state.halt,
            cycles: cpu_state.cycles,
        }
    }
    pub fn get_timer_state(&self)-> WasmTimerState{
        let timer_state =self.gameboy.timer.save_state();
        WasmTimerState{
            div_counter: timer_state.div_counter,
            tima_counter: timer_state.tima_counter,
        }
    }
    pub fn get_bus_state(&self) -> WasmBusState {
        let bus_state = self.gameboy.bus.borrow().save_state();
        let joypad_state = self.gameboy.bus.borrow().joypad.clone();
        WasmBusState {
            joypad:  WasmJoypad { register: joypad_state.register, keys: joypad_state.keys},         
            io_registers: bus_state.io_registers,
            hram: bus_state.hram,
            ie_register: bus_state.ie_register,
            vram: bus_state.vram,
            ram_bank_0: bus_state.ram_bank_0,
            ram_bank_n: bus_state.ram_bank_n,
        }
    }
    pub fn get_ppu_state(&self) -> WasmPpuState {
        let ppu_state = self.gameboy.ppu.save_state();
        WasmPpuState {
            mode: match ppu_state.mode {
                PPUMode::HBLANK => WasmPPUMode::HBLANK,
                PPUMode::VBLANK => WasmPPUMode::VBLANK,
                PPUMode::OAM_SCAN => WasmPPUMode::OAM_SCAN,
                PPUMode::DRAWING => WasmPPUMode::DRAWING,
            },
            mode_cycles: ppu_state.mode_cycles,
          
            fetcher: WasmFetcher{
                step: ppu_state.fetcher.step,
                is_window_fetch: ppu_state.fetcher.is_window_fetch,
                x_pos_counter: ppu_state.fetcher.x_pos_counter,
                window_line_counter: ppu_state.fetcher.window_line_counter,
                pause: ppu_state.fetcher.pause,
            },
            sprite_fetcher: WasmSpriteFetcher {
                step: ppu_state.sprite_fetcher.step,
             
                active: ppu_state.sprite_fetcher.active,
                remaining_pixels: ppu_state.sprite_fetcher.remaining_pixels,
                sprite: WasmSprite {
                    y_pos: ppu_state.sprite_fetcher.sprite.y_pos,
                    x_pos: ppu_state.sprite_fetcher.sprite.x_pos,
                    tile_number: ppu_state.sprite_fetcher.sprite.tile_number,
                    flags: ppu_state.sprite_fetcher.sprite.flags,
                },
            },
           
            window_triggered_this_frame: ppu_state.window_triggered_this_frame,
            
            x_render_counter: ppu_state.x_render_counter,
            window_line_counter_incremented_this_scanline: ppu_state.window_line_counter_incremented_this_scanline,
            new_frame: ppu_state.new_frame,
            debug_config: WasmDebugConfig {
                sprite_debug_enabled: ppu_state.debug_config.sprite_debug_enabled,
                window_debug_enabled: ppu_state.debug_config.window_debug_enabled,
            },
        }
    }

    pub fn toggle_sprite_debug_mode(&mut self, enabled: bool) {
        self.gameboy.ppu.toggle_sprite_debug_mode(enabled);
    }
    pub fn toggle_window_debug_mode(&mut self, enabled: bool) {
        self.gameboy.ppu.toggle_window_debug_mode(enabled);
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

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct WasmCPUState {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub f: u8,
    pub sp: u16,
    pub pc: u16,
    pub ime: bool,
    pub halt: bool,
    pub cycles: usize,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct WasmTimerState {
    pub div_counter: usize,
    pub tima_counter: usize,
}
#[wasm_bindgen]
#[derive(Clone, Debug, Copy)]
pub struct WasmJoypad {
    pub register: u8,
    pub keys: u8,
}
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct WasmBusState {
    pub joypad: WasmJoypad, 
    io_registers: [u8; 0x7F],
    hram: [u8; 0x7F],
    pub ie_register: u8,
    vram: [u8; 0x2000],
    ram_bank_0: [u8; 0x1000],
    ram_bank_n: [u8; 0x1000],
}

#[wasm_bindgen]
impl WasmBusState {
    #[wasm_bindgen(getter)]
    pub fn io_registers(&self) -> Vec<u8> {
        self.io_registers.to_vec()
    }
    #[wasm_bindgen(getter)]
    pub fn hram(&self) -> Vec<u8> {
        self.hram.to_vec()
    }
    #[wasm_bindgen(getter)]
    pub fn vram(&self) -> Vec<u8> {
        self.vram.to_vec()
    }
    #[wasm_bindgen(getter)]
    pub fn ram_bank_0(&self) -> Vec<u8> {
        self.ram_bank_0.to_vec()
    }
    #[wasm_bindgen(getter)]
    pub fn ram_bank_n(&self) -> Vec<u8> {
        self.ram_bank_n.to_vec()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct WasmPpuState {
    pub mode: WasmPPUMode,
    pub mode_cycles: usize,
    pub fetcher: WasmFetcher,
    pub sprite_fetcher: WasmSpriteFetcher,
    pub window_triggered_this_frame: bool,
    pub x_render_counter: i16,
    pub window_line_counter_incremented_this_scanline: bool,
    pub new_frame: bool,
    pub debug_config: WasmDebugConfig,
}
#[wasm_bindgen]
#[derive(Clone, Debug, Copy)]
pub struct WasmDebugConfig {
    pub sprite_debug_enabled: bool,
    pub window_debug_enabled: bool,
}
#[wasm_bindgen]
#[derive(Clone, Debug, Copy)]
pub enum WasmPPUMode {
    HBLANK = 0,
    VBLANK = 1,
    OAM_SCAN = 2,
    DRAWING = 3,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Copy)]
pub struct WasmSprite {
    pub y_pos: u8,
    pub x_pos: u8,
    pub tile_number: u8,
    pub flags: u8,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Copy)]
pub struct WasmFetcher {
    pub step: u8,
   
    pub is_window_fetch: bool,

    pub x_pos_counter: u16,
    pub window_line_counter: u16,
    pub pause: bool,    
}
#[wasm_bindgen]
#[derive(Clone, Debug, Copy)]
pub struct WasmSpriteFetcher {
    pub step: u8,
    
    pub active: bool,
    pub remaining_pixels: u8,
    pub sprite: WasmSprite,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct WasmApuState {
    pub apu_enabled: bool,
    pub ch1_enabled: bool,
    pub ch2_enabled: bool,
    pub ch3_enabled: bool,
    pub ch4_enabled: bool,
    pub current_ch1_output: f32,
    pub current_ch2_output: f32,
    pub current_ch3_output: f32,
    pub current_ch4_output: f32,
}