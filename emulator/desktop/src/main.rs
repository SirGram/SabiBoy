use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, SampleRate, StreamConfig};
use gameboy_core::{self};
use minifb::{Key, Window, WindowOptions};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

mod debug_window;
fn main() {
    // Parse command line arguments
    let debug_enabled = std::env::args().any(|arg| arg == "--debug" || arg == "-d");
    let turbo_mode = std::env::args().any(|arg| arg == "--turbo" || arg == "-t");
    let audio_disabled = std::env::args().any(|arg| arg == "--audio" || arg == "-a");
    let mut window = set_up_window(turbo_mode);
    let mut debug_window = if debug_enabled {
        Some(debug_window::DebugWindow::new())
    } else {
        None
    };

    // Initialize GameBoy
    let palette: [u32; 4] = [0x9bbc0f, 0x8bac0f, 0x306230, 0x0f380f];
    let mut gameboy = gameboy_core::gameboy::Gameboy::new(palette);

    if audio_disabled {
        gameboy.apu.toggle_audio();
    }

    gameboy.load_rom(include_bytes!(
        "../../../games/pokemon-red-jap/rom.gb" /*   "../../../games/tennis--1/rom.gb" */
    ));

    /*  if let Ok(save_state) = std::fs::read("./rom.gb.state") {
           if let Err(e) = gameboy.load_state(save_state) {
               println!("Failed to load state: {}", e);
           }
       }
    */
    // Setup audio
    let audio_output = match AudioOutput::new() {
        Ok(audio) => Some(audio),
        Err(e) => {
            println!("Audio disabled - couldn't initialize: {}", e);
            None
        }
    };

    run(
        &mut window,
        &mut gameboy,
        &mut debug_window,
        audio_output.as_ref(),
        turbo_mode,
    );
}
fn set_up_window(turbo_mode: bool) -> Window {
    let width = 160;
    let height = 144;
    let window_options = WindowOptions {
        scale: minifb::Scale::X2,
        borderless: true,
        ..WindowOptions::default()
    };

    let mut window =
        Window::new("SabiBoy", width, height, window_options).expect("Failed to create window");

    // Only limit update rate if not in turbo mode
    if !turbo_mode {
        window.limit_update_rate(Some(std::time::Duration::from_micros(16667)));
    }
    window
}

fn run(
    window: &mut Window,
    gameboy: &mut gameboy_core::gameboy::Gameboy,
    debug_window: &mut Option<debug_window::DebugWindow>,
    audio_output: Option<&AudioOutput>,
    turbo_mode: bool,
) {
    let mut last_fps_check = Instant::now();
    let mut frames = 0;
    let mut current_fps = 0;
    let mut buffer = vec![0u32; 160 * 144];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_start = Instant::now();

        // Always run exactly 1 frame per iteration
        gameboy.run_frame();
        frames += 1;

        // Update display
        let gb_buffer = gameboy.bus.ppu.get_frame_buffer();
        buffer.copy_from_slice(gb_buffer);
        window.update_with_buffer(&buffer, 160, 144).unwrap();

        // FPS calculation
        if last_fps_check.elapsed() > Duration::from_secs(1) {
            current_fps = frames;
            frames = 0;
            last_fps_check = Instant::now();
            window.set_title(&format!("SabiBoy - {} FPS{}", 
                current_fps,
                if turbo_mode { " (Turbo)" } else { "" }
            ));
        }

        // Only limit FPS in non-turbo mode
        if !turbo_mode {
            let frame_time = frame_start.elapsed();
            if frame_time < Duration::from_micros(16_667) {
                std::thread::sleep(Duration::from_micros(16_667) - frame_time);
            }
        }

        // Rest of the code remains the same...
        handle_input(window, gameboy);
        
        if let Some(debug_window) = debug_window {
            debug_window.update(&gameboy.cpu, &gameboy.bus, &gameboy.bus.ppu, current_fps);
            debug_window.render();
        }

        if let Some(audio) = audio_output {
            let samples = gameboy.apu.get_samples();
            audio.add_samples(&samples);
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
    gameboy.bus.joypad.update_keys(new_keys);

    // Handle additional input: Save state
    if window.is_key_down(Key::Key1) {
        let save = gameboy.save_state().expect("Failed to save state");
        std::fs::write("rom.gb.state", save).expect("Failed to write state to file");
    }
}

pub struct AudioOutput {
    stream: cpal::Stream,
    samples: Arc<Mutex<Vec<f32>>>,
}

impl AudioOutput {
    pub fn new() -> Result<Self, anyhow::Error> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("No output device available");

        // Configure stream parameters
        let sample_rate = 48_000;
        let buffer_size = 1600;

        // Create thread-safe sample buffer
        let samples = Arc::new(Mutex::new(Vec::new()));
        let samples_clone = Arc::clone(&samples);

        // Configure stream
        let stream_config = StreamConfig {
            channels: 2,
            sample_rate: SampleRate(sample_rate),
            buffer_size: cpal::BufferSize::Fixed(buffer_size),
        };

        let stream = device.build_output_stream(
            &stream_config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let mut samples_lock = samples_clone.lock().unwrap();

                // Fill buffer with available samples or silence
                let fill_len = data.len().min(samples_lock.len());
                data[..fill_len].copy_from_slice(&samples_lock[..fill_len]);

                // Fill remaining with silence if needed
                if fill_len < data.len() {
                    data[fill_len..].fill(0.0);
                }

                // Remove used samples
                samples_lock.drain(0..fill_len);
            },
            |err| eprintln!("Audio stream error: {:?}", err),
            None,
        )?;

        // Start the stream
        stream.play()?;

        Ok(Self { stream, samples })
    }

    // Method to add samples from APU
    pub fn add_samples(&self, new_samples: &[f32]) {
        let mut samples_lock = self.samples.lock().unwrap();
        samples_lock.extend_from_slice(new_samples);
    }
}
