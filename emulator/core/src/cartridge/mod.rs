use mbc0::Mbc0;
use mbc1::Mbc1;
use mbc5::Mbc5;

pub mod cartridge_header;
pub mod mbc0;
pub mod mbc1;
pub mod mbc5;

pub enum MbcType {
    None,
    Mbc0(Mbc0),
    Mbc1(Mbc1),
    Mbc3,
    Mbc5(Mbc5),
}
impl MbcType {
    pub fn read_byte(&self, address: u16) -> u8 {
        match self {
            MbcType::None => 0xFF,
            MbcType::Mbc0(mbc) => mbc.read_byte(address),
            MbcType::Mbc1(mbc) => mbc.read_byte(address),
            MbcType::Mbc5(mbc) => mbc.read_byte(address),
            _ => 0xFF,
        }
    }
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match self {
            MbcType::None => {}
            MbcType::Mbc0(mbc) => mbc.write_byte(address, value),
            MbcType::Mbc1(mbc) => mbc.write_byte(address, value),
            MbcType::Mbc5(mbc) => mbc.write_byte(address, value),
            _ => {}
        }
    }
}
