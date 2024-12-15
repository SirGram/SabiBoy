use crate::bus::{self, io_address::IoRegister, Bus};

#[derive(Debug, Clone)]
pub struct Channel4 {
    frequency_timer: usize,
    length_timer: u8,
    disabled: bool,
    period_timer: u8,
    current_volume: u8,

    lfsr: u16,
}
impl Channel4 {
    /*
          FF1F ---- ---- Not used
    NR41 FF20 --LL LLLL Length load (64-L)
    NR42 FF21 VVVV APPP Starting volume, Envelope add mode, period
    NR43 FF22 SSSS WDDD Clock shift, Width mode of LFSR, Divisor code
    NR44 FF23 TL-- ---- Trigger, Length enable
        */
    pub fn new() -> Self {
        Self {
            frequency_timer: 0,
            length_timer: 0,
            disabled: false,
            period_timer: 0,
            current_volume: 0,
            lfsr: 0,
        }
    }
    pub fn tick(&mut self, bus: &mut Bus) {
        let is_triggered =
            bus.read_byte(bus::io_address::IoRegister::Nr44.address()) & 0b10000000 != 0;
        if is_triggered {
            self.trigger(bus);
        }

        if self.frequency_timer > 0 {
            self.frequency_timer -= 1
        };
        if self.frequency_timer == 0 {
            self.frequency_timer = self.calculate_frequency_timer(bus);

            let nr43 = bus.read_byte(IoRegister::Nr43.address());
            let width_mode = nr43 & 0b1000 != 0;

            let xor_result = (self.lfsr & 0b01) ^ ((self.lfsr & 0b10) >> 1);
            if width_mode {
                self.lfsr = (self.lfsr >> 1) & 0x7F;
                self.lfsr |= (xor_result << 6);
            } else {
                self.lfsr = (self.lfsr >> 1) | (xor_result << 14);
            }
        }
    }
    fn calculate_frequency_timer(&self, bus: &mut Bus) -> usize {
        let nr43 = bus.read_byte(IoRegister::Nr43.address());
        let divisor_code = nr43 & 0b111;
        let divisor = match divisor_code {
            0 => 8,
            1 => 16,
            2 => 32,
            3 => 48,
            4 => 64,
            5 => 80,
            6 => 96,
            7 => 112,
            _ => 8,
        };
        let shift_amount = (nr43 & 0b1110000) >> 4;
        (divisor << shift_amount) as usize
    }

    pub fn trigger(&mut self, bus: &Bus) {
        // Set LFSR to all 1s
        self.lfsr = 0x7FFF;

        let nr42 = bus.read_byte(IoRegister::Nr42.address());
        self.current_volume = (nr42 & 0xF0) >> 4;
        self.period_timer = nr42 & 0x07;
        if bus.read_byte(IoRegister::Nr44.address()) & 0x40 != 0 {
            self.length_timer = 64 - (bus.read_byte(IoRegister::Nr41.address()) & 0x3F);
        }
        self.disabled = false;
    }
    pub fn sample(&self) -> f32 {
        if self.disabled {
            return 0.0;
        }
        let amplitude = if self.lfsr & 1 == 0 { 1.0 } else { 0.0 };
        let dac_input = (self.current_volume as f32) * amplitude;
        (dac_input / 7.5) - 1.0
    }
    pub fn update_envelope(&mut self, bus: &mut Bus) {
        let period = bus.read_byte(bus::io_address::IoRegister::Nr42.address()) & 0b00000111;
        let is_upwards =
            bus.read_byte(bus::io_address::IoRegister::Nr42.address()) & 0b00001000 == 1;
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
    pub fn update_length(&mut self, bus: &Bus) {
        self.length_timer = 64 - self.get_length(bus);
        if bus.read_byte(bus::io_address::IoRegister::Nr44.address()) & 0b01000000 == 1 {
            self.length_timer -= 1;
            if self.length_timer == 0 {
                self.disabled = true;
            }
        }
    }

    fn get_length(&self, bus: &Bus) -> u8 {
        bus.read_byte(bus::io_address::IoRegister::Nr41.address()) & 0b00111111
    }
}
