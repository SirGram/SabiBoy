mod bus;
mod cpu;
mod gameboy;

fn main() {
    let mut gameboy = gameboy::GameBoy::new();
    gameboy.load_rom(&include_bytes!("../test/01-special.gb")[..]);
    gameboy.run();
}
