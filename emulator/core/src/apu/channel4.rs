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

    pub fn tick(&mut self, nr42: u8, nr43: u8, nr44: u8) {
        let is_triggered = nr44 & 0b10000000 != 0;
        if is_triggered {
            self.trigger(nr42);
        }
        self.period_timer -= 1;
        if self.period_timer == 0 {
            self.period_timer = self.calculate_period(nr43);
            let lfsr_bit = (self.lfsr ^ (self.lfsr >> 1)) & 1;
            self.lfsr = (self.lfsr >> 1) | (lfsr_bit << 14);
            if nr43 & 0b00001000 != 0 {
                self.lfsr = (self.lfsr & 0xFFBF) | (lfsr_bit << 6);
            }
        }
    }

    pub fn trigger(&mut self, nr42: u8) {
        self.current_volume = (nr42 & 0b11110000) >> 4;
        self.period_timer = nr42 & 0b00000111;
        if self.length_timer == 0 {
            self.length_timer = 64;
        }
        self.disabled = false;
    }

    pub fn sample(&self) -> f32 {
        if self.disabled {
            return 0.0;
        }
        let dac_input = self.current_volume * (!self.lfsr & 1) as u8;
        let dac_output = (dac_input as f32 / 7.5) - 1.0;

        dac_output
    }

    pub fn update_envelope(&mut self, nr42: u8) {
        let period = nr42 & 0b00000111;
        let is_upwards = nr42 & 0b00001000 == 1;
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

    pub fn update_length(&mut self, nr41: u8) {
        self.length_timer = 64 - self.get_length(nr41);
        // Whenever a length clock is provided by the frame sequencer AND bit 6 of NR44 register is set, the length timer is decremented by one.
        if nr41 & 0b01000000 == 1 {
            self.length_timer -= 1;
            if self.length_timer == 0 {
                self.disabled = true;
            }
        }
    }

    pub fn update_sweep(&mut self) {}

    fn get_length(&self, nr41: u8) -> u8 {
        nr41 & 0b00111111
    }

    fn calculate_period(&self, nr43: u8) -> u8 {
        let shift = nr43 >> 4;
        let divisor_code = nr43 & 0b00000111;
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
