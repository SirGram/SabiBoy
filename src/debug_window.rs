use minifb::{Scale, Window, WindowOptions};
use minifb_fonts::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::bus::io_address::IoRegister;
use crate::cartridge::cartridge_header;
use crate::cpu::CPU;
use crate::ppu::{PPUMode, COLORS};
use crate::{bus, ppu};

const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;

struct CartridgeHeaderState {
    title: String,
    kind: String,
    rom_size: String,
    ram_size: String,
    destination: String,
    sgb_flag: String,
    rom_version: String,
    licensee_code: String,
}
impl CartridgeHeaderState {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            kind: String::new(),
            rom_size: String::new(),
            ram_size: String::new(),
            destination: String::new(),
            sgb_flag: String::new(),
            rom_version: String::new(),
            licensee_code: String::new(),
        }
    }
}
pub struct DebugWindow {
    window: Window,
    cpu_registers: [u8; 8],
    interrupt_enable: u8,
    interrupt_request: u8,
    sp: u16,
    pc: u16,
    op: u8,
    ime: bool,
    halt: bool,
    last_cycle: usize,
    io_registers: Vec<(String, u8)>,
    tile_data: [u8; 0x1800],
    bg_tilemap: [u8; 0x800],
    oam_data: [u8; 0xA0],
    window_tilemap: [u8; 0x800],
    cartridge_header: [u8; 0x50],
    cartridge_header_read: bool,
    lcdc: u8,
    window_y: u8,
    window_x: u8,
    mode_cycles: usize,
    mode: PPUMode,
    frame_cycles: usize,
    cartridgeState: CartridgeHeaderState,
}

impl DebugWindow {
    pub fn new() -> Self {
        let window = Window::new(
            "Debug",
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            WindowOptions {
                scale: Scale::X2,
                ..WindowOptions::default()
            },
        )
        .unwrap();

        Self {
            window,
            cpu_registers: [0; 8],
            interrupt_enable: 0,
            interrupt_request: 0,
            sp: 0,
            pc: 0,
            op: 0,
            ime: false,
            halt: false,
            io_registers: Vec::new(),
            tile_data: [0; 0x1800],
            bg_tilemap: [0; 0x800],
            window_tilemap: [0; 0x800],
            lcdc: 0,
            window_y: 0,
            window_x: 0,
            mode_cycles: 0,
            mode: PPUMode::OAM_SCAN,
            last_cycle: 0,
            cartridge_header: [0; 0x50],
            cartridge_header_read: false,
            cartridgeState: CartridgeHeaderState::new(),
            frame_cycles: 0,
            oam_data: [0; 0xA0],
        }
    }
    pub fn render(&mut self) {
        let debug_buffer = self.debug_buffer();
        self.window
            .update_with_buffer(&debug_buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }

    pub fn update(&mut self, cpu: &CPU, bus: &Rc<RefCell<bus::Bus>>, ppu: &ppu::PPU) {
        // CPU registers
        self.cpu_registers = [
            cpu.a,
            cpu.f.bits(),
            cpu.b,
            cpu.c,
            cpu.d,
            cpu.e,
            cpu.h,
            cpu.l,
        ];
        self.sp = cpu.sp;
        self.pc = cpu.pc;
        self.op = bus.borrow().read_byte(cpu.pc);
        self.ime = cpu.ime;
        self.halt = cpu.halt;
        self.lcdc = bus.borrow().read_byte(IoRegister::Lcdc.address());
        self.window_y = bus.borrow().read_byte(IoRegister::Wy.address());
        self.window_x = bus.borrow().read_byte(IoRegister::Wx.address());
        self.last_cycle = cpu.cycles;

        self.mode_cycles = ppu.mode_cycles;
        self.mode = ppu.mode;
        self.frame_cycles = ppu.frame_cycles;

        // IO registers update (grouped)
        self.io_registers = vec![
            // PPU Registers
            (
                "LCDC".to_string(),
                bus.borrow().read_byte(IoRegister::Lcdc.address()),
            ),
            (
                "STAT".to_string(),
                bus.borrow().read_byte(IoRegister::Stat.address()),
            ),
            (
                "SCY".to_string(),
                bus.borrow().read_byte(IoRegister::Scy.address()),
            ),
            (
                "SCX".to_string(),
                bus.borrow().read_byte(IoRegister::Scx.address()),
            ),
            (
                "LY".to_string(),
                bus.borrow().read_byte(IoRegister::Ly.address()),
            ),
            (
                "LYC".to_string(),
                bus.borrow().read_byte(IoRegister::Lyc.address()),
            ),
            // Timer Registers
            (
                "DIV".to_string(),
                bus.borrow().read_byte(IoRegister::Div.address()),
            ),
            (
                "TIMA".to_string(),
                bus.borrow().read_byte(IoRegister::Tima.address()),
            ),
            (
                "TMA".to_string(),
                bus.borrow().read_byte(IoRegister::Tma.address()),
            ),
            (
                "TAC".to_string(),
                bus.borrow().read_byte(IoRegister::Tac.address()),
            ),
            // Interrupt Registers
            (
                "IF".to_string(),
                bus.borrow().read_byte(IoRegister::If.address()),
            ),
            (
                "IE".to_string(),
                bus.borrow().read_byte(IoRegister::Ie.address()),
            ),
            // Joypad and Serial IO
            (
                "JOY".to_string(),
                bus.borrow().read_byte(IoRegister::Joyp.address()),
            ),
            (
                "SB".to_string(),
                bus.borrow().read_byte(IoRegister::Sb.address()),
            ),
            (
                "SC".to_string(),
                bus.borrow().read_byte(IoRegister::Sc.address()),
            ),
            // Sound Registers (example subset)
            (
                "NR10".to_string(),
                bus.borrow().read_byte(IoRegister::Nr10.address()),
            ),
            (
                "NR11".to_string(),
                bus.borrow().read_byte(IoRegister::Nr11.address()),
            ),
        ];

        // Tiledata from vram
        // 0x8000-0x97FF
        // Read tile data from VRAM (0x8000-0x97FF)
        const CHUNK_SIZE: usize = 256;
        for chunk in (0..0x1800).step_by(CHUNK_SIZE) {
            let end = (chunk + CHUNK_SIZE).min(0x1800);
            for i in chunk..end {
                self.tile_data[i] = bus.borrow().read_byte(0x8000 + i as u16);
            }
        }

        // Read background and window tilemaps with correct base addresses
        let background_tilemap_base = if (self.lcdc & 0x08) != 0 {
            0x9C00
        } else {
            0x9800
        };
        let window_tilemap_base = if (self.lcdc & 0x40) != 0 {
            0x9C00
        } else {
            0x9800
        };

        for i in 0..0x800 {
            self.bg_tilemap[i] = bus.borrow().read_byte(background_tilemap_base + i as u16);
            self.window_tilemap[i] = bus.borrow().read_byte(window_tilemap_base + i as u16);
        }
        for i in 0..0xA0 {
            self.oam_data[i] = bus.borrow().read_byte(0xFE00 + i as u16);
        }

        // Read cartridge header
        if !self.cartridge_header_read {
            self.cartridge_header = bus.borrow().read_cartridge_header();
            let title = cartridge_header::get_title(&self.cartridge_header);
            self.cartridgeState.title = title;
            self.cartridgeState.kind = cartridge_header::get_cartridge_type(&self.cartridge_header);
            self.cartridgeState.rom_size = cartridge_header::get_rom_size(&self.cartridge_header);
            self.cartridgeState.ram_size = cartridge_header::get_ram_size(&self.cartridge_header);
            self.cartridgeState.destination =
                cartridge_header::get_destination_code(&self.cartridge_header);
            self.cartridgeState.sgb_flag = cartridge_header::get_sgb_flag(&self.cartridge_header);
            self.cartridgeState.rom_version =
                cartridge_header::get_mask_rom_version(&self.cartridge_header);
            self.cartridgeState.licensee_code =
                cartridge_header::get_licensee_code(&self.cartridge_header);

            self.cartridge_header_read = true;
        }
    }
    fn render_tile_data(&self, buffer: &mut Vec<u32>, start_x: usize, start_y: usize) {
        // Constants for tile layout
        let tiles_per_row = 12; // Increased from 12 for better layout
        let total_tiles = 384;
        let tile_size = 8;
        let signed_addressing = (self.lcdc & 0x10) == 0;

        for tile_index in 0..total_tiles {
            let actual_tile_index = if signed_addressing {
                // Convert to signed addressing (-128 to 127)
                let signed_index = (tile_index as i16) - 128;
                (signed_index & 0xFF) as usize
            } else {
                // Regular addressing (0 to 255)
                tile_index
            };

            let tile_x = tile_index % tiles_per_row;
            let tile_y = tile_index / tiles_per_row;

            let screen_x = start_x + (tile_x * tile_size);
            let screen_y = start_y + (tile_y * tile_size);

            let tile_offset = actual_tile_index * 16;

            for row in 0..tile_size {
                let y = screen_y + row;
                if y >= WINDOW_HEIGHT {
                    continue;
                }

                let low_byte = self.tile_data[tile_offset + (row * 2)];
                let high_byte = self.tile_data[tile_offset + (row * 2) + 1];

                for px in 0..tile_size {
                    let x = screen_x + px;
                    if x >= WINDOW_WIDTH {
                        continue;
                    }

                    let color_bit = 7 - px;
                    let color_index =
                        (((high_byte >> color_bit) & 1) << 1) | ((low_byte >> color_bit) & 1);

                    let buffer_index = y * WINDOW_WIDTH + x;
                    if buffer_index < buffer.len() {
                        buffer[buffer_index] = COLORS[color_index as usize];
                    }
                }
            }
        }
    }

    fn render_tilemap(&self, buffer: &mut Vec<u32>, start_x: usize, start_y: usize, window: bool) {
        let tile_size = 8;
        let map_width = 32;
        let map_height = 32;

        // Draw external border (one-pixel-thick square around the tilemap)
        let border_color = 0x6f6f6f;
        for y in start_y..start_y + map_height * tile_size {
            for x in start_x..start_x + map_width * tile_size {
                // Draw a border around the tilemap area
                if y == start_y
                    || y == start_y + map_height * tile_size - 1
                    || x == start_x
                    || x == start_x + map_width * tile_size - 1
                {
                    let buffer_index = y * WINDOW_WIDTH + x;
                    if buffer_index < buffer.len() {
                        buffer[buffer_index] = border_color;
                    }
                }
            }
        }

        if (self.lcdc & 0x01) == 0 {
            // Check background & window enable bit
            return;
        }

        let tilemap = if window {
            // Only render window if it's enabled
            if (self.lcdc & 0x20) == 0 {
                // Check window enable bit
                return;
            }
            &self.window_tilemap
        } else {
            &self.bg_tilemap
        };
        // Render the tilemap itself
        for map_y in 0..map_height {
            for map_x in 0..map_width {
                let tile_index = tilemap[map_y * map_width + map_x] as usize;
                let tile_offset = tile_index * 16;

                for y in 0..tile_size {
                    let screen_y = start_y + (map_y * tile_size) + y;

                    let low_byte = self.tile_data[tile_offset + (y * 2)];
                    let high_byte = self.tile_data[tile_offset + (y * 2) + 1];

                    for x in 0..tile_size {
                        let screen_x = start_x + (map_x * tile_size) + x;

                        let color_bit = 7 - x;
                        let color_index =
                            (((high_byte >> color_bit) & 1) << 1) | ((low_byte >> color_bit) & 1);

                        let buffer_index = screen_y * WINDOW_WIDTH + screen_x;
                        if buffer_index < buffer.len() {
                            let mut color = COLORS[color_index as usize];
                            if window {
                                // Apply a tint for window tiles if necessary
                                color = color.saturating_add(0x000040);
                            }
                            buffer[buffer_index] = color;
                        }
                    }
                }
            }
        }
    }

    fn render_viewport(&self, buffer: &mut Vec<u32>, start_x: usize, start_y: usize) {
        // Draw the visible area rectangle (160x144)
        let viewport_color = 0xFF0000; // Red color for viewport border
        let window_color = 0x0000FF; // Blue color for window position
        let scx = self
            .io_registers
            .iter()
            .find(|(name, _)| name == "SCX")
            .map(|(_, value)| *value as usize)
            .unwrap_or(0);

        let scy = self
            .io_registers
            .iter()
            .find(|(name, _)| name == "SCY")
            .map(|(_, value)| *value as usize)
            .unwrap_or(0);

        // Draw viewport rectangle
        for y in 0..144 {
            for x in 0..160 {
                let buffer_index = (start_y + y + scy) * WINDOW_WIDTH + (start_x + x + scx);
                if buffer_index < buffer.len() {
                    if x == 0 || x == 159 || y == 0 || y == 143 {
                        buffer[buffer_index] = viewport_color;
                    }
                }
            }
        }

        // Draw window position indicator if window is enabled
        if (self.lcdc & 0x20) != 0 {
            let wx = self.window_x.saturating_sub(7) as usize;
            let wy = self.window_y as usize;

            // Draw window position markers
            for i in 0..8 {
                let x_index = start_x + wx + i;
                let y_index = start_y + wy;

                if x_index < WINDOW_WIDTH && y_index < WINDOW_HEIGHT {
                    let buffer_index = y_index * WINDOW_WIDTH + x_index;
                    if buffer_index < buffer.len() {
                        buffer[buffer_index] = window_color;
                    }
                }
            }
        }
    }

    fn render_oam(&self, buffer: &mut Vec<u32>, start_x: usize, start_y: usize) {
        const TILE_SIZE: usize = 8; // Size of one sprite (8x8)
        const MAX_SPRITES: usize = 40; // Maximum number of sprites
        const SPRITE_SIZE: usize = 4; // Each sprite entry in OAM is 4 bytes
        const ROW_SIZE: usize = 4; // Number of sprites per row in the grid
        const BORDER_COLOR: u32 = 0x6f6f6f; // Color for the outer border

        // Calculate the total grid dimensions
        let cols = ROW_SIZE;
        let rows = MAX_SPRITES / ROW_SIZE;
        let grid_width = cols * TILE_SIZE;
        let grid_height = rows * TILE_SIZE;

        // Draw the outer border
        for y in (start_y - 1)..=(start_y + grid_height) {
            for x in (start_x - 1)..=(start_x + grid_width) {
                if y == start_y - 1
                    || y == start_y + grid_height
                    || x == start_x - 1
                    || x == start_x + grid_width
                {
                    let buffer_index = y * WINDOW_WIDTH + x;
                    if buffer_index < buffer.len() {
                        buffer[buffer_index] = BORDER_COLOR;
                    }
                }
            }
        }

        // Render each OAM entry's tile
        for sprite_index in 0..MAX_SPRITES {
            let row = sprite_index / ROW_SIZE;
            let col = sprite_index % ROW_SIZE;
            let oam_offset = sprite_index * SPRITE_SIZE;

            // Get tile number from OAM (byte 2 of the sprite's 4 bytes)
            let tile_index = self.oam_data[oam_offset + 2] as usize;

            // Calculate where to draw this tile in our grid
            let sprite_start_x = start_x + col * TILE_SIZE;
            let sprite_start_y = start_y + row * TILE_SIZE;

            // Draw the actual tile
            for y in 0..TILE_SIZE {
                // Each tile row is 2 bytes (16 bytes total per tile)
                let tile_offset = tile_index * 16;
                let low_byte = self.tile_data[tile_offset + (y * 2)];
                let high_byte = self.tile_data[tile_offset + (y * 2) + 1];

                for x in 0..TILE_SIZE {
                    let color_bit = 7 - x;
                    let color_index =
                        (((high_byte >> color_bit) & 1) << 1) | ((low_byte >> color_bit) & 1);

                    let screen_x = sprite_start_x + x;
                    let screen_y = sprite_start_y + y;

                    if screen_x < WINDOW_WIDTH && screen_y < WINDOW_HEIGHT {
                        let buffer_index = screen_y * WINDOW_WIDTH + screen_x;
                        if buffer_index < buffer.len() {
                            buffer[buffer_index] = COLORS[color_index as usize];
                        }
                    }
                }
            }
        }
    }

    fn debug_buffer(&self) -> Vec<u32> {
        let mut buffer = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];
        let title_color: u32 = 0x4f4f4f;
        let text_color: u32 = 0x6F6F6F;
        let background_color: u32 = 0x000000;
        let value_color: u32 = 0x8F8F8F;

        let title_font_renderer = font5x8::new_renderer(WINDOW_WIDTH, WINDOW_HEIGHT, title_color);
        let text_font_renderer = font5x8::new_renderer(WINDOW_WIDTH, WINDOW_HEIGHT, text_color);
        let value_font_renderer = font5x8::new_renderer(WINDOW_WIDTH, WINDOW_HEIGHT, value_color);

        // Fill background
        buffer
            .iter_mut()
            .for_each(|pixel| *pixel = background_color);

        // CPU Registers (left column)
        title_font_renderer.draw_text(&mut buffer, 10, 10, "CPU Registers");
        let reg_labels = ["A", "F", "B", "C", "D", "E", "H", "L"];
        for i in 0..reg_labels.len() / 2 {
            // Calculate the y position for the first register in the pair
            let y_offset = 30 + i * 15; // Increased spacing for the two registers

            // Draw first register in the pair
            text_font_renderer.draw_text(&mut buffer, 10, y_offset, reg_labels[i * 2]);
            value_font_renderer.draw_text(
                &mut buffer,
                30,
                y_offset,
                &format!("{:02X}", self.cpu_registers[i * 2]),
            );

            // Draw second register in the pair
            text_font_renderer.draw_text(&mut buffer, 70, y_offset, reg_labels[i * 2 + 1]);
            value_font_renderer.draw_text(
                &mut buffer,
                90,
                y_offset,
                &format!("{:02X}", self.cpu_registers[i * 2 + 1]),
            );
        }

        // Special registers
        text_font_renderer.draw_text(&mut buffer, 10, 160, "SP");
        value_font_renderer.draw_text(&mut buffer, 60, 160, &format!("{:04X}", self.sp));
        text_font_renderer.draw_text(&mut buffer, 10, 175, "PC");
        value_font_renderer.draw_text(&mut buffer, 60, 175, &format!("{:04X}", self.pc));
        text_font_renderer.draw_text(&mut buffer, 10, 190, "OP");
        value_font_renderer.draw_text(&mut buffer, 60, 190, &format!("{:02X}", self.op));
        text_font_renderer.draw_text(&mut buffer, 10, 205, "CYCLES");
        value_font_renderer.draw_text(&mut buffer, 60, 205, &format!("{:02X}", self.last_cycle));

        // IO Registers (left column, below CPU registers)
        let mut y_offset = 220;
        title_font_renderer.draw_text(&mut buffer, 10, y_offset, "IO Registers");
        y_offset += 20;
        for (label, value) in &self.io_registers {
            text_font_renderer.draw_text(&mut buffer, 10, y_offset, label);
            value_font_renderer.draw_text(&mut buffer, 60, y_offset, &format!("{:02X}", value));
            y_offset += 15;
        }

        // Render tilesets
        title_font_renderer.draw_text(&mut buffer, 300, 5, "VRAM");
        self.render_tile_data(&mut buffer, 300, 20);
        title_font_renderer.draw_text(&mut buffer, 400, 5, "Background Tilemap");
        self.render_tilemap(&mut buffer, 400, 20, false);
        self.render_viewport(&mut buffer, 400, 20);
        title_font_renderer.draw_text(&mut buffer, 400, 300, "Window Tilemap");
        self.render_tilemap(&mut buffer, 400, 320, true);
        title_font_renderer.draw_text(&mut buffer, 300, 430, "OAM");
        self.render_oam(&mut buffer, 300, 440);

        // Render PPU Mode
        title_font_renderer.draw_text(&mut buffer, 300, 320, "PPU Mode");
        value_font_renderer.draw_text(&mut buffer, 300, 335, &format!("{:?}", self.mode));
        title_font_renderer.draw_text(&mut buffer, 300, 350, "Mode Cycles");
        value_font_renderer.draw_text(&mut buffer, 300, 365, &format!("{:02X}", self.mode_cycles));
        title_font_renderer.draw_text(&mut buffer, 300, 380, "Frames");
        value_font_renderer.draw_text(
            &mut buffer,
            300,
            395,
            &format!("{:?}", self.frame_cycles / 70224),
        );

        // Cartridge Header Information
        title_font_renderer.draw_text(&mut buffer, 150, 10, "CARTRIDGE HEADER");
        title_font_renderer.draw_text(&mut buffer, 150, 25, "Title");
        value_font_renderer.draw_text(&mut buffer, 190, 25, &self.cartridgeState.title);
        title_font_renderer.draw_text(&mut buffer, 150, 40, "Type");
        value_font_renderer.draw_text(&mut buffer, 190, 40, &self.cartridgeState.kind);
        title_font_renderer.draw_text(&mut buffer, 150, 55, "ROM ");
        value_font_renderer.draw_text(&mut buffer, 190, 55, &self.cartridgeState.rom_size);
        title_font_renderer.draw_text(&mut buffer, 150, 70, "RAM ");
        value_font_renderer.draw_text(&mut buffer, 190, 70, &self.cartridgeState.ram_size);
        title_font_renderer.draw_text(&mut buffer, 150, 85, "Destin");
        value_font_renderer.draw_text(&mut buffer, 190, 85, &self.cartridgeState.destination);
        title_font_renderer.draw_text(&mut buffer, 150, 100, "SGB");
        value_font_renderer.draw_text(&mut buffer, 190, 100, &self.cartridgeState.sgb_flag);
        title_font_renderer.draw_text(&mut buffer, 150, 115, "Version");
        value_font_renderer.draw_text(&mut buffer, 190, 115, &self.cartridgeState.rom_version);
        title_font_renderer.draw_text(&mut buffer, 150, 130, "Licensee");
        value_font_renderer.draw_text(&mut buffer, 190, 130, &self.cartridgeState.licensee_code);

        buffer
    }
}
