use std::{cell::RefCell, io, rc::Rc};

use minifb::Key;

use crate::bus::{io_address::IoRegister, Bus};

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
    pub fn update(&mut self, window: &mut minifb::Window)-> bool {
        /*
            0 = Pressed, 1 = Released
            Bit 0 = Right
            Bit 1 = Left
            Bit 2 = Up
            Bit 3 = Down
            Bit 4 = A
            Bit 5 = B
            Bit 6 = Select
            Bit 7 = Start
        */
        let old_keys = self.keys;
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
        if window.is_key_down(Key::S) {
            self.keys &= !0x40;
        }
        if window.is_key_down(Key::Enter) {
            self.keys &= !0x80;
        }

         // Only trigger interrupt if there's a change in key state and a key is pressed
         if old_keys != self.keys && self.keys != 0xFF {
            // Request joypad interrupt
            true;
        }
        false
    }
   
   
    pub fn read(&self) -> u8 {
        let mut result = 0xCF; // Bit 6 and 7 are always set. Buttons released

        let direction_select = self.register & 0x10 == 0;
        let action_select = self.register & 0x20 == 0;

        if direction_select {
            result &= !(0x0F & !self.keys);  
        }
        if action_select {
            result &= !(0x0F & !(self.keys >> 4));  
        }

        result
    }
    pub fn write(&mut self, value: u8) {
         // Only bits 4-5 are writable
         self.register = (value & 0x30) | (self.register & 0xCF);
    }
}
