use crate::cpu::{flags::Condition, CPU};

impl CPU {
    pub fn clock_cycles(&self, opcode: u8, is_cb__prefix: bool) -> u64 {
        if is_cb__prefix {
            match opcode {
                0x00 => 4,  // RLC B
                0x01 => 4,  // RLC C
                0x02 => 4,  // RLC D
                0x03 => 4,  // RLC E
                0x04 => 4,  // RLC H
                0x05 => 4,  // RLC L
                0x06 => 12, // RLC (HL)
                0x07 => 4,  // RLC A
                0x08 => 4,  // RRC B
                0x09 => 4,  // RRC C
                0x0A => 4,  // RRC D
                0x0B => 4,  // RRC E
                0x0C => 4,  // RRC H
                0x0D => 4,  // RRC L
                0x0E => 12, // RRC (HL)
                0x0F => 4,  // RRC A
                0x10 => 4,  // RL B
                0x11 => 4,  // RL C
                0x12 => 4,  // RL D
                0x13 => 4,  // RL E
                0x14 => 4,  // RL H
                0x15 => 4,  // RL L
                0x16 => 12, // RL (HL)
                0x17 => 4,  // RL A
                0x18 => 4,  // RR B
                0x19 => 4,  // RR C
                0x1A => 4,  // RR D
                0x1B => 4,  // RR E
                0x1C => 4,  // RR H
                0x1D => 4,  // RR L
                0x1E => 12, // RR (HL)
                0x1F => 4,  // RR A
                0x20 => 4,  // SLA B
                0x21 => 4,  // SLA C
                0x22 => 4,  // SLA D
                0x23 => 4,  // SLA E
                0x24 => 4,  // SLA H
                0x25 => 4,  // SLA L
                0x26 => 12, // SLA (HL)
                0x27 => 4,  // SLA A
                0x28 => 4,  // SRA B
                0x29 => 4,  // SRA C
                0x2A => 4,  // SRA D
                0x2B => 4,  // SRA E
                0x2C => 4,  // SRA H
                0x2D => 4,  // SRA L
                0x2E => 12, // SRA (HL)
                0x2F => 4,  // SRA A
                0x30 => 4,  // SWAP B
                0x31 => 4,  // SWAP C
                0x32 => 4,  // SWAP D
                0x33 => 4,  // SWAP E
                0x34 => 4,  // SWAP H
                0x35 => 4,  // SWAP L
                0x36 => 12, // SWAP (HL)
                0x37 => 4,  // SWAP A
                0x38 => 4,  // SRL B
                0x39 => 4,  // SRL C
                0x3A => 4,  // SRL D
                0x3B => 4,  // SRL E
                0x3C => 4,  // SRL H
                0x3D => 4,  // SRL L
                0x3E => 12, // SRL (HL)
                0x3F => 4,  // SRL A
                0x40 => 4,  // BIT 0, B
                0x41 => 4,  // BIT 0, C
                0x42 => 4,  // BIT 0, D
                0x43 => 4,  // BIT 0, E
                0x44 => 4,  // BIT 0, H
                0x45 => 4,  // BIT 0, L
                0x46 => 8,  // BIT 0, (HL)
                0x47 => 4,  // BIT 0, A
                0x48 => 4,  // BIT 1, B
                0x49 => 4,  // BIT 1, C
                0x4A => 4,  // BIT 1, D
                0x4B => 4,  // BIT 1, E
                0x4C => 4,  // BIT 1, H
                0x4D => 4,  // BIT 1, L
                0x4E => 8,  // BIT 1, (HL)
                0x4F => 4,  // BIT 1, A
                0x50 => 4,  // BIT 2, B
                0x51 => 4,  // BIT 2, C
                0x52 => 4,  // BIT 2, D
                0x53 => 4,  // BIT 2, E
                0x54 => 4,  // BIT 2, H
                0x55 => 4,  // BIT 2, L
                0x56 => 8,  // BIT 2, (HL)
                0x57 => 4,  // BIT 2, A
                0x58 => 4,  // BIT 3, B
                0x59 => 4,  // BIT 3, C
                0x5A => 4,  // BIT 3, D
                0x5B => 4,  // BIT 3, E
                0x5C => 4,  // BIT 3, H
                0x5D => 4,  // BIT 3, L
                0x5E => 8,  // BIT 3, (HL)
                0x5F => 4,  // BIT 3, A
                0x60 => 4,  // BIT 4, B
                0x61 => 4,  // BIT 4, C
                0x62 => 4,  // BIT 4, D
                0x63 => 4,  // BIT 4, E
                0x64 => 4,  // BIT 4, H
                0x65 => 4,  // BIT 4, L
                0x66 => 8,  // BIT 4, (HL)
                0x67 => 4,  // BIT 4, A
                0x68 => 4,  // BIT 5, B
                0x69 => 4,  // BIT 5, C
                0x6A => 4,  // BIT 5, D
                0x6B => 4,  // BIT 5, E
                0x6C => 4,  // BIT 5, H
                0x6D => 4,  // BIT 5, L
                0x6E => 8,  // BIT 5, (HL)
                0x6F => 4,  // BIT 5, A
                0x70 => 4,  // BIT 6, B
                0x71 => 4,  // BIT 6, C
                0x72 => 4,  // BIT 6, D
                0x73 => 4,  // BIT 6, E
                0x74 => 4,  // BIT 6, H
                0x75 => 4,  // BIT 6, L
                0x76 => 8,  // BIT 6, (HL)
                0x77 => 4,  // BIT 6, A
                0x78 => 4,  // BIT 7, B
                0x79 => 4,  // BIT 7, C
                0x7A => 4,  // BIT 7, D
                0x7B => 4,  // BIT 7, E
                0x7C => 4,  // BIT 7, H
                0x7D => 4,  // BIT 7, L
                0x7E => 8,  // BIT 7, (HL)
                0x7F => 4,  // BIT 7, A
                0x80 => 4,  // RES 0, B
                0x81 => 4,  // RES 0, C
                0x82 => 4,  // RES 0, D
                0x83 => 4,  // RES 0, E
                0x84 => 4,  // RES 0, H
                0x85 => 4,  // RES 0, L
                0x86 => 12, // RES 0, (HL)
                0x87 => 4,  // RES 0, A
                0x88 => 4,  // RES 1, B
                0x89 => 4,  // RES 1, C
                0x8A => 4,  // RES 1, D
                0x8B => 4,  // RES 1, E
                0x8C => 4,  // RES 1, H
                0x8D => 4,  // RES 1, L
                0x8E => 12, // RES 1, (HL)
                0x8F => 4,  // RES 1, A
                0x90 => 4,  // RES 2, B
                0x91 => 4,  // RES 2, C
                0x92 => 4,  // RES 2, D
                0x93 => 4,  // RES 2, E
                0x94 => 4,  // RES 2, H
                0x95 => 4,  // RES 2, L
                0x96 => 12, // RES 2, (HL)
                0x97 => 4,  // RES 2, A
                0x98 => 4,  // RES 3, B
                0x99 => 4,  // RES 3, C
                0x9A => 4,  // RES 3, D
                0x9B => 4,  // RES 3, E
                0x9C => 4,  // RES 3, H
                0x9D => 4,  // RES 3, L
                0x9E => 12, // RES 3, (HL)
                0x9F => 4,  // RES 3, A
                0xA0 => 4,  // RES 4, B
                0xA1 => 4,  // RES 4, C
                0xA2 => 4,  // RES 4, D
                0xA3 => 4,  // RES 4, E
                0xA4 => 4,  // RES 4, H
                0xA5 => 4,  // RES 4, L
                0xA6 => 12, // RES 4, (HL)
                0xA7 => 4,  // RES 4, A
                0xA8 => 4,  // RES 5, B
                0xA9 => 4,  // RES 5, C
                0xAA => 4,  // RES 5, D
                0xAB => 4,  // RES 5, E
                0xAC => 4,  // RES 5, H
                0xAD => 4,  // RES 5, L
                0xAE => 12, // RES 5, (HL)
                0xAF => 4,  // RES 5, A
                0xB0 => 4,  // RES 6, B
                0xB1 => 4,  // RES 6, C
                0xB2 => 4,  // RES 6, D
                0xB3 => 4,  // RES 6, E
                0xB4 => 4,  // RES 6, H
                0xB5 => 4,  // RES 6, L
                0xB6 => 12, // RES 6, (HL)
                0xB7 => 4,  // RES 6, A
                0xB8 => 4,  // RES 7, B
                0xB9 => 4,  // RES 7, C
                0xBA => 4,  // RES 7, D
                0xBB => 4,  // RES 7, E
                0xBC => 4,  // RES 7, H
                0xBD => 4,  // RES 7, L
                0xBE => 12, // RES 7, (HL)
                0xBF => 4,  // RES 7, A
                0xC0 => 4,  // SET 0, B
                0xC1 => 4,  // SET 0, C
                0xC2 => 4,  // SET 0, D
                0xC3 => 4,  // SET 0, E
                0xC4 => 4,  // SET 0, H
                0xC5 => 4,  // SET 0, L
                0xC6 => 12, // SET 0, (HL)
                0xC7 => 4,  // SET 0, A
                0xC8 => 4,  // SET 1, B
                0xC9 => 4,  // SET 1, C
                0xCA => 4,  // SET 1, D
                0xCB => 4,  // SET 1, E
                0xCC => 4,  // SET 1, H
                0xCD => 4,  // SET 1, L
                0xCE => 12, // SET 1, (HL)
                0xCF => 4,  // SET 1, A
                0xD0 => 4,  // SET 2, B
                0xD1 => 4,  // SET 2, C
                0xD2 => 4,  // SET 2, D
                0xD3 => 4,  // SET 2, E
                0xD4 => 4,  // SET 2, H
                0xD5 => 4,  // SET 2, L
                0xD6 => 12, // SET 2, (HL)
                0xD7 => 4,  // SET 2, A
                0xD8 => 4,  // SET 3, B
                0xD9 => 4,  // SET 3, C
                0xDA => 4,  // SET 3, D
                0xDB => 4,  // SET 3, E
                0xDC => 4,  // SET 3, H
                0xDD => 4,  // SET 3, L
                0xDE => 12, // SET 3, (HL)
                0xDF => 4,  // SET 3, A
                0xE0 => 4,  // SET 4, B
                0xE1 => 4,  // SET 4, C
                0xE2 => 4,  // SET 4, D
                0xE3 => 4,  // SET 4, E
                0xE4 => 4,  // SET 4, H
                0xE5 => 4,  // SET 4, L
                0xE6 => 12, // SET 4, (HL)
                0xE7 => 4,  // SET 4, A
                0xE8 => 4,  // SET 5, B
                0xE9 => 4,  // SET 5, C
                0xEA => 4,  // SET 5, D
                0xEB => 4,  // SET 5, E
                0xEC => 4,  // SET 5, H
                0xED => 4,  // SET 5, L
                0xEE => 12, // SET 5, (HL)
                0xEF => 4,  // SET 5, A
                0xF0 => 4,  // SET 6, B
                0xF1 => 4,  // SET 6, C
                0xF2 => 4,  // SET 6, D
                0xF3 => 4,  // SET 6, E
                0xF4 => 4,  // SET 6, H
                0xF5 => 4,  // SET 6, L
                0xF6 => 12, // SET 6, (HL)
                0xF7 => 4,  // SET 6, A
                0xF8 => 4,  // SET 7, B
                0xF9 => 4,  // SET 7, C
                0xFA => 4,  // SET 7, D
                0xFB => 4,  // SET 7, E
                0xFC => 4,  // SET 7, H
                0xFD => 4,  // SET 7, L
                0xFE => 12, // SET 7, (HL)
                0xFF => 4,  // SET 7, A
            }
        } else {
            // check if opcode branches
            let is_branching = match opcode {
                // opcodes with condition
                0x20 | 0x28 | 0x30 | 0x48 | 0xC0 | 0xC2 | 0xC4 | 0xC8 | 0xCA | 0xCC | 0xD0
                | 0xD2 | 0xD4 | 0xD8 | 0xDA | 0xDC => true,
                _ => false,
            };

            // T cycles. 1M = 4T
            if is_branching {
                match opcode {
                    0x20 => {
                        if self.should_jump(Condition::NZ) {
                            12
                        } else {
                            8
                        }
                    }
                    0xC0 => {
                        if self.should_jump(Condition::NZ) {
                            20
                        } else {
                            8
                        }
                    }
                    0xC2 => {
                        if self.should_jump(Condition::NZ) {
                            16
                        } else {
                            12
                        }
                    }
                    0xC4 => {
                        if self.should_jump(Condition::NZ) {
                            24
                        } else {
                            12
                        }
                    }

                    0x28 => {
                        if self.should_jump(Condition::Z) {
                            12
                        } else {
                            8
                        }
                    }
                    0xC8 => {
                        if self.should_jump(Condition::Z) {
                            20
                        } else {
                            8
                        }
                    }
                    0xCA => {
                        if self.should_jump(Condition::Z) {
                            16
                        } else {
                            12
                        }
                    }
                    0xCC => {
                        if self.should_jump(Condition::Z) {
                            24
                        } else {
                            12
                        }
                    }

                    0x48 => {
                        if self.should_jump(Condition::C) {
                            12
                        } else {
                            8
                        }
                    }
                    0xD8 => {
                        if self.should_jump(Condition::C) {
                            20
                        } else {
                            8
                        }
                    }
                    0xDA => {
                        if self.should_jump(Condition::C) {
                            16
                        } else {
                            12
                        }
                    }
                    0xDC => {
                        if self.should_jump(Condition::C) {
                            24
                        } else {
                            12
                        }
                    }

                    0x30 => {
                        if self.should_jump(Condition::NC) {
                            12
                        } else {
                            8
                        }
                    }
                    0xD0 => {
                        if self.should_jump(Condition::NC) {
                            20
                        } else {
                            8
                        }
                    }
                    0xD2 => {
                        if self.should_jump(Condition::NC) {
                            16
                        } else {
                            12
                        }
                    }
                    0xD4 => {
                        if self.should_jump(Condition::NC) {
                            24
                        } else {
                            12
                        }
                    }

                    _ => panic!("Unknown branching opcode: {:#X}", opcode),
                }
            } else {
                match opcode {
                    0x00 => 4,  // NOP
                    0x01 => 12, // LD BC, nn
                    0x02 => 8,  // LD (BC), A
                    0x03 => 8,  // INC BC
                    0x04 => 4,  // INC B
                    0x05 => 4,  // DEC B
                    0x06 => 8,  // LD B, n
                    0x07 => 4,  // RLCA
                    0x08 => 20, // LD (nn), SP
                    0x09 => 8,  // ADD HL, BC
                    0x0A => 8,  // LD A, (BC)
                    0x0B => 8,  // DEC BC
                    0x0C => 4,  // INC C
                    0x0D => 4,  // DEC C
                    0x0E => 8,  // LD C, n
                    0x0F => 4,  // RRCA

                    0x10 => 4,  // STOP
                    0x11 => 12, // LD DE, nn
                    0x12 => 8,  // LD (DE), A
                    0x13 => 8,  // INC DE
                    0x14 => 4,  // INC D
                    0x15 => 4,  // DEC D
                    0x16 => 8,  // LD D, n
                    0x17 => 4,  // RLA
                    0x18 => 12, // JR r
                    0x19 => 8,  // ADD HL, DE
                    0x1A => 8,  // LD A, (DE)
                    0x1B => 8,  // DEC DE
                    0x1C => 4,  // INC E
                    0x1D => 4,  // DEC E
                    0x1E => 8,  // LD E, n
                    0x1F => 4,  // RRA

                    0x20 => 12, // JR NZ, r
                    0x21 => 12, // LD HL, nn
                    0x22 => 8,  // LD (HL+), A
                    0x23 => 8,  // INC HL
                    0x24 => 4,  // INC H
                    0x25 => 4,  // DEC H
                    0x26 => 8,  // LD H, n
                    0x27 => 4,  // DAA
                    0x28 => 12, // JR Z, r
                    0x29 => 8,  // ADD HL, HL
                    0x2A => 8,  // LD A, (HL+)
                    0x2B => 8,  // DEC HL
                    0x2C => 4,  // INC L
                    0x2D => 4,  // DEC L
                    0x2E => 8,  // LD L, n
                    0x2F => 4,  // CPL

                    0x30 => 12, // JR NC, r
                    0x31 => 12, // LD SP, nn
                    0x32 => 8,  // LD (HL-), A
                    0x33 => 8,  // INC SP
                    0x34 => 12, // INC (HL)
                    0x35 => 12, // DEC (HL)
                    0x36 => 12, // LD (HL), n
                    0x37 => 4,  // SCF
                    0x38 => 12, // JR C, r
                    0x39 => 8,  // ADD HL, SP
                    0x3A => 8,  // LD A, (HL-)
                    0x3B => 8,  // DEC SP
                    0x3C => 4,  // INC A
                    0x3D => 4,  // DEC A
                    0x3E => 8,  // LD A, n
                    0x3F => 4,  // CCF

                    0x40 => 4, // LD B, B
                    0x41 => 4, // LD B, C
                    0x42 => 4, // LD B, D
                    0x43 => 4, // LD B, E
                    0x44 => 4, // LD B, H
                    0x45 => 4, // LD B, L
                    0x46 => 8, // LD B, (HL)
                    0x47 => 4, // LD B, A
                    0x48 => 4, // LD C, B
                    0x49 => 4, // LD C, C
                    0x4A => 4, // LD C, D
                    0x4B => 4, // LD C, E
                    0x4C => 4, // LD C, H
                    0x4D => 4, // LD C, L
                    0x4E => 8, // LD C, (HL)
                    0x4F => 4, // LD C, A

                    0x50 => 4, // LD D, B
                    0x51 => 4, // LD D, C
                    0x52 => 4, // LD D, D
                    0x53 => 4, // LD D, E
                    0x54 => 4, // LD D, H
                    0x55 => 4, // LD D, L
                    0x56 => 8, // LD D, (HL)
                    0x57 => 4, // LD D, A
                    0x58 => 4, // LD E, B
                    0x59 => 4, // LD E, C
                    0x5A => 4, // LD E, D
                    0x5B => 4, // LD E, E
                    0x5C => 4, // LD E, H
                    0x5D => 4, // LD E, L
                    0x5E => 8, // LD E, (HL)
                    0x5F => 4, // LD E, A

                    0x60 => 4, // LD H, B
                    0x61 => 4, // LD H, C
                    0x62 => 4, // LD H, D
                    0x63 => 4, // LD H, E
                    0x64 => 4, // LD H, H
                    0x65 => 4, // LD H, L
                    0x66 => 8, // LD H, (HL)
                    0x67 => 4, // LD H, A
                    0x68 => 4, // LD L, B
                    0x69 => 4, // LD L, C
                    0x6A => 4, // LD L, D
                    0x6B => 4, // LD L, E
                    0x6C => 4, // LD L, H
                    0x6D => 4, // LD L, L
                    0x6E => 8, // LD L, (HL)
                    0x6F => 4, // LD L, A

                    0x70 => 8, // LD (HL), B
                    0x71 => 8, // LD (HL), C
                    0x72 => 8, // LD (HL), D
                    0x73 => 8, // LD (HL), E
                    0x74 => 8, // LD (HL), H
                    0x75 => 8, // LD (HL), L
                    0x76 => 4, // HALT
                    0x77 => 8, // LD (HL), A
                    0x78 => 4, // LD A, B
                    0x79 => 4, // LD A, C
                    0x7A => 4, // LD A, D
                    0x7B => 4, // LD A, E
                    0x7C => 4, // LD A, H
                    0x7D => 4, // LD A, L
                    0x7E => 8, // LD A, (HL)
                    0x7F => 4, // LD A, A

                    0x80 => 4, // ADD A, B
                    0x81 => 4, // ADD A, C
                    0x82 => 4, // ADD A, D
                    0x83 => 4, // ADD A, E
                    0x84 => 4, // ADD A, H
                    0x85 => 4, // ADD A, L
                    0x86 => 8, // ADD A, (HL)
                    0x87 => 4, // ADD A, A
                    0x88 => 4, // ADC A, B
                    0x89 => 4, // ADC A, C
                    0x8A => 4, // ADC A, D
                    0x8B => 4, // ADC A, E
                    0x8C => 4, // ADC A, H
                    0x8D => 4, // ADC A, L
                    0x8E => 8, // ADC A, (HL)
                    0x8F => 4, // ADC A, A

                    0x90 => 4, // SUB B
                    0x91 => 4, // SUB C
                    0x92 => 4, // SUB D
                    0x93 => 4, // SUB E
                    0x94 => 4, // SUB H
                    0x95 => 4, // SUB L
                    0x96 => 8, // SUB (HL)
                    0x97 => 4, // SUB A
                    0x98 => 4, // SBC A, B
                    0x99 => 4, // SBC A, C
                    0x9A => 4, // SBC A, D
                    0x9B => 4, // SBC A, E
                    0x9C => 4, // SBC A, H
                    0x9D => 4, // SBC A, L
                    0x9E => 8, // SBC A, (HL)
                    0x9F => 4, // SBC A, A

                    0xA0 => 4, // AND B
                    0xA1 => 4, // AND C
                    0xA2 => 4, // AND D
                    0xA3 => 4, // AND E
                    0xA4 => 4, // AND H
                    0xA5 => 4, // AND L
                    0xA6 => 8, // AND (HL)
                    0xA7 => 4, // AND A
                    0xA8 => 4, // XOR B
                    0xA9 => 4, // XOR C
                    0xAA => 4, // XOR D
                    0xAB => 4, // XOR E
                    0xAC => 4, // XOR H
                    0xAD => 4, // XOR L
                    0xAE => 8, // XOR (HL)
                    0xAF => 4, // XOR A

                    0xB0 => 4, // OR B
                    0xB1 => 4, // OR C
                    0xB2 => 4, // OR D
                    0xB3 => 4, // OR E
                    0xB4 => 4, // OR H
                    0xB5 => 4, // OR L
                    0xB6 => 8, // OR (HL)
                    0xB7 => 4, // OR A
                    0xB8 => 4, // CP B
                    0xB9 => 4, // CP C
                    0xBA => 4, // CP D
                    0xBB => 4, // CP E
                    0xBC => 4, // CP H
                    0xBD => 4, // CP L
                    0xBE => 8, // CP (HL)
                    0xBF => 4, // CP A

                    0xC0 => 20, // RET NZ
                    0xC1 => 12, // POP BC
                    0xC2 => 16, // JP NZ, nn
                    0xC3 => 16, // JP nn
                    0xC4 => 24, // CALL NZ, nn
                    0xC5 => 16, // PUSH BC
                    0xC6 => 8,  // ADD A, n
                    0xC7 => 16, // RST 00H
                    0xC8 => 20, // RET Z
                    0xC9 => 16, // RET
                    0xCA => 16, // JP Z, nn
                    0xCB => 4,  // CB prefix
                    0xCC => 24, // CALL Z, nn
                    0xCD => 24, // CALL nn
                    0xCE => 8,  // ADC A, n
                    0xCF => 16, // RST 08H

                    0xD0 => 20, // RET NC
                    0xD1 => 12, // POP DE
                    0xD2 => 16, // JP NC, nn
                    0xD3 => 16, // OUT (n), A
                    0xD4 => 24, // CALL NC, nn
                    0xD5 => 16, // PUSH DE
                    0xD6 => 8,  // SUB n
                    0xD7 => 16, // RST 10H
                    0xD8 => 20, // RET C
                    0xD9 => 16, // RETI
                    0xDA => 16, // JP C, nn
                    0xDB => 16, // IN A, (n)
                    0xDC => 24, // CALL C, nn
                    0xDD => 0,  // IX prefix
                    0xDE => 8,  // SBC A, n
                    0xDF => 16, // RST 18H

                    0xE0 => 12, // LDH (n), A
                    0xE1 => 12, // POP HL
                    0xE2 => 8,  // LD (C), A
                    0xE3 => 19, // EX (SP), HL
                    0xE4 => 24, // CALL PO, nn
                    0xE5 => 16, // PUSH HL
                    0xE6 => 8,  // AND n
                    0xE7 => 16, // RST 20H
                    0xE8 => 8,  // ADD SP, n
                    0xE9 => 4,  // JP (HL)
                    0xEA => 16, // LD (nn), A
                    0xEB => 4,  // EX DE, HL
                    0xEC => 24, // CALL PE, nn
                    0xED => 0,  // ED prefix
                    0xEE => 8,  // XOR n
                    0xEF => 16, // RST 28H

                    0xF0 => 12, // LDH A, (n)
                    0xF1 => 12, // POP AF
                    0xF2 => 8,  // LD A, (C)
                    0xF3 => 4,  // DI
                    0xF4 => 24, // CALL P, nn
                    0xF5 => 16, // PUSH AF
                    0xF6 => 8,  // OR n
                    0xF7 => 16, // RST 30H
                    0xF8 => 16, // LD HL, SP + n
                    0xF9 => 12,
                    0xFA => 16, // LD A, (nn)
                    0xFB => 4,  // EI
                    0xFE => 8,  // CP n
                    0xFF => 16, // RST 38H

                    _ => panic!("Invald opcode: {:#X}", opcode),
                }
            }
        }
    }
}
