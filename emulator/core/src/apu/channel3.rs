#[derive(Debug, Clone)]
pub struct Channel3 {
    frequency_timer: usize,
    wave_position: u8,
    length_timer: u16,
    disabled: bool,
    period_timer: u8,
    current_volume: u8,
}
impl Channel3 {
    /*
        NR30 FF1A E--- ---- DAC power
    NR31 FF1B LLLL LLLL Length load (256-L)
    NR32 FF1C -VV- ---- Volume code (00=0%, 01=100%, 10=50%, 11=25%)
    NR33 FF1D FFFF FFFF Frequency LSB
    NR34 FF1E TL-- -FFF Trigger, Length enable, Frequency MSB
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
    pub fn tick(&mut self, nr32: u8, nr33: u8, nr34: u8) {
        let is_triggered = nr34 & 0b10000000 != 0;
        if is_triggered {
            self.trigger(nr32);
        }
        self.frequency_timer -= 1;
        if self.frequency_timer == 0 {
            self.frequency_timer = (2048 - self.calculate_frequency(nr33, nr34)) * 4;
            self.wave_position = (self.wave_position + 1) % 8;
        }
    }
    pub fn trigger(&mut self, nr32: u8) {
        self.current_volume = (nr32 & 0b11110000) >> 4;
        self.period_timer = nr32 & 0b00000111;
        if self.length_timer == 0 {
            self.length_timer = 64;
        }
        self.disabled = false;
    }
    fn get_sample(&self, wave_ram: &[u8; 16]) -> u8 {
        // 1 byte -> 2 samples
        let index = self.wave_position as usize / 2;
        let byte = wave_ram[index];
        if self.wave_position % 2 == 0 {
            byte & 0xF0 >> 4
        } else {
            byte & 0xF
        }
    }
    pub fn sample(&self, nr50: u8, wave_ram: &[u8; 16]) -> f32 {
        if self.disabled {
            return 0.0;
        }
        let raw_samle = self.get_sample(wave_ram);
        let volume_shift = self.get_volume_shift(nr50);
        let dac_input = raw_samle >> volume_shift;

        let dac_output = (dac_input as f32 / 7.5) - 1.0;

        dac_output
    }
    pub fn update_length(&mut self, nr31: u8) {
        self.length_timer = 256 - self.get_length(nr31) as u16;
        if nr31 & 0b01000000 == 1 {
            self.length_timer -= 1;
            if self.length_timer == 0 {
                self.disabled = true;
            }
        }
    }

    fn get_length(&self, nr31: u8) -> u8 {
        nr31 & 0b11111111
    }
    fn calculate_frequency(&self, nr33: u8, nr34: u8) -> usize {
        let low_frequency = nr33 as usize;
        let high_frequency = nr34 as usize;
        ((high_frequency & 7) << 8) | low_frequency
    }
    fn get_volume_shift(&self, nr50: u8) -> u8 {
        let volume_bits = (nr50 & 0b01100000) >> 5;
        match volume_bits {
            0 => 4,
            1 => 0,
            2 => 1,
            3 => 2,
            _ => unreachable!(),
        }
    }
}
