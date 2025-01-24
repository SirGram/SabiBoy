// https://nightshade256.github.io/2021/03/27/gb-sound-emulation.html

use channel1::Channel1;
use channel2::Channel2;
use channel3::Channel3;
use channel4::Channel4;

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
    pub rgs: APURegisters,
}
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct APURegisters {
    pub nr10: u8,
    pub nr11: u8,
    pub nr12: u8,
    pub nr13: u8,
    pub nr14: u8,
    pub nr20: u8,
    pub nr21: u8,
    pub nr22: u8,
    pub nr23: u8,
    pub nr24: u8,
    pub nr30: u8,
    pub nr31: u8,
    pub nr32: u8,
    pub nr33: u8,
    pub nr34: u8,
    pub nr40: u8,
    pub nr41: u8,
    pub nr42: u8,
    pub nr43: u8,
    pub nr44: u8,
    pub nr50: u8,
    pub nr51: u8,
    pub nr52: u8,
    pub wave_ram: [u8; 16],
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
            rgs: APURegisters {
                nr10: 0,
                nr11: 0,
                nr12: 0,
                nr13: 0,
                nr14: 0,
                nr20: 0,
                nr21: 0,
                nr22: 0,
                nr23: 0,
                nr24: 0,
                nr30: 0,
                nr31: 0,
                nr32: 0,
                nr33: 0,
                nr34: 0,
                nr40: 0,
                nr41: 0,
                nr42: 0,
                nr43: 0,
                nr44: 0,
                nr50: 0,
                nr51: 0,
                nr52: 0,
                wave_ram: [0; 16],
            },
        }
    }
    pub fn read_register(&self, address: u16) -> u8 {
        match address {
            0xFF10 => self.rgs.nr10,
            0xFF11 => self.rgs.nr11,
            0xFF12 => self.rgs.nr12,
            0xFF13 => self.rgs.nr13,
            0xFF14 => self.rgs.nr14,
            0xFF16 => self.rgs.nr21,
            0xFF17 => self.rgs.nr22,
            0xFF18 => self.rgs.nr23,
            0xFF19 => self.rgs.nr24,
            0xFF1A => self.rgs.nr30,
            0xFF1B => self.rgs.nr31,
            0xFF1C => self.rgs.nr32,
            0xFF1D => self.rgs.nr33,
            0xFF1E => self.rgs.nr34,
            0xFF20 => self.rgs.nr41,
            0xFF21 => self.rgs.nr42,
            0xFF22 => self.rgs.nr43,
            0xFF23 => self.rgs.nr44,
            0xFF24 => self.rgs.nr50,
            0xFF25 => self.rgs.nr51,
            0xFF26 => self.rgs.nr52,
            _ => 0xFF,
        }
    }

    pub fn write_register(&mut self, address: u16, value: u8) {
        match address {
            0xFF10 => self.rgs.nr10 = value,
            0xFF11 => self.rgs.nr11 = value,
            0xFF12 => self.rgs.nr12 = value,
            0xFF13 => self.rgs.nr13 = value,
            0xFF14 => self.rgs.nr14 = value,
            0xFF16 => self.rgs.nr21 = value,
            0xFF17 => self.rgs.nr22 = value,
            0xFF18 => self.rgs.nr23 = value,
            0xFF19 => self.rgs.nr24 = value,
            0xFF1A => self.rgs.nr30 = value,
            0xFF1B => self.rgs.nr31 = value,
            0xFF1C => self.rgs.nr32 = value,
            0xFF1D => self.rgs.nr33 = value,
            0xFF1E => self.rgs.nr34 = value,
            0xFF20 => self.rgs.nr41 = value,
            0xFF21 => self.rgs.nr42 = value,
            0xFF22 => self.rgs.nr43 = value,
            0xFF23 => self.rgs.nr44 = value,
            0xFF24 => self.rgs.nr50 = value,
            0xFF25 => self.rgs.nr51 = value,
            0xFF26 => self.rgs.nr52 = value,
            _ => {}
        }
    }

    pub fn read_wave_ram(&self, index: u16) -> u8 {
        self.rgs.wave_ram[index as usize]
    }

    pub fn write_wave_ram(&mut self, index: u16, value: u8) {
        self.rgs.wave_ram[index as usize] = value;
    }

    pub fn get_wave_ram(&self) -> [u8; 16] {
        self.rgs.wave_ram
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
        self.channel1
            .tick(self.rgs.nr10, self.rgs.nr12, self.rgs.nr13, self.rgs.nr14);
        self.channel2
            .tick(self.rgs.nr22, self.rgs.nr23, self.rgs.nr24);
        self.channel3
            .tick(self.rgs.nr32, self.rgs.nr33, self.rgs.nr34);
        self.channel4
            .tick(self.rgs.nr42, self.rgs.nr43, self.rgs.nr44);

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
        samples.extend(self.samples.drain(..).take(SAMPLE_BUFFER_SIZE));

        // Pad with silence if needed
        if samples.len() < SAMPLE_BUFFER_SIZE {
            samples.resize(SAMPLE_BUFFER_SIZE, 0.0);
        }

        samples
    }
    fn generate_sample(&mut self) {
        // Read panning and volume registers

        // Master volume
        if self.rgs.nr52 & 0x80 == 0 {
            self.samples.push(0.0);
            self.samples.push(0.0);
            return;
        }

        let ch1_sample = if self.ch1_enabled {
            self.channel1.sample(self.rgs.nr11)
        } else {
            0.0
        };
        let ch2_sample = if self.ch2_enabled {
            self.channel2.sample(self.rgs.nr21)
        } else {
            0.0
        };
        let ch3_sample = if self.ch3_enabled {
            self.channel3.sample(self.rgs.nr50, &self.rgs.wave_ram)
        } else {
            0.0
        };
        let ch4_sample = if self.ch4_enabled {
            self.channel4.sample()
        } else {
            0.0
        };

        // Panning for left and right channels
        let mut left_amplitude = 0.0;
        let mut right_amplitude = 0.0;
        let nr51 = self.rgs.nr51;
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
        let left_volume = (self.rgs.nr50 & 0x70) >> 4;
        let right_volume = self.rgs.nr50 & 0x07;

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
    fn update_lengths(&mut self) {
        self.channel1.update_length(self.rgs.nr11);
        self.channel2.update_length(self.rgs.nr21);
        self.channel3.update_length(self.rgs.nr31);
        self.channel4.update_length(self.rgs.nr41);
    }

    fn update_sweeps(&mut self) {
        self.channel1.update_sweep(self.rgs.nr10);
    }

    fn update_envelopes(&mut self) {
        self.channel1.update_envelope(self.rgs.nr12);
        self.channel2.update_envelope(self.rgs.nr22);
        self.channel4.update_envelope(self.rgs.nr42);
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
