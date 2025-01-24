use crate::bus::{self, io_address::IoRegister, Bus, MemoryInterface};
#[derive(Debug, Clone)]
pub struct Channel1 {
    frequency_timer: usize,
    wave_position: u8,
    length_timer: u8,
    disabled: bool,
    period_timer: u8,
    current_volume: u8,

    sweep_timer: u8,
    shadow_frequency: usize,
    sweep_enabled: bool,
}
impl Channel1 {
    /*
    NR10 FF10 -PPP NSSS Sweep period, negate, shift
    NR11 FF11 DDLL LLLL Duty, Length load (64-L)
    NR12 FF12 VVVV APPP Starting volume, Envelope add mode, period
    NR13 FF13 FFFF FFFF Frequency LSB
    NR14 FF14 TL-- -FFF Trigger, Length enable, Frequency MSB
    */
    pub fn new() -> Self {
        Self {
            frequency_timer: 2048 * 4,
            wave_position: 0,
            length_timer: 0,
            disabled: false,
            period_timer: 0,
            current_volume: 0,
            sweep_timer: 0,
            shadow_frequency: 0,
            sweep_enabled: false,
        }
    }
    pub fn tick(&mut self, nr10: u8, nr12: u8, nr13: u8, nr14: u8) {
        let is_triggered = nr14 & 0b10000000 != 0;
        if is_triggered {
            self.trigger(nr10, nr12, nr13, nr14);
        }
        self.frequency_timer -= 1;
        if self.frequency_timer == 0 {
            self.frequency_timer = (2048 - self.calculate_frequency(nr13, nr14)) * 4;
            self.wave_position = (self.wave_position + 1) % 8;
        }
    }
    pub fn trigger(&mut self, nr10: u8, nr12: u8, nr13: u8, nr14: u8) {
        self.current_volume = (nr12 & 0b11110000) >> 4;
        self.period_timer = nr12 & 0b00000111;
        if self.length_timer == 0 {
            self.length_timer = 64;
        }
        self.disabled = false;

        // sweep
        let sweep_period = nr10 & 0b01110000;
        let sweep_direction = nr10 & 0b00001000;
        let sweep_shift = nr10 & 0b00000111;
        self.shadow_frequency = self.calculate_frequency(nr13, nr14);
        self.sweep_timer = if sweep_period == 0 { 8 } else { sweep_period };
        self.sweep_enabled = sweep_period != 0 || sweep_shift != 0;
        if sweep_shift != 0 {
            let new_frequency = self.calculate_sweep_frequency(sweep_direction == 0, sweep_shift);
            if new_frequency > 2047 {
                self.sweep_enabled = false;
            }
        }
    }
    pub fn sample(&self, nr11: u8) -> f32 {
        if self.disabled {
            return 0.0;
        }
        let wave_duty = self.get_wave_duty(nr11);
        let duty_cycle_bit = (wave_duty >> self.wave_position) & 1;
        let dac_input = self.current_volume * duty_cycle_bit;
        let dac_output = (dac_input as f32 / 7.5) - 1.0;

        dac_output
    }
    pub fn update_envelope(&mut self, nr12: u8) {
        let period = nr12 & 0b00000111;
        let is_upwards = nr12 & 0b00001000 == 1;
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
    pub fn update_length(&mut self, nr11: u8) {
        self.length_timer = 64 - self.get_length(nr11);
        // Whenever a length clock is provided by the frame sequencer AND bit 6 of NR24 register is set, the length timer is decremented by one.
        if nr11 & 0b01000000 == 1 {
            self.length_timer -= 1;
            if self.length_timer == 0 {
                self.disabled = true;
            }
        }
    }
    pub fn update_sweep(&mut self, nr10: u8) {
        let sweep_period = nr10 & 0b01110000;
        let sweep_direction = nr10 & 0b00001000;
        let sweep_shift = nr10 & 0b00000111;

        if self.sweep_timer > 0 {
            self.sweep_timer -= 1;
        }
        if self.sweep_timer != 0 {
            return;
        }
        self.sweep_timer = if sweep_period == 0 { 8 } else { sweep_period };
        if sweep_period > 0 && self.sweep_enabled {
            let new_frequency = self.calculate_sweep_frequency(sweep_direction == 0, sweep_shift);
            if new_frequency <= 2047 && sweep_shift > 0 {
                self.shadow_frequency = new_frequency;
                self.frequency_timer = new_frequency;
                let double_check_frequency =
                    self.calculate_sweep_frequency(sweep_direction == 0, sweep_shift);
                if double_check_frequency > 2047 {
                    self.sweep_enabled = false;
                }
            }
        }
    }
    fn calculate_sweep_frequency(&mut self, is_decrementing: bool, sweep_shift: u8) -> usize {
        let mut new_frequency = self.shadow_frequency >> sweep_shift;
        new_frequency = if is_decrementing {
            self.shadow_frequency - new_frequency
        } else {
            self.shadow_frequency + new_frequency
        };
        if new_frequency > 2047 {
            self.sweep_enabled = false;
        }
        new_frequency
    }

    fn get_length(&self, nr11: u8) -> u8 {
        nr11 & 0b00111111
    }
    fn calculate_frequency(&self, nr13: u8, nr14: u8) -> usize {
        let low_frequency = nr13 as usize;
        let high_frequency = nr14 as usize;
        ((high_frequency & 7) << 8) | low_frequency
    }
    fn get_wave_duty(&self, nr11: u8) -> u8 {
        let duty = (nr11 & 0b11000000) >> 6;
        match duty {
            0 => 0b00000001,
            1 => 0b00000011,
            2 => 0b00001111,
            3 => 0b11111100,
            _ => unreachable!(),
        }
    }
}
