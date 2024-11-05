use bitflags::bitflags;

bitflags! {
    pub struct Flags: u8 {
        const Z = 0b10000000; // zero
        const N = 0b01000000; // subtraction
        const H = 0b00100000; // half carry
        const C = 0b00010000; // carry
    }
}
