use mbc0::{Mbc0, Mbc0State};
use mbc1::{Mbc1, Mbc1State};
use mbc3::{Mbc3, Mbc3State};
use mbc5::{Mbc5, Mbc5State};
use serde::{Deserialize, Serialize};

pub mod cartridge_header;
pub mod mbc0;
pub mod mbc1;
pub mod mbc3;
pub mod mbc5;
#[derive(Clone, Debug)]
pub enum MbcType {
    None,
    Mbc0(Mbc0),
    Mbc1(Mbc1),
    Mbc3(Mbc3),
    Mbc5(Mbc5),
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MbcTypeState {
    None,
    Mbc0(Mbc0State),
    Mbc1(Mbc1State),
    Mbc3(Mbc3State),
    Mbc5(Mbc5State),
}
impl MbcType {
    pub fn save_state(&self) -> MbcTypeState {
        match self {
            MbcType::None => MbcTypeState::None,
            MbcType::Mbc0(mbc) => MbcTypeState::Mbc0(mbc.save_state()),
            MbcType::Mbc1(mbc) => MbcTypeState::Mbc1(mbc.save_state()),
            MbcType::Mbc3(mbc) => MbcTypeState::Mbc3(mbc.save_state()),
            MbcType::Mbc5(mbc) => MbcTypeState::Mbc5(mbc.save_state()),
        }
    }

    pub fn load_state(&mut self, state: MbcTypeState) {
        match (self, state) {
            (MbcType::Mbc0(mbc), MbcTypeState::Mbc0(state)) => mbc.load_state(state),
            (MbcType::Mbc1(mbc), MbcTypeState::Mbc1(state)) => mbc.load_state(state),
            (MbcType::Mbc3(mbc), MbcTypeState::Mbc3(state)) => mbc.load_state(state),
            (MbcType::Mbc5(mbc), MbcTypeState::Mbc5(state)) => mbc.load_state(state),
            _ => {} // Handle mismatched types or None case
        }
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        match self {
            MbcType::None => 0xFF,
            MbcType::Mbc0(mbc) => mbc.read_byte(address),
            MbcType::Mbc1(mbc) => mbc.read_byte(address),
            MbcType::Mbc3(mbc) => mbc.read_byte(address),
            MbcType::Mbc5(mbc) => mbc.read_byte(address),
            _ => 0xFF,
        }
    }
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match self {
            MbcType::None => {}
            MbcType::Mbc0(mbc) => mbc.write_byte(address, value),
            MbcType::Mbc1(mbc) => mbc.write_byte(address, value),
            MbcType::Mbc3(mbc) => mbc.write_byte(address, value),
            MbcType::Mbc5(mbc) => mbc.write_byte(address, value),
            _ => {}
        }
    }
    pub fn tick(&mut self) {
        if let MbcType::Mbc3(mbc) = self {
            if let Some(rtc) = mbc.rtc.as_mut() {
                rtc.tick();
            }
        }
    }
}
