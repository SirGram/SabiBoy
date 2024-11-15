use minifb::Key;

pub struct Joypad {
    keys: u8,
}

impl Joypad {
    pub fn update(&mut self, window: &mut minifb::Window) {
        /*
            0 = Pressed, 1 = Released
            Bit 0 = Select
            Bit 1 = Start
            Bit 2 = Up
            Bit 3 = Down
            Bit 4 = Left
            Bit 5 = Right
            Bit 6 = A
            Bit 7 = B
        */
        self.keys = 0xFF;
        if window.is_key_down(Key::S) {
            self.keys &= 0xFE;
        }
        if window.is_key_down(Key::Enter) {
            self.keys &= 0xFD;
        }
        if window.is_key_down(Key::Up) {
            self.keys &= 0xFB;
        }
        if window.is_key_down(Key::Down) {
            self.keys &= 0xF7;
        }
        if window.is_key_down(Key::Left) {
            self.keys &= 0xEF;
        }
        if window.is_key_down(Key::Right) {
            self.keys &= 0xDF;
        }
        if window.is_key_down(Key::A) {
            self.keys &= 0xBF;
        }
        if window.is_key_down(Key::B) {
            self.keys &= 0x7F;
        }
    }

    pub fn get_state(&self) -> u8 {
        self.keys
    }
}
