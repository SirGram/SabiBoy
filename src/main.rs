mod bus;
mod cpu;
mod gameboy;
mod ppu;
mod test;
mod timer;

fn main() {
    let mut gameboy = gameboy::GameBoy::new();
    gameboy.run();

    /* let test = include_str!("../test/sm83/69.json");
    gameboy.run_test(test,&mut 0);
    //Failed: 196 (calls) 198-458 (CB)
    gameboy.run_tests(498, None);
     */
}
