use crate::bus::{self, io_address::IoRegister, Bus, MemoryInterface};

#[derive(Debug, Clone)]
pub struct Channel4 {
    length_timer: u8,
    disabled: bool,
    period_timer: u8,
    current_volume: u8,
    lfsr: u16,
}

impl Channel4 {
    /*
        NR41 FF20 --LL LLLL Length load (64-L)
        NR42 FF21 VVVV APPP Starting volume, Envelope add mode, period
        NR43 FF22 SSSS WDDD Clock shift, Width mode of LFSR, Divisor code
        NR44 FF23 TL-- ---- Trigger, Length enable
    */
    pub fn new() -> Self {
        Self {
            length_timer: 0,
            disabled: false,
            period_timer: 0,
            current_volume: 0,
            lfsr: 0x7FFF,
        }
    }

    pub fn tick<M: MemoryInterface>(&mut self, memory: &mut M) {
        let is_triggered =
            memory.read_byte(bus::io_address::IoRegister::Nr44.address()) & 0b10000000 != 0;
        if is_triggered {
            self.trigger(memory);
        }
        self.period_timer -= 1;
        if self.period_timer == 0 {
            self.period_timer = self.calculate_period(memory);
            let lfsr_bit = (self.lfsr ^ (self.lfsr >> 1)) & 1;
            self.lfsr = (self.lfsr >> 1) | (lfsr_bit << 14);
            if memory.read_byte(bus::io_address::IoRegister::Nr43.address()) & 0b00001000 != 0 {
                self.lfsr = (self.lfsr & 0xFFBF) | (lfsr_bit << 6);
            }
        }
    }

    pub fn trigger<M: MemoryInterface>(&mut self, memory: &mut M) {
        self.current_volume =
            (memory.read_byte(bus::io_address::IoRegister::Nr42.address()) & 0b11110000) >> 4;
        self.period_timer =
            memory.read_byte(bus::io_address::IoRegister::Nr42.address()) & 0b00000111;
        if self.length_timer == 0 {
            self.length_timer = 64;
        }
        self.disabled = false;
    }

    pub fn sample<M: MemoryInterface>(&self, memory: &mut M) -> f32 {
        if self.disabled {
            return 0.0;
        }
        let dac_input = self.current_volume * (!self.lfsr & 1) as u8;
        let dac_output = (dac_input as f32 / 7.5) - 1.0;

        dac_output
    }

    pub fn update_envelope<M: MemoryInterface>(&mut self, memory: &mut M) {
        let period = memory.read_byte(bus::io_address::IoRegister::Nr42.address()) & 0b00000111;
        let is_upwards =
            memory.read_byte(bus::io_address::IoRegister::Nr42.address()) & 0b00001000 == 1;
        if period != 0 {
            if self.period_timer > 0 {
                self.period_timer -= 1
            }

            if self.period_timer == 0 {
                self.period_timer = period;

                if (self.current_volume < 0xF && is_upwards)
                    || (self.current_volume > 0x0 && !is_upwards)
                {
                    if is_upwards {
                        self.current_volume += 1
                    } else {
                        self.current_volume -= 1
                    }
                }
            }
        }
    }

    pub fn update_length<M: MemoryInterface>(&mut self, memory: &M) {
        self.length_timer = 64 - self.get_length(memory);
        // Whenever a length clock is provided by the frame sequencer AND bit 6 of NR44 register is set, the length timer is decremented by one.
        if memory.read_byte(bus::io_address::IoRegister::Nr41.address()) & 0b01000000 == 1 {
            self.length_timer -= 1;
            if self.length_timer == 0 {
                self.disabled = true;
            }
        }
    }

    pub fn update_sweep<M: MemoryInterface>(&mut self, memory: &mut M) {}

    fn get_length<M: MemoryInterface>(&self, memory: &M) -> u8 {
        memory.read_byte(bus::io_address::IoRegister::Nr41.address()) & 0b00111111
    }

    fn calculate_period<M: MemoryInterface>(&self, memory: &mut M) -> u8 {
        let shift = memory.read_byte(bus::io_address::IoRegister::Nr43.address()) >> 4;
        let divisor_code =
            memory.read_byte(bus::io_address::IoRegister::Nr43.address()) & 0b00000111;
        let divisor = match divisor_code {
            0 => 8,
            1 => 16,
            2 => 32,
            3 => 48,
            4 => 64,
            5 => 80,
            6 => 96,
            7 => 112,
            _ => unreachable!(),
        };
        (divisor << shift) as u8
    }
}
