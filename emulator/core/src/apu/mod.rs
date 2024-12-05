// https://nightshade256.github.io/2021/03/27/gb-sound-emulation.html

use crate::bus::{self, io_address::IoRegister, Bus};
use std::{cell::RefCell, rc::Rc};

const SAMPLE_RATE: usize = 48_000;
const CPU_FREQ: usize = 4_194_304;
const CYCLES_PER_SAMPLE: usize = CPU_FREQ / SAMPLE_RATE;
#[derive(Debug, Clone)]
pub struct Channel2 {
    frequency_timer: usize,
    wave_duty_position: u8,
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
            wave_duty_position: 0,
            length_timer: 0,
            disabled: false,
            period_timer: 0,
            current_volume: 0,
        }
    }
    pub fn tick(&mut self, bus: &mut Bus) {
        let is_triggered =
            bus.read_byte(bus::io_address::IoRegister::Nr24.address()) & 0b10000000 != 0;
        if is_triggered {
            self.trigger(bus);
        }
        self.frequency_timer -= 1;
        if self.frequency_timer == 0 {
            self.frequency_timer = (2048 - self.get_frequency(bus)) * 4;
            self.wave_duty_position = (self.wave_duty_position + 1) % 8;
        }
    }
    pub fn trigger(&mut self, bus: &mut Bus) {
        self.current_volume =
            (bus.read_byte(bus::io_address::IoRegister::Nr22.address()) & 0b11110000) >> 4;
        self.period_timer = bus.read_byte(bus::io_address::IoRegister::Nr22.address()) & 0b00000111;
        if self.length_timer == 0 {
            self.length_timer = 64;
        }
        self.disabled = false;

    self.frequency_timer = (2048 - self.get_frequency(bus)) * 4;
    }
    pub fn sample(&self, bus: &mut Bus) -> f32 {
        let wave_duty = self.get_wave_duty(bus);
        let duty_cycle_bit = (wave_duty >> self.wave_duty_position) & 1;
        let dac_input = self.current_volume * duty_cycle_bit;
        let dac_output = (dac_input as f32 / 7.5) - 1.0;
        dac_output
    }
    pub fn update_envelope(&mut self, bus: &mut Bus) {
        let period = bus.read_byte(bus::io_address::IoRegister::Nr22.address()) & 0b00000111;
        let is_upwards =
            bus.read_byte(bus::io_address::IoRegister::Nr22.address()) & 0b00001000 == 1;
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
        // Whenever a length clock is provided by the frame sequencer AND bit 6 of NR24 register is set, the length timer is decremented by one.
        if bus.read_byte(bus::io_address::IoRegister::Nr21.address()) & 0b01000000 == 1 {
            self.length_timer -= 1;
            if self.length_timer == 0 {
                self.disabled = true;
            }
        }
    }
    pub fn update_sweep(&mut self, bus: &mut Bus) {}

    fn get_length(&self, bus: &Bus) -> u8 {
        bus.read_byte(bus::io_address::IoRegister::Nr21.address()) & 0b00111111
    }
    fn get_frequency(&self, bus: &mut Bus) -> usize {
        let low_frequency = bus.read_byte(bus::io_address::IoRegister::Nr23.address()) as usize;
        let high_frequency = bus.read_byte(bus::io_address::IoRegister::Nr24.address()) as usize;
        ((high_frequency & 7) << 8) | low_frequency
    }
    fn get_wave_duty(&self, bus: &mut Bus) -> u8 {
        let duty = (bus.read_byte(bus::io_address::IoRegister::Nr21.address()) & 0b11000000) >> 6;
        match duty {
            0 => 0b00000001,
            1 => 0b00000011,
            2 => 0b00001111,
            3 => 0b11111100,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone)]

pub struct Channel3 {}
impl Channel3 {
    pub fn new() -> Self {
        Self {}
    }
    pub fn tick(&mut self, bus: &mut Bus) {}
}
pub struct APU {
    bus: Rc<RefCell<Bus>>,
    channel2: Channel2,

    frame_sequencer_timer: usize,
    frame_sequencer_step: u8,

    cycle_sample_counter: usize,
    samples: Vec<f32>,
}
impl APU {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        Self {
            bus: bus,
            channel2: Channel2::new(),
            frame_sequencer_timer: 0,
            frame_sequencer_step: 0,
            cycle_sample_counter: 0,
            samples: Vec::new(),
        }
    }
    pub fn tick(&mut self) {
        self.frame_sequencer_timer += 1;
        if self.frame_sequencer_timer >= 8192 {
            self.frame_sequencer_timer = 0;
            self.step_frame_sequencer()
        }
        self.channel2.tick(&mut self.bus.borrow_mut());

        if self.cycle_sample_counter >= CYCLES_PER_SAMPLE {
            self.cycle_sample_counter = 0;
            self.generate_sample();
        }
        self.cycle_sample_counter += 1;
    }
    pub fn get_samples(&mut self) -> Vec<f32> {
        // Limit samples to max buffer size
        println!("sample total length: {}", self.samples.len());
        let sample_size = 1600;
        let samples_to_return = self.samples.drain(..self.samples.len().min(sample_size)).collect();
        println!("sample total length: {}", self.samples.len());
        self.samples.clear();
        samples_to_return
    }
    fn generate_sample(&mut self) {
        let bus = &mut self.bus.borrow_mut();

        // Read panning and volume registers
        let nr50 = bus.read_byte(bus::io_address::IoRegister::Nr50.address());
        let nr51 = bus.read_byte(bus::io_address::IoRegister::Nr51.address());
        let nr52 = bus.read_byte(bus::io_address::IoRegister::Nr52.address());

        // Check if sound is enabled (bit 7 of NR52)
        if nr52 & 0x80 == 0 {
            self.samples.push(0.0);
            self.samples.push(0.0);
            return;
        }

        // Get channel 2 sample
        let ch2_sample = self.channel2.sample(bus);

        // Panning for left and right channels
        let mut left_amplitude = 0.0;
        let mut right_amplitude = 0.0;

        // Check panning bits for channel 2 in NR51
        if nr51 & 0x20 != 0 {
            left_amplitude += ch2_sample;
        } // Channel 2 left
        if nr51 & 0x02 != 0 {
            right_amplitude += ch2_sample;
        } // Channel 2 right

        // Apply volume from NR50 register
        let left_volume = (nr50 & 0x70) >> 4;
        let right_volume = nr50 & 0x07;

        let left_sample = left_amplitude * (left_volume as f32 );
        let right_sample = right_amplitude * (right_volume as f32);

        self.samples.push(left_sample);
        self.samples.push(right_sample);
        println!("left {}, right {}", left_sample, right_sample);
    }
    fn step_frame_sequencer(&mut self) {
        /* TODO: get from DIV register
        Step   Length Ctr  Vol Env     Sweep
        ---------------------------------------
        0      Clock       -           -
        1      -           -           -
        2      Clock       -           Clock
        3      -           -           -
        4      Clock       -           -
        5      -           -           -
        6      Clock       -           Clock
        7      -           Clock       -
        ---------------------------------------
        Rate   256 Hz      64 Hz       128 Hz
        */
        match self.frame_sequencer_step {
            0 => {
                self.channel2.update_length(&self.bus.borrow());
            }
            1 => {}
            2 => {
                self.channel2.update_length(&self.bus.borrow());
                self.channel2.update_sweep(&mut self.bus.borrow_mut());
            }
            3 => {}
            4 => {}
            5 => {}
            6 => {
                self.channel2.update_length(&self.bus.borrow());
                self.channel2.update_sweep(&mut self.bus.borrow_mut());
            }
            7 => {
                self.channel2.update_envelope(&mut self.bus.borrow_mut());
            }
            _ => unreachable!(),
        }
        self.frame_sequencer_step = (self.frame_sequencer_step + 1) % 8;
    }
}
