use cpu::flags::Flags;
use serde::de;

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
    let mut gb = gameboy::GameBoy::new(true);

    gb.set_power_up_sequence();
    gb.load_rom(include_bytes!("../test/tennis.gb"));
    gb.run();
}
 