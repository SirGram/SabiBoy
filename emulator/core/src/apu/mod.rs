// https://nightshade256.github.io/2021/03/27/gb-sound-emulation.html

use channel1::Channel1;
use channel2::Channel2;
use channel3::Channel3;
use channel4::Channel4;

use crate::bus::{self, io_address::IoRegister, Bus};
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
    bus: Rc<RefCell<Bus>>,
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
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        Self {
            bus: bus,
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
    pub fn tick(&mut self) {
        if !self.enabled {
            return;
        }
        self.frame_sequencer_timer += 1;
        if self.frame_sequencer_timer >= 8192 {
            self.frame_sequencer_timer = 0;
            self.step_frame_sequencer()
        }
        self.channel1.tick(&mut self.bus.borrow_mut());
        self.channel2.tick(&mut self.bus.borrow_mut());
        self.channel3.tick(&mut self.bus.borrow_mut());
        self.channel4.tick(&mut self.bus.borrow_mut());

        if self.cycle_sample_counter >= CYCLES_PER_SAMPLE {
            self.cycle_sample_counter = 0;
            self.generate_sample();
        }
        self.cycle_sample_counter += 1;
    }
    pub fn get_samples(&mut self) -> Vec<f32> {
        const SAMPLE_BUFFER_SIZE: usize = 1600;
        
        // Preallocate to avoid repeated allocations
        let mut samples = Vec::with_capacity(SAMPLE_BUFFER_SIZE);
        
        // Drain samples more efficiently
        samples.extend(
            self.samples
                .drain(..)
                .take(SAMPLE_BUFFER_SIZE)
        );
        
        // Pad with silence if needed
        if samples.len() < SAMPLE_BUFFER_SIZE {
            samples.resize(SAMPLE_BUFFER_SIZE, 0.0);
        }
        
        samples
    }
    fn generate_sample(&mut self) {
        let bus = &mut self.bus.borrow_mut();

        // Read panning and volume registers
        let nr50 = bus.read_byte(bus::io_address::IoRegister::Nr50.address());
        let nr51 = bus.read_byte(bus::io_address::IoRegister::Nr51.address());
        let nr52 = bus.read_byte(bus::io_address::IoRegister::Nr52.address());

        // Master volume
        if nr52 & 0x80 == 0 {
            self.samples.push(0.0);
            self.samples.push(0.0);
            return;
        }

        let ch1_sample = if self.ch1_enabled { self.channel1.sample(bus) } else { 0.0 };
        let ch2_sample = if self.ch2_enabled { self.channel2.sample(bus) } else { 0.0 };
        let ch3_sample = if self.ch3_enabled { self.channel3.sample(bus) } else { 0.0 };
        let ch4_sample = if self.ch4_enabled { self.channel4.sample() } else { 0.0 };

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
        self.current_ch1_output = ch1_sample ;
        self.current_ch2_output = ch2_sample ;
        self.current_ch3_output = ch3_sample ;
        self.current_ch4_output = ch4_sample ; 
    }
    fn update_lengths(&mut self) {
        self.channel1.update_length(&self.bus.borrow());
        self.channel2.update_length(&self.bus.borrow());
        self.channel3.update_length(&self.bus.borrow());
        self.channel4.update_length(&self.bus.borrow());
    }
    fn update_sweeps(&mut self) {
        self.channel2.update_sweep(&mut self.bus.borrow_mut());
    }
    fn update_envelopes(&mut self) {
        self.channel1.update_envelope(&mut self.bus.borrow_mut());
        self.channel2.update_envelope(&mut self.bus.borrow_mut());
        self.channel4.update_envelope(&mut self.bus.borrow_mut());
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
                self.update_lengths();
            }
            1 => {}
            2 => {
                self.update_lengths();
                self.update_sweeps();
            }
            3 => {}
            4 => {
                self.update_lengths();
            }
            5 => {}
            6 => {
                self.update_lengths();
                self.update_sweeps();
            }
            7 => {
                self.update_envelopes();
            }
            _ => unreachable!(),
        }
        self.frame_sequencer_step = (self.frame_sequencer_step + 1) % 8;
    }
}
