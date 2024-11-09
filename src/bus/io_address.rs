pub enum IoRegister {
    // Joypad and Serial
    Joyp = 0xFF00,
    Sb = 0xFF01,
    Sc = 0xFF02,

    // Timer and Divider
    Div = 0xFF04,
    Tima = 0xFF05,
    Tma = 0xFF06,
    Tac = 0xFF07,

    // Interrupt Flags
    If = 0xFF0F,
    Ie = 0xFFFF,

    // Sound Registers
    Nr10 = 0xFF10,
    Nr11 = 0xFF11,
    Nr12 = 0xFF12,
    Nr13 = 0xFF13,
    Nr14 = 0xFF14,
    Nr21 = 0xFF16,
    Nr22 = 0xFF17,
    Nr23 = 0xFF18,
    Nr24 = 0xFF19,
    Nr30 = 0xFF1A,
    Nr31 = 0xFF1B,
    Nr32 = 0xFF1C,
    Nr33 = 0xFF1D,
    Nr34 = 0xFF1E,
    Nr41 = 0xFF20,
    Nr42 = 0xFF21,
    Nr43 = 0xFF22,
    Nr44 = 0xFF23,
    Nr50 = 0xFF24,
    Nr51 = 0xFF25,
    Nr52 = 0xFF26,

    // Wave RAM (0xFF30-0xFF3F is handled as a range)
    WaveRamStart = 0xFF30,
    WaveRamEnd = 0xFF3F,

    // LCD and GPU
    Lcdc = 0xFF40,
    Stat = 0xFF41,
    Scy = 0xFF42,
    Scx = 0xFF43,
    Ly = 0xFF44,
    Lyc = 0xFF45,
    Dma = 0xFF46,
    Bgp = 0xFF47,
    Obp0 = 0xFF48,
    Obp1 = 0xFF49,
    Wy = 0xFF4A,
    Wx = 0xFF4B,

    // CGB-Specific Registers
    Key1 = 0xFF4D,
    Vbk = 0xFF4F,
    Hdma1 = 0xFF51,
    Hdma2 = 0xFF52,
    Hdma3 = 0xFF53,
    Hdma4 = 0xFF54,
    Hdma5 = 0xFF55,
    Rp = 0xFF56,
    Bcps = 0xFF68,
    Bcpd = 0xFF69,
    Ocps = 0xFF6A,
    Ocpd = 0xFF6B,
    Opri = 0xFF6C,
    Svbk = 0xFF70,
    Pcm12 = 0xFF76,
    Pcm34 = 0xFF77,
}
impl IoRegister {
    pub fn address(self) -> u16 {
        self as u16
    }
}
