// Compares state with logs from https://github.com/wheremyfoodat/Gameboy-logs

use crate::gameboy::Gameboy;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct CPUState {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: u8,
    sp: u16,
    pc: u16,
    mem: [u8; 4], // Next 4 bytes in memory from PC
}

pub struct TestHarness {
    expected_states: Vec<CPUState>,
    current_state: usize,
}

impl TestHarness {
    pub fn new(log_path: &Path) -> io::Result<Self> {
        let file = File::open(log_path)?;
        let reader = BufReader::new(file);
        let mut states = Vec::new();

        let re = Regex::new(r"A: ([0-9A-F]{2}) F: ([0-9A-F]{2}) B: ([0-9A-F]{2}) C: ([0-9A-F]{2}) D: ([0-9A-F]{2}) E: ([0-9A-F]{2}) H: ([0-9A-F]{2}) L: ([0-9A-F]{2}) SP: ([0-9A-F]{4}) PC: [0-9A-F]{2}:([0-9A-F]{4}) \(([0-9A-F]{2}) ([0-9A-F]{2}) ([0-9A-F]{2}) ([0-9A-F]{2})\)").unwrap();

        for line in reader.lines() {
            let line = line?;
            if let Some(caps) = re.captures(&line) {
                let state = CPUState {
                    a: u8::from_str_radix(&caps[1], 16).unwrap(),
                    f: u8::from_str_radix(&caps[2], 16).unwrap(),
                    b: u8::from_str_radix(&caps[3], 16).unwrap(),
                    c: u8::from_str_radix(&caps[4], 16).unwrap(),
                    d: u8::from_str_radix(&caps[5], 16).unwrap(),
                    e: u8::from_str_radix(&caps[6], 16).unwrap(),
                    h: u8::from_str_radix(&caps[7], 16).unwrap(),
                    l: u8::from_str_radix(&caps[8], 16).unwrap(),
                    sp: u16::from_str_radix(&caps[9], 16).unwrap(),
                    pc: u16::from_str_radix(&caps[10], 16).unwrap(),
                    mem: [
                        u8::from_str_radix(&caps[11], 16).unwrap(),
                        u8::from_str_radix(&caps[12], 16).unwrap(),
                        u8::from_str_radix(&caps[13], 16).unwrap(),
                        u8::from_str_radix(&caps[14], 16).unwrap(),
                    ],
                };
                states.push(state);
            }
        }

        Ok(Self {
            expected_states: states,
            current_state: 0,
        })
    }

    pub fn get_current_state(&self, gb: &Gameboy) -> CPUState {
        let bus = gb.bus.borrow();
        CPUState {
            a: gb.cpu.a,
            b: gb.cpu.b,
            c: gb.cpu.c,
            d: gb.cpu.d,
            e: gb.cpu.e,
            h: gb.cpu.h,
            l: gb.cpu.l,
            f: gb.cpu.f.bits(),
            sp: gb.cpu.sp,
            pc: gb.cpu.pc,
            mem: [
                bus.read_byte(gb.cpu.pc),
                bus.read_byte(gb.cpu.pc.wrapping_add(1)),
                bus.read_byte(gb.cpu.pc.wrapping_add(2)),
                bus.read_byte(gb.cpu.pc.wrapping_add(3)),
            ],
        }
    }

