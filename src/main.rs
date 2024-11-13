use cpu::flags::Flags;

mod bus;
mod cpu;
mod gameboy;
mod ppu;
mod test;
mod test2;
mod timer;

fn main() {
    let mut gb = gameboy::GameBoy::new();

    // Set initial CPU state to match test expectations
    gb.cpu.a = 0x01;
    gb.cpu.f.set(Flags::N, false);
    gb.cpu.f.set(Flags::H, true);
    gb.cpu.f.set(Flags::C, true);
    gb.cpu.f.set(Flags::Z, true);
    gb.cpu.b = 0x00;
    gb.cpu.c = 0x13;
    gb.cpu.d = 0x00;
    gb.cpu.e = 0xD8;
    gb.cpu.h = 0x01;
    gb.cpu.l = 0x4D;
    gb.cpu.sp = 0xFFFE;
    gb.cpu.pc = 0x0100;

    gb.bus.borrow_mut().write_byte(0xFF44, 0x90);
   
    gb.load_rom(include_bytes!("../test/blargg/02-interrupts.gb"));
    gb.run(); 

}
