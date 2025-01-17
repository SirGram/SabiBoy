use super::{
    fetcher::{self, Fetcher},
    Sprite,
};
use crate::bus::{io_address::IoRegister, Bus, GameboyMode, MemoryInterface};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Clone, Debug, Serialize, Deserialize, Copy)]
pub struct Pixel {
    pub color: u8,
    pub sprite_priority: bool,
    pub bg_priority: bool,
    pub palette: u8,     // For CGB: 0-7, For DMG: 0 = OBP0, 1 = OBP1
    pub is_sprite: bool, // Added to track if pixel is from sprite or background
    pub cgb_attrs: Option<CgbAttributes>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy)]
pub struct CgbAttributes {
    pub vram_bank: u8,
    pub palette_number: u8,
    pub priority: bool,
}

impl Pixel {
    pub fn new_bg<M: MemoryInterface>(memory: &M, color: u8, attrs: u8) -> Self {
        match memory.gb_mode() {
            GameboyMode::DMG => Self {
                color,
                sprite_priority: false,
                bg_priority: false,
                palette: 0,
                is_sprite: false,
                cgb_attrs: None,
            },
            GameboyMode::CGB => Self {
                color,
                sprite_priority: false,
                bg_priority: attrs & 0x80 != 0,
                palette: attrs & 0x07,
                is_sprite: false,
                cgb_attrs: Some(CgbAttributes {
                    vram_bank: (attrs >> 3) & 1,
                    palette_number: attrs & 0x07,
                    priority: attrs & 0x80 != 0,
                }),
            },
        }
    }

    pub fn new_sprite<M: MemoryInterface>(memory: &M, color: u8, attrs: u8) -> Self {
        match memory.gb_mode() {
            GameboyMode::DMG => Self {
                color,
                sprite_priority: false,
                bg_priority: attrs & 0x80 != 0,
                palette: if (attrs & 0x10) != 0 { 1 } else { 0 },
                is_sprite: true,
                cgb_attrs: None,
            },
            GameboyMode::CGB => Self {
                color,
                sprite_priority: false,
                bg_priority: attrs & 0x80 != 0,
                palette: attrs & 0x07,
                is_sprite: true,
                cgb_attrs: Some(CgbAttributes {
                    vram_bank: (attrs >> 3) & 1,
                    palette_number: attrs & 0x07,
                    priority: attrs & 0x80 != 0,
                }),
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PixelFifo {
    pub bg_fifo: VecDeque<Pixel>,
    pub sprite_fifo: VecDeque<Pixel>,
    fine_scroll_applied: bool,
}

impl PixelFifo {
    pub fn new() -> Self {
        Self {
            bg_fifo: VecDeque::new(),
            sprite_fifo: VecDeque::new(),
            fine_scroll_applied: false,
        }
    }

    pub fn reset(&mut self) {
        self.bg_fifo.clear();
        self.sprite_fifo.clear();
        self.fine_scroll_applied = false;
    }

    pub fn pop_pixel<M: MemoryInterface>(
        &mut self,
        memory: &M,
        fetcher: &mut Fetcher,
    ) -> Option<u8> {
        if self.bg_fifo.is_empty() {
            return None;
        }

        self.apply_fine_scroll(memory.read_byte(IoRegister::Scx.address()), fetcher);
        let bg_pixel = self.bg_fifo.pop_front().unwrap();
        let sprite_pixel = self.sprite_fifo.pop_front();

        match memory.gb_mode() {
            GameboyMode::DMG => self.mix_dmg_pixels(memory, bg_pixel, sprite_pixel),
            GameboyMode::CGB => {
                let (r, g, b) = self.mix_cgb_pixels(memory, bg_pixel, sprite_pixel);
                // Convert RGB to grayscale for compatibility with DMG output
                Some(((r as u16 + g as u16 + b as u16) / 3) as u8)
            }
        }
    }

    fn mix_dmg_pixels<M: MemoryInterface>(
        &self,
        memory: &M,
        bg_pixel: Pixel,
        sprite_pixel: Option<Pixel>,
    ) -> Option<u8> {
        let lcdc = memory.read_byte(IoRegister::Lcdc.address());
        let mut final_color = bg_pixel.color;

        if lcdc & 0x01 == 0 {
            final_color = 0;
        }

        let bgp = memory.read_byte(IoRegister::Bgp.address());
        final_color = (bgp >> (final_color * 2)) & 0x03;

        if let Some(sprite) = sprite_pixel {
            if lcdc & 0x02 != 0 && sprite.color != 0 {
                if !sprite.bg_priority || final_color == 0 {
                    let obp = if sprite.palette > 0 {
                        memory.read_byte(IoRegister::Obp1.address())
                    } else {
                        memory.read_byte(IoRegister::Obp0.address())
                    };
                    final_color = (obp >> (sprite.color * 2)) & 0x03;
                }
            }
        }

        Some(final_color)
    }

    fn mix_cgb_pixels<M: MemoryInterface>(
        &self,
        memory: &M,
        bg_pixel: Pixel,
        sprite_pixel: Option<Pixel>,
    ) -> (u8, u8, u8) {
        // Returns RGB color
        let lcdc = memory.read_byte(IoRegister::Lcdc.address());

        // In CGB mode, background is always rendered unless explicitly disabled
        if lcdc & 0x01 == 0 {
            return (255, 255, 255); // White screen when background is disabled
        }

        // Get the background color first
        let bg_color = if bg_pixel.color == 0 {
            (255, 255, 255) // White for color 0
        } else {
            memory.cgb().get_bg_color(bg_pixel.palette, bg_pixel.color)
        };

        // If no sprite or sprites are disabled, return background color
        if sprite_pixel.is_none() || lcdc & 0x02 == 0 {
            return bg_color;
        }

        let sprite = sprite_pixel.unwrap();

        // If sprite color is 0, it's transparent
        if sprite.color == 0 {
            return bg_color;
        }

        // Check sprite priority rules
        let use_bg = (bg_pixel.bg_priority && bg_pixel.color != 0) || // BG priority bit set and BG not transparent
                     (sprite.bg_priority && bg_pixel.color != 0); // Sprite priority bit set and BG not transparent

        if use_bg {
            bg_color
        } else {
            memory.cgb().get_obj_color(sprite.palette, sprite.color)
        }
    }

    pub fn bg_pixel_count(&self) -> usize {
        self.bg_fifo.len()
    }

    pub fn sprite_pixel_count(&self) -> usize {
        self.sprite_fifo.len()
    }

    pub fn is_paused(&self, sprite_fetcher_active: bool, fetcher_active: bool) -> bool {
        self.bg_fifo.len() == 0 || sprite_fetcher_active || fetcher_active
    }

    pub fn apply_fine_scroll(&mut self, scx: u8, fetcher: &mut Fetcher) {
        if !self.fine_scroll_applied {
            let fine_scroll_offset = (scx & 0x07) as usize;

            for _ in 0..fine_scroll_offset {
                if !self.bg_fifo.is_empty() {
                    self.bg_fifo.pop_front();
                    fetcher.x_pos_counter += 1;
                }
            }
            self.fine_scroll_applied = true;
        }
    }
}
