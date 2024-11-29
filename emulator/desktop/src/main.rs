use gameboy_core::{self, gameboy, joyp::JoyPadKey};
use minifb::{Key, Window, WindowOptions};
use std::{
    env::Args,
    time::{Duration, Instant},
};

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
    gameboy.load_rom(include_bytes!("../../../test/dmg-acid2.gb"));

    run(&mut window, &mut gameboy, &mut debug_window);
}

fn set_up_window() -> Window {
    let width = 160;
    let height = 144;
    let window_options = WindowOptions {
        scale: minifb::Scale::X2,
        borderless: true,
        ..WindowOptions::default()
    };

    let mut window =
        Window::new("SabiBoy", width, height, window_options).expect("Failed to create window");

    window.limit_update_rate(Some(std::time::Duration::from_micros(16667)));
    window
}

fn run(
    window: &mut Window,
    gameboy: &mut gameboy_core::gameboy::Gameboy,
    debug_window: &mut Option<debug_window::DebugWindow>,
) {
    let target_frame_time = Duration::from_micros(16_667); // 60 fps
    let mut last_fps_check = Instant::now();
    let mut frames = 0;
    let mut current_fps = 0;

    // Buffer to hold the converted pixels
    let mut buffer = vec![0u32; 160 * 144];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_start_time = Instant::now();
        gameboy.run_frame();

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
        window
            .update_with_buffer(&buffer, 160, 144)
            .expect("Failed to update window");

        // FPS calculation
        frames += 1;
        if last_fps_check.elapsed() > Duration::from_secs(1) {
            current_fps = frames;
            frames = 0;
            last_fps_check = Instant::now();
        }

        // Frame timing
        let frame_time = frame_start_time.elapsed();
        if frame_time < target_frame_time {
            std::thread::sleep(target_frame_time - frame_time);
        }

        // update key input
        handle_input(window, gameboy);

        // update debug window
        if let Some(debug_window) = debug_window {
            debug_window.update(&gameboy.cpu, &gameboy.bus, &gameboy.ppu, current_fps);
            debug_window.render();
        }
    }
}

fn handle_input(window: &mut Window, gameboy: &mut gameboy_core::gameboy::Gameboy) {
    use gameboy_core::joyp::JoyPadKey;
    let keys = [
        (Key::Right, JoyPadKey::Right),
        (Key::Left, JoyPadKey::Left),
        (Key::Up, JoyPadKey::Up),
        (Key::Down, JoyPadKey::Down),
        (Key::Z, JoyPadKey::A),
        (Key::X, JoyPadKey::B),
        (Key::Backspace, JoyPadKey::Select),
        (Key::Enter, JoyPadKey::Start),
    ];
    let mut new_keys: u8 = 0xFF; // Start with all keys released
    for (minifb_key, gb_key) in keys.iter() {
        if window.is_key_down(*minifb_key) {
            new_keys &= !(gb_key.bit_mask()); // Set key as pressed (bit 0)
        }
    }
    gameboy.bus.borrow_mut().joypad.update_keys(new_keys);
}