    pub fn step(&mut self, gb: &Gameboy) -> Result<(), String> {
        if self.current_state >= self.expected_states.len() {
            return Err("Test completed successfully".to_string());
        }

        let expected = &self.expected_states[self.current_state];
        let actual = self.get_current_state(gb);

        // Print side by side comparison
        println!("\nStep {}:", self.current_state);
        println!("Opcode    | {:02X}", expected.mem[0]);
        println!("Expected  | A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X}", 
            expected.a, expected.f, expected.b, expected.c, expected.d, expected.e, expected.h, expected.l, expected.sp, expected.pc);
        println!("Actual    | A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X}", 
            actual.a, actual.f, actual.b, actual.c, actual.d, actual.e, actual.h, actual.l, actual.sp, actual.pc);
        println!(
            "Memory Ex | {:02X} {:02X} {:02X} {:02X}",
            expected.mem[0], expected.mem[1], expected.mem[2], expected.mem[3]
        );
        println!(
            "Memory Ac | {:02X} {:02X} {:02X} {:02X}",
            actual.mem[0], actual.mem[1], actual.mem[2], actual.mem[3]
        );

        // Check if states match and print result
        let matches = actual == *expected;
        println!("Result    | {}", if matches { "PASS ✓" } else { "FAIL ✗" });

        // If states don't match, print specific differences
        if !matches {
            if actual.a != expected.a {
                println!("A: expected {:02X}, got {:02X}", expected.a, actual.a);
            }
            if actual.f != expected.f {
                println!("F: expected {:02X}, got {:02X}", expected.f, actual.f);
            }
            if actual.b != expected.b {
                println!("B: expected {:02X}, got {:02X}", expected.b, actual.b);
            }
            if actual.c != expected.c {
                println!("C: expected {:02X}, got {:02X}", expected.c, actual.c);
            }
            if actual.d != expected.d {
                println!("D: expected {:02X}, got {:02X}", expected.d, actual.d);
            }
            if actual.e != expected.e {
                println!("E: expected {:02X}, got {:02X}", expected.e, actual.e);
            }
            if actual.h != expected.h {
                println!("H: expected {:02X}, got {:02X}", expected.h, actual.h);
            }
            if actual.l != expected.l {
                println!("L: expected {:02X}, got {:02X}", expected.l, actual.l);
            }
            if actual.sp != expected.sp {
                println!("SP: expected {:04X}, got {:04X}", expected.sp, actual.sp);
            }
            if actual.pc != expected.pc {
                println!("PC: expected {:04X}, got {:04X}", expected.pc, actual.pc);
            }
            if actual.mem != expected.mem {
                println!(
                    "Memory: expected {:02X} {:02X} {:02X} {:02X}, got {:02X} {:02X} {:02X} {:02X}",
                    expected.mem[0],
                    expected.mem[1],
                    expected.mem[2],
                    expected.mem[3],
                    actual.mem[0],
                    actual.mem[1],
                    actual.mem[2],
                    actual.mem[3]
                );
            }

            return Err("States don't match".to_string());
        }

        self.current_state += 1;
        Ok(())
    }

    pub fn total_states(&self) -> usize {
        self.expected_states.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::flags::Flags;

    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_blargg_cpu_instructions() -> io::Result<()> {
        let mut gb = Gameboy::new();

        // Set initial CPU state to match test expectations
        gb.cpu.a = 0x01;
        gb.cpu.f.set(Flags::N, false);
        gb.cpu.f.set(Flags::H, true);
        gb.cpu.f.set(Flags::C, true);
        gb.cpu.f.set(Flags::Z, true);
        gb.cpu.b = 0x00;
        gb.cpu.c = 0x13;
        gb.cpu.d = 0x00;
        gb.cpu.e = 0xD8;
        gb.cpu.h = 0x01;
        gb.cpu.l = 0x4D;
        gb.cpu.sp = 0xFFFE;
        gb.cpu.pc = 0x0100;

        gb.bus.borrow_mut().write_byte(0xFF44, 0x90);

        // Load test ROM
        let rom = std::fs::read("test/blargg/02-interrupts.gb")?;
        gb.load_rom(&rom);

        let mut harness = TestHarness::new(Path::new(
            "test/Gameboy-logs-master/Blargg2LYStubbed/EpicLog.txt",
        ))?;

        let mut step_count = 0;
        while let Ok(()) = harness.step(&gb) {
            gb.tick();
            step_count += 1;
        }

        Ok(())
    }
}
