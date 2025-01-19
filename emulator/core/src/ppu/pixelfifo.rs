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
    ) -> Option<ColorValue> {
        if self.bg_fifo.is_empty() {
            return None;
        }
    
        self.apply_fine_scroll(memory.read_byte(IoRegister::Scx.address()), fetcher);
        let bg_pixel = self.bg_fifo.pop_front().unwrap();
        let sprite_pixel = self.sprite_fifo.pop_front();
    
        match memory.gb_mode() {
            GameboyMode::DMG => self.mix_dmg_pixels(memory, bg_pixel, sprite_pixel).map(ColorValue::Dmg),
            GameboyMode::CGB => {
                let rgb = self.mix_cgb_pixels(memory, bg_pixel, sprite_pixel);
                Some(ColorValue::Cgb(rgb))
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
        
        
        
                // In CGB mode, background is always rendered unless explicitly disabled
        let lcdc = memory.read_byte(IoRegister::Lcdc.address());
        if lcdc & 0x01 == 0 {
            return (255, 255, 255);
        } 

        let bg_color =  memory
                .cgb()
                .get_bg_color(bg_pixel.palette, bg_pixel.color);
          
       

      /*   // If no sprite or sprites are disabled, return background color
        if let Some(sprite) = &sprite_pixel {
           
        }

        let sprite = match sprite_pixel {
            None => return bg_color,
            Some(s) if lcdc & 0x02 == 0 => {
                return bg_color;
            }
            Some(s) => s,
        };

        // If sprite color is 0, it's transparent
        if sprite.color == 0 {
            return bg_color;
        }

        // Get sprite color from CGB palette
        let sprite_color = if let Some(cgb_attrs) = sprite.cgb_attrs {
            let color = memory
                .cgb()
                .get_obj_color(cgb_attrs.palette_number, sprite.color);
          
            color
        } else {
            return bg_color;
        };

        // Check sprite priority rules
        let use_bg = (bg_pixel.bg_priority && bg_pixel.color != 0)
            || (sprite.bg_priority && bg_pixel.color != 0);

        let final_color = if use_bg {
            bg_color
        } else {
            sprite_color
        };

        final_color  */
        bg_color

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

#[derive(Clone, Copy, Debug)]
pub enum ColorValue {
    Dmg(u8),
    Cgb((u8, u8, u8)),
}
