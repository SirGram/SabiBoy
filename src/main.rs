use log::info;

mod bus;
mod cartridge;
mod cpu;
mod debug_window;
mod gameboy;
mod joyp;
mod ppu;
mod test;
mod test2;
mod timer;

fn main() {
    env_logger::init(); // cargo run > output.log 2>&1
    info!("Program started");

    let mut gb = gameboy::GameBoy::new(true);

    gb.set_power_up_sequence();
    gb.load_rom(include_bytes!("../test/dr_mario.gb"));

    gb.run();
}
