use std::{env::Args, time::{Duration, Instant}};
use gameboy_core::{self, gameboy};
use minifb::{Key, Window, WindowOptions};

mod debug_window;
fn main() {
    // Parse command line arguments
    let debug_enabled = std::env::args().any(|arg| arg == "--debug" || arg == "-d");
    let mut window = set_up_window();
    let mut debug_window = if debug_enabled {
        Some(debug_window::DebugWindow::new())
    } else {
        None
    };

    // Initialize GameBoy
    let mut gameboy = gameboy_core::gameboy::Gameboy::new();
    gameboy.set_power_up_sequence();
    gameboy.load_rom(include_bytes!("../../wasm/test/tennis.gb"));

    run(&mut window, &mut gameboy, &mut debug_window);
}

fn set_up_window()-> Window {
    let width = 160;
    let height = 144;
    let window_options = WindowOptions {
        scale: minifb::Scale::X2,
        borderless: true,
        ..WindowOptions::default()
    };

    let mut window = Window::new(
        "SabiBoy",
        width,
        height,
        window_options,
    ).expect("Failed to create window");
    
    window.limit_update_rate(Some(std::time::Duration::from_micros(16667)));
    window
}
    

 fn run(window: &mut Window, gameboy: &mut gameboy_core::gameboy::Gameboy, debug_window: &mut Option<debug_window::DebugWindow>) {
    let cycles_per_frame = 70_224;
    let target_frame_time = Duration::from_micros(16_667); // 60 fps
    let mut last_fps_check = Instant::now();
    let mut frames = 0;
    let mut current_fps = 0;
    
    // Buffer to hold the converted pixels
    let mut buffer = vec![0u32; 160 * 144];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_start_time = Instant::now();
        let mut cycles_this_frame = 0;

        // Run one frame worth of emulation
        while cycles_this_frame < cycles_per_frame {
            gameboy.tick();
            cycles_this_frame += gameboy.cpu.cycles;
        }

        // Get the frame buffer from PPU and convert colors
        let gb_buffer = gameboy.ppu.get_frame_buffer();
        for (i, &color) in gb_buffer.iter().enumerate() {
            // Convert GameBoy color to RGB888 format that minifb expects
            let r = ((color >> 16) & 0xFF) as u32;
            let g = ((color >> 8) & 0xFF) as u32;
            let b = (color & 0xFF) as u32;
            
            // Pack RGB values into a single u32 (0RGB)
            buffer[i] = (r << 16) | (g << 8) | b;
        }

        // Update the window with the new frame
        window.update_with_buffer(
            &buffer,
            160,
            144
        ).expect("Failed to update window");

        // FPS calculation
        frames += 1;
        if last_fps_check.elapsed() > Duration::from_secs(1) {
            current_fps = frames;
            frames = 0;
            last_fps_check = Instant::now();
            println!("FPS: {}", current_fps);
        }

        // Frame timing
        let frame_time = frame_start_time.elapsed();
        if frame_time < target_frame_time {
            std::thread::sleep(target_frame_time - frame_time);
        }

        if let Some(debug_window) = debug_window {
            debug_window.update(&gameboy.cpu, &gameboy.bus, &gameboy.ppu, current_fps);
            debug_window.render();
        }
        
    }
}