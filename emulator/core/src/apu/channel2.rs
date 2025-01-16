use crate::bus::{self, io_address::IoRegister, Bus, MemoryInterface};

#[derive(Debug, Clone)]
pub struct Channel2 {
    frequency_timer: usize,
    wave_position: u8,
    length_timer: u8,
    disabled: bool,
    period_timer: u8,
    current_volume: u8,
}

impl Channel2 {
    /*
    FF15 ---- ---- Not used
    NR21 FF16 DDLL LLLL Duty, Length load (64-L)
    NR22 FF17 VVVV APPP Starting volume, Envelope add mode, period
    NR23 FF18 FFFF FFFF Frequency LSB
    NR24 FF19 TL-- -FFF Trigger, Length enable, Frequency MSB
    */
    pub fn new() -> Self {
        Self {
            frequency_timer: 2048 * 4,
            wave_position: 0,
            length_timer: 0,
            disabled: false,
            period_timer: 0,
            current_volume: 0,
        }
    }

    pub fn tick<M: MemoryInterface>(&mut self, memory: &mut M) {
        let is_triggered =
            memory.read_byte(bus::io_address::IoRegister::Nr24.address()) & 0b10000000 != 0;
        if (is_triggered) {
            self.trigger(memory);
        }
        self.frequency_timer -= 1;
        if self.frequency_timer == 0 {
            self.frequency_timer = (2048 - self.calculate_frequency(memory)) * 4;
            self.wave_position = (self.wave_position + 1) % 8;
        }
    }

    pub fn trigger<M: MemoryInterface>(&mut self, memory: &mut M) {
        self.current_volume =
            (memory.read_byte(bus::io_address::IoRegister::Nr22.address()) & 0b11110000) >> 4;
        self.period_timer =
            memory.read_byte(bus::io_address::IoRegister::Nr22.address()) & 0b00000111;
        if self.length_timer == 0 {
            self.length_timer = 64;
        }
        self.disabled = false;
    }

    pub fn sample<M: MemoryInterface>(&self, memory: &mut M) -> f32 {
        if self.disabled {
            return 0.0;
        }
        let wave_duty = self.get_wave_duty(memory);
        let duty_cycle_bit = (wave_duty >> self.wave_position) & 1;
        let dac_input = self.current_volume * duty_cycle_bit;
        let dac_output = (dac_input as f32 / 7.5) - 1.0;

        dac_output
    }

    pub fn update_envelope<M: MemoryInterface>(&mut self, memory: &mut M) {
        let period = memory.read_byte(bus::io_address::IoRegister::Nr22.address()) & 0b00000111;
        let is_upwards =
            memory.read_byte(bus::io_address::IoRegister::Nr22.address()) & 0b00001000 == 1;
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
        // Whenever a length clock is provided by the frame sequencer AND bit 6 of NR24 register is set, the length timer is decremented by one.
        if memory.read_byte(bus::io_address::IoRegister::Nr21.address()) & 0b01000000 == 1 {
            self.length_timer -= 1;
            if self.length_timer == 0 {
                self.disabled = true;
            }
        }
    }

    pub fn update_sweep<M: MemoryInterface>(&mut self, memory: &mut M) {}

    fn get_length<M: MemoryInterface>(&self, memory: &M) -> u8 {
        memory.read_byte(bus::io_address::IoRegister::Nr21.address()) & 0b00111111
    }

    fn calculate_frequency<M: MemoryInterface>(&self, memory: &mut M) -> usize {
        let low_frequency = memory.read_byte(bus::io_address::IoRegister::Nr23.address()) as usize;
        let high_frequency = memory.read_byte(bus::io_address::IoRegister::Nr24.address()) as usize;
        ((high_frequency & 7) << 8) | low_frequency
    }

    fn get_wave_duty<M: MemoryInterface>(&self, memory: &mut M) -> u8 {
        let duty =
            (memory.read_byte(bus::io_address::IoRegister::Nr21.address()) & 0b11000000) >> 6;
        match duty {
            0 => 0b00000001,
            1 => 0b00000011,
            2 => 0b00001111,
            3 => 0b11111100,
            _ => unreachable!(),
        }
    }
}
