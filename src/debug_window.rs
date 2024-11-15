use minifb::{Scale, Window, WindowOptions};
use minifb_fonts::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::bus;
use crate::bus::io_address::IoRegister;
use crate::cpu::CPU;
// GREEN COLORS
const COLORS: [u32; 4] = [0xA8D08D, 0x6A8E3C, 0x3A5D1D, 0x1F3C06];    
const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;
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
    io_registers: Vec<(String, u8)>,
    tile_data: [u8; 0x1800],
    bg_tilemap: [u8; 0x800],
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
        }
    }

    pub fn update(&mut self, cpu: &CPU, bus: &Rc<RefCell<bus::Bus>>) {
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

        // Read background tilemap (0x9800-0x9BFF for BG Map 0)
        let lcdc = bus.borrow().read_byte(IoRegister::Lcdc.address());
        let tilemap_base = if (lcdc & 0x08) != 0 { 0x9C00 } else { 0x9800 };

        for i in 0..0x800 {
            self.bg_tilemap[i] = bus.borrow().read_byte(tilemap_base + i as u16);
        }
    }
    fn render_tile_data(&self, buffer: &mut Vec<u32>, start_x: usize, start_y: usize) {
        // Constants for tile layout
        let tiles_per_row = 12; // Increased from 12 for better layout
        let total_tiles = 384;
        let tile_size = 8;

        for tile_index in 0..total_tiles {
            let tile_x = tile_index % tiles_per_row;
            let tile_y = tile_index / tiles_per_row;

            let screen_x = start_x + (tile_x * tile_size);
            let screen_y = start_y + (tile_y * tile_size);

            let tile_offset = tile_index * 16;

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

    pub fn render(&mut self) {
        let debug_buffer = self.debug_buffer();
        self.window
            .update_with_buffer(&debug_buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }

    fn render_background_tilemap(&self, buffer: &mut Vec<u32>, start_x: usize, start_y: usize) {
        for map_y in 0..32 {
            for map_x in 0..32 {
                let tile_index = self.bg_tilemap[map_y * 32 + map_x] as usize;
                let tile_offset = tile_index * 16;

                for y in 0..8 {
                    let screen_y = start_y + (map_y * 8) + (y);

                    let low_byte = self.tile_data[tile_offset + (y * 2)];
                    let high_byte = self.tile_data[tile_offset + (y * 2) + 1];

                    for x in 0..8 {
                        let screen_x = start_x + (map_x * 8) + x;

                        let color_bit = 7 - x;
                        let color_index =
                            (((high_byte >> color_bit) & 1) << 1) | ((low_byte >> color_bit) & 1);

                        let buffer_index = screen_y * WINDOW_WIDTH + screen_x;
                        if buffer_index < buffer.len() {
                            buffer[buffer_index] = COLORS[color_index as usize];
                        }
                    }
                }
            }
        }
    }
    fn render_viewport(&self, buffer: &mut Vec<u32>, start_x: usize, start_y: usize) {
        // viewport  is 160x144 pixels
        let color = 0xFF0000; //red color
        for y in 0..144 {
            for x in 0..160 {
                let buffer_index = (y + start_y) * WINDOW_WIDTH + x + start_x;
                if buffer_index < buffer.len() {
                    if x == 0 || x == 159 || y == 0 || y == 143 {
                        buffer[buffer_index] = color;
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
        for (i, reg) in reg_labels.iter().enumerate() {
            text_font_renderer.draw_text(&mut buffer, 10, 30 + i * 15, reg);
            value_font_renderer.draw_text(
                &mut buffer,
                30,
                30 + i * 15,
                &format!("{:02X}", self.cpu_registers[i]),
            );
        }

        // Special registers
        text_font_renderer.draw_text(&mut buffer, 10, 160, "SP");
        value_font_renderer.draw_text(&mut buffer, 40, 160, &format!("{:04X}", self.sp));
        text_font_renderer.draw_text(&mut buffer, 10, 175, "PC");
        value_font_renderer.draw_text(&mut buffer, 40, 175, &format!("{:04X}", self.pc));
        text_font_renderer.draw_text(&mut buffer, 10, 190, "OP");
        value_font_renderer.draw_text(&mut buffer, 40, 190, &format!("{:02X}", self.op));

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
        title_font_renderer.draw_text(&mut buffer, 400, 5, "Background Tilemap");
        self.render_background_tilemap(&mut buffer, 400, 20);
        self.render_viewport(&mut buffer, 400, 20);

        title_font_renderer.draw_text(&mut buffer, 300, 5, "VRAM");
        self.render_tile_data(&mut buffer, 300, 20);

        buffer
    }
}
