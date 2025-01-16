// https://nightshade256.github.io/2021/03/27/gb-sound-emulation.html

use channel1::Channel1;
use channel2::Channel2;
use channel3::Channel3;
use channel4::Channel4;

use crate::bus::{self, io_address::IoRegister, Bus, MemoryInterface};
use std::{cell::RefCell, rc::Rc};

const SAMPLE_RATE: usize = 48_000;
const CPU_FREQ: usize = 4_194_304;
const CYCLES_PER_SAMPLE: usize = CPU_FREQ / SAMPLE_RATE;

mod channel1;
mod channel2;
mod channel3;
mod channel4;

#[derive(Debug, Clone)]
pub struct APU {
    channel1: Channel1,
    channel2: Channel2,
    channel3: Channel3,
    channel4: Channel4,

    frame_sequencer_timer: usize,
    frame_sequencer_step: u8,

    cycle_sample_counter: usize,
    samples: Vec<f32>,
    pub enabled: bool,

    pub current_ch1_output: f32,
    pub current_ch2_output: f32,
    pub current_ch3_output: f32,
    pub current_ch4_output: f32,
    pub ch1_enabled: bool,
    pub ch2_enabled: bool,
    pub ch3_enabled: bool,
    pub ch4_enabled: bool,
}
impl APU {
    pub fn new() -> Self {
        Self {
            channel1: Channel1::new(),
            channel2: Channel2::new(),
            channel3: Channel3::new(),
            channel4: Channel4::new(),
            frame_sequencer_timer: 0,
            frame_sequencer_step: 0,
            cycle_sample_counter: 0,
            samples: Vec::new(),
            enabled: true,
            ch1_enabled: true,
            ch2_enabled: true,
            ch3_enabled: true,
            ch4_enabled: true,
            current_ch1_output: 0.0,
            current_ch2_output: 0.0,
            current_ch3_output: 0.0,
            current_ch4_output: 0.0,
        }
    }
    pub fn toggle_audio(&mut self) {
        self.enabled = !self.enabled;
    }
    pub fn toggle_channel(&mut self, channel: u8) {
        match channel {
            1 => self.ch1_enabled = !self.ch1_enabled,
            2 => self.ch2_enabled = !self.ch2_enabled,
            3 => self.ch3_enabled = !self.ch3_enabled,
            4 => self.ch4_enabled = !self.ch4_enabled,
            _ => {}
        }
    }
    pub fn tick<M: MemoryInterface>(&mut self, memory: &mut M) {
        if !self.enabled {
            return;
        }
        self.frame_sequencer_timer += 1;
        if self.frame_sequencer_timer >= 8192 {
            self.frame_sequencer_timer = 0;
            self.step_frame_sequencer(memory)
        }
        self.channel1.tick(memory);
        self.channel2.tick(memory);
        self.channel3.tick(memory);
        self.channel4.tick(memory);

        if self.cycle_sample_counter >= CYCLES_PER_SAMPLE {
            self.cycle_sample_counter = 0;
            self.generate_sample(memory);
        }
        self.cycle_sample_counter += 1;
    }
    pub fn get_samples(&mut self) -> Vec<f32> {
        const SAMPLE_BUFFER_SIZE: usize = 1600;

        // Preallocate to avoid repeated allocations
        let mut samples = Vec::with_capacity(SAMPLE_BUFFER_SIZE);

        // Drain samples more efficiently
        samples.extend(self.samples.drain(..).take(SAMPLE_BUFFER_SIZE));

        // Pad with silence if needed
        if samples.len() < SAMPLE_BUFFER_SIZE {
            samples.resize(SAMPLE_BUFFER_SIZE, 0.0);
        }

        samples
    }
    fn generate_sample<M: MemoryInterface>(&mut self, memory: &mut M) {
        // Read panning and volume registers
        let nr50 = memory.read_byte(bus::io_address::IoRegister::Nr50.address());
        let nr51 = memory.read_byte(bus::io_address::IoRegister::Nr51.address());
        let nr52 = memory.read_byte(bus::io_address::IoRegister::Nr52.address());

        // Master volume
        if nr52 & 0x80 == 0 {
            self.samples.push(0.0);
            self.samples.push(0.0);
            return;
        }

        let ch1_sample = if self.ch1_enabled {
            self.channel1.sample(memory)
        } else {
            0.0
        };
        let ch2_sample = if self.ch2_enabled {
            self.channel2.sample(memory)
        } else {
            0.0
        };
        let ch3_sample = if self.ch3_enabled {
            self.channel3.sample(memory)
        } else {
            0.0
        };
        let ch4_sample = if self.ch4_enabled {
            self.channel4.sample(memory)
        } else {
            0.0
        };

        // Panning for left and right channels
        let mut left_amplitude = 0.0;
        let mut right_amplitude = 0.0;
        if nr51 & 0b00010000 != 0 {
            left_amplitude += ch1_sample;
        }
        if nr51 & 0b00000001 != 0 {
            right_amplitude += ch1_sample;
        }
        if nr51 & 0x20 != 0 {
            left_amplitude += ch2_sample;
        }
        if nr51 & 0x02 != 0 {
            right_amplitude += ch2_sample;
        }
        if nr51 & 0b01000000 != 0 {
            left_amplitude += ch3_sample;
        }
        if nr51 & 0b00000100 != 0 {
            right_amplitude += ch3_sample;
        }
        if nr51 & 0b10000000 != 0 {
            left_amplitude += ch4_sample;
        }
        if nr51 & 0b00001000 != 0 {
            right_amplitude += ch4_sample;
        }
        right_amplitude = right_amplitude / 4.0;
        left_amplitude = left_amplitude / 4.0;

        // Apply volume from NR50 register
        let left_volume = (nr50 & 0x70) >> 4;
        let right_volume = nr50 & 0x07;

        let left_sample = left_amplitude * left_volume as f32;
        let right_sample = right_amplitude * right_volume as f32;

        self.samples.push(left_sample);
        self.samples.push(right_sample);

        // debug
        self.current_ch1_output = ch1_sample;
        self.current_ch2_output = ch2_sample;
        self.current_ch3_output = ch3_sample;
        self.current_ch4_output = ch4_sample;
    }
    fn update_lengths<M: MemoryInterface>(&mut self, memory: &M) {
        self.channel1.update_length(memory);
        self.channel2.update_length(memory);
        self.channel3.update_length(memory);
        self.channel4.update_length(memory);
    }

    fn update_sweeps<M: MemoryInterface>(&mut self, memory: &mut M) {
        self.channel1.update_sweep(memory);
    }

    fn update_envelopes<M: MemoryInterface>(&mut self, memory: &mut M) {
        self.channel1.update_envelope(memory);
        self.channel2.update_envelope(memory);
        self.channel4.update_envelope(memory);
    }

    fn step_frame_sequencer<M: MemoryInterface>(&mut self, memory: &mut M) {
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
                self.update_lengths(memory);
            }
            1 => {}
            2 => {
                self.update_lengths(memory);
                self.update_sweeps(memory);
            }
            3 => {}
            4 => {
                self.update_lengths(memory);
            }
            5 => {}
            6 => {
                self.update_lengths(memory);
                self.update_sweeps(memory);
            }
            7 => {
                self.update_envelopes(memory);
            }
            _ => unreachable!(),
        }
        self.frame_sequencer_step = (self.frame_sequencer_step + 1) % 8;
    }
}
