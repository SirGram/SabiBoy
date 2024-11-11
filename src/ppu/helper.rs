use crate::bus::io_address::IoRegister;
use crate::ppu::Sprite;

const SPRITE_HEGHT_TALL: u8 = 16;
const SPRITE_HEIGHT_NORMAL: u8 = 8;
const MAX_SPRITES_IN_BUFFER: usize = 10;

pub fn should_add_sprite(sprite: &Sprite, ly: u8, buffer_count: usize) -> bool {
    /*
    Sprite X-Position must be greater than 0
    LY + 16 must be greater than or equal to Sprite Y-Position
    LY + 16 must be less than Sprite Y-Position + Sprite Height (8 in Normal Mode, 16 in Tall-Sprite-Mode)
    The amount of sprites already stored in the OAM Buffer must be less than 10
    */
    let sprite_size = if ly & 0b1000_0100 == 0 {
        SPRITE_HEIGHT_NORMAL
    } else {
        SPRITE_HEGHT_TALL
    };

    sprite.x_pos > 0
        && ly + 16 >= sprite.y_pos
        && ly + 16 < sprite.y_pos + sprite_size
        && buffer_count < MAX_SPRITES_IN_BUFFER
}