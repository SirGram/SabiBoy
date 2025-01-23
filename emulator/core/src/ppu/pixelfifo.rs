use super::{
    fetcher::{self, Fetcher},
    Sprite,
};
use crate::bus::{io_address::IoRegister, Bus, GameboyMode};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Clone, Debug, Serialize, Deserialize, Copy)]
pub struct Pixel {
    pub color: u8,
    pub bg_priority: bool,
    pub palette: u8, // For CGB: 0-7, For DMG: 0 = OBP0, 1 = OBP1
    pub cgb_attrs: Option<CgbAttributes>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy)]
pub struct CgbAttributes {
    pub sprite_priority: bool,
}

impl Pixel {
    pub fn new_bg( color: u8, attrs: u8, gb_mode: GameboyMode) -> Self {
        match gb_mode {
            GameboyMode::DMG => Self {
                color,
                bg_priority: false,
                palette: 0,
                cgb_attrs: None,
            },
            GameboyMode::CGB => Self {
                color,
                bg_priority: attrs & 0x80 != 0,
                palette: attrs & 0x07,
                cgb_attrs: Some(CgbAttributes {
                    sprite_priority: false,
                }),
            },
        }
    }

    pub fn new_sprite( color: u8, attrs: u8 , gb_mode: GameboyMode) -> Self {
        match gb_mode {
            GameboyMode::DMG => Self {
                color,
                bg_priority: attrs & 0x80 != 0,
                palette: if (attrs & 0x10) != 0 { 1 } else { 0 },
                cgb_attrs: None,
            },
            GameboyMode::CGB => Self {
                color,
                bg_priority: attrs & 0x80 != 0,
                palette: attrs & 0x07,
                cgb_attrs: Some(CgbAttributes {
                    sprite_priority: attrs & 0x80 != 0,
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

    pub fn pop_pixel(
        &mut self,
        
        fetcher: &mut Fetcher,
        scx: u8,
        gb_mode: GameboyMode,
        lcdc: u8,
        bgp: u8,
        obp0: u8,
        obp1: u8,


    ) -> Option<ColorValue> {
        if self.bg_fifo.is_empty() {
            return None;
        }

        self.apply_fine_scroll(scx, fetcher);
        let bg_pixel = self.bg_fifo.pop_front().unwrap();
        let sprite_pixel = self.sprite_fifo.pop_front();

        match gb_mode {
            GameboyMode::DMG => self
                .mix_dmg_pixels( bg_pixel, sprite_pixel , lcdc, bgp, obp0, obp1)
                .map(ColorValue::Dmg),
            GameboyMode::CGB => {
                let rgb = 0;
                Some(ColorValue::Cgb(rgb))
            }
        }
    }

    fn mix_dmg_pixels(
        &self,
        
        bg_pixel: Pixel,
        sprite_pixel: Option<Pixel>,
        lcdc: u8,
        bgp: u8,
        obp0: u8,
        obp1: u8,
    ) -> Option<u8> {
        let mut final_color = bg_pixel.color;

        if lcdc & 0x01 == 0 {
            final_color = 0;
        }

        final_color = (bgp >> (final_color * 2)) & 0x03;

        if let Some(sprite) = sprite_pixel {
            if lcdc & 0x02 != 0 && sprite.color != 0 {
                if !sprite.bg_priority || final_color == 0 {
                    let obp = if sprite.palette > 0 {
                        obp1
                    } else {
                        obp0
                    };
                    final_color = (obp >> (sprite.color * 2)) & 0x03;
                }
            }
        }

        Some(final_color)
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

    fn apply_fine_scroll(&mut self, scx: u8, fetcher: &mut Fetcher) {
        if !self.fine_scroll_applied {
            // Do not apply fine scroll for window tiles
            if fetcher.is_window_fetch {
                self.fine_scroll_applied = true;
                return;
            }

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

#[derive(Clone, Copy, Debug)]
pub enum ColorValue {
    Dmg(u8),
    Cgb(u32),
}
