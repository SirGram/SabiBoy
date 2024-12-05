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
    NR10 FF10 -PPP NSSS Sweep period, negate, shift
    NR11 FF11 DDLL LLLL Duty, Length load (64-L)
    NR12 FF12 VVVV APPP Starting volume, Envelope add mode, period
    NR13 FF13 FFFF FFFF Frequency LSB
    NR14 FF14 TL-- -FFF Trigger, Length enable, Frequency MSB
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
            self.trigger();
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
                // 7-bit mode: XOR result goes to bit 6 after shifting
                self.lfsr = (self.lfsr >> 1) & 0x7F;
                self.lfsr |= xor_result << 6;
            } else {
                // 15-bit mode: XOR result goes to bit 14
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

    pub fn trigger(&mut self) {
        self.lfsr = 0x7FFF;
    }
    pub fn sample(&self) -> f32 {
        let amplitude = if self.lfsr & 1 == 0 { 1 } else { 0 };
        let dac_input = self.current_volume * amplitude;
        let dac_output = (dac_input as f32 / 7.5) - 1.0;
        dac_output
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
