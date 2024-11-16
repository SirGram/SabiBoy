use std::io;

use minifb::Key;

pub struct Joypad {
    keys: u8,
    register: u8,
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            keys: 0xFF,
            register: 0xFF,
        }
    }
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
        if window.is_key_down(Key::Right) {
            self.keys &= !0x01;
        }
        if window.is_key_down(Key::Left) {
            self.keys &= !0x02;
        }
        if window.is_key_down(Key::Up) {
            self.keys &= !0x04;
        }
        if window.is_key_down(Key::Down) {
            self.keys &= !0x08;
        }
        if window.is_key_down(Key::A) {
            self.keys &= !0x10;
        }
        if window.is_key_down(Key::B) {
            self.keys &= !0x20;
        }
        if window.is_key_down(Key::Enter) {
            self.keys &= !0x40;
        }
        if window.is_key_down(Key::S) {
            self.keys &= !0x80;
        }
    }
    pub fn read(&self) -> u8 {
        let mut result = 0xC0; // Bit 6 and 7 are always set
                               // Send keys being pressed when rom reads
        let action_select = self.register & 0x10 == 1;
        let direction_select = self.register & 0x20 == 1;

        if action_select && direction_select {
            result |= 0x0F; // When both are 1, no buttons are pressed
        } else if action_select {
            result |= (self.keys >> 4) & 0x0F;
        } else if direction_select {
            result |= self.keys & 0x0F;
        } else {
            result |= ((self.keys >> 4) & 0x0F) & (self.keys & 0x0F); // either button pressed
        }

        result
    }
    pub fn write(&mut self, value: u8) {
        self.register = value;
    }
}
