use std::{fs, path::Path};

use crate::{bus::io_address::IoRegister, gameboy::Gameboy};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct CPUTest {
    name: String,
    initial: CPUState,
    #[serde(rename = "final")]
    final_state: CPUState,
    cycles: Vec<CycleState>,
}

#[derive(Debug, Deserialize)]
struct CPUState {
    pc: u16,
    sp: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    ime: u8,
    #[serde(default)]
    ei: u8,
    ram: Vec<[u16; 2]>,
}

#[derive(Debug, Deserialize)]
#[serde(from = "Vec<serde_json::Value>")]
struct CycleState {
    address: Option<u16>,
    data: Option<u8>,
    bus_state: String,
}

impl From<Vec<serde_json::Value>> for CycleState {
    fn from(v: Vec<serde_json::Value>) -> Self {
        CycleState {
            address: v.get(0).and_then(|v| v.as_u64()).map(|n| n as u16),
            data: v.get(1).and_then(|v| v.as_u64()).map(|n| n as u8),
            bus_state: v
                .get(2)
                .and_then(|v| v.as_str())
                .unwrap_or("---")
                .to_string(),
        }
    }
}

impl Gameboy {
    pub fn run_tests(&mut self, from: usize, to: Option<usize>) {
        // Get the path to the sm83 test directory
        let sm83_dir = Path::new("test/sm83");

        let mut total_cycles = 0;
        // Check if the directory exists
        if sm83_dir.is_dir() {
            // Read all .json files from the directory
            let entries = fs::read_dir(sm83_dir).unwrap_or_else(|_| {
                eprintln!("Failed to read test directory.");
                std::process::exit(1);
            });

            // Collect all test files
            let test_files: Vec<_> = entries
                .filter_map(Result::ok)
                .filter(|entry| {
                    entry
                        .path()
                        .extension()
                        .map(|e| e == "json")
                        .unwrap_or(false)
                })
                .collect();

            // Loop through the specified range of test files
            for (test_idx, entry) in test_files.iter().enumerate() {
                // Check if the current test is within the from-to range
                if test_idx < from {
                    continue; // Skip tests before the "from" index
                }
                if let Some(to_idx) = to {
                    if test_idx > to_idx {
                        break; // Stop after running tests up to the "to" index
                    }
                }

                let path = entry.path();
                // Read the contents of the test file
                let test_json = fs::read_to_string(&path).unwrap_or_else(|_| {
                    eprintln!("Failed to read test file: {:?}", path);
                    std::process::exit(1);
                });

                // Run the test with the current test JSON
                match self.run_test(&test_json, &mut total_cycles) {
                    Ok(_) => {
                        println!("Test passed successfully: {:?}", path);
                    }
                    Err(e) => {
                        eprintln!("Test failed for {:?}: {}", path, e);
                        // Stop after the first failure
                        break; // Exit the loop on failure
                    }
                }
            }
        } else {
            eprintln!("Test directory does not exist: {:?}", sm83_dir);
        }
    }
    pub fn run_test(&mut self, test_json: &str, total_cycles: &mut u64) -> Result<(), String> {
        let test: Vec<CPUTest> = serde_json::from_str(test_json)
            .map_err(|e| format!("Failed to parse test JSON: {}", e))?;
        println!("Running {} tests", test.len());
        for (test_idx, test_case) in test.iter().enumerate() {
            print!(
                "Test {}/{}: {} ... ",
                test_idx + 1,
                test.len(),
                test_case.name
            );

            // Initialize CPU state
            self.init_cpu_state(&test_case.initial)?;

            // Run instruction and capture cycles
            self.tick();
            let expected_cycles_for_test = (test_case.cycles.len() * 4) as u64;
            *total_cycles += expected_cycles_for_test;

            println!("Cycles: {} {}", self.cpu.cycles, *total_cycles);

            // Validate and print detailed output on failure
            match self.validate_final_state(&test_case.final_state) {
                Ok(_) => println!("✓ PASS"),
                Err(e) => {
                    println!("✗ FAIL\n");
                    println!("Test '{}' failed: {}", test_case.name, e);
                    print_detailed_state_comparison(
                        self,
                        &test_case.final_state,
                        &test_case.initial,
                    );
                    return Err(format!("Test {} failed", test_idx));
                }
            }
        }
        Ok(())
    }

    fn init_cpu_state(&mut self, initial: &CPUState) -> Result<(), String> {
        // Set CPU registers directly since we have public access
        self.cpu.pc = initial.pc;
        self.cpu.sp = initial.sp;
        self.cpu.a = initial.a;
        self.cpu.b = initial.b;
        self.cpu.c = initial.c;
        self.cpu.d = initial.d;
        self.cpu.e = initial.e;
        self.cpu.f = crate::cpu::flags::Flags::from_bits_truncate(initial.f);
        self.cpu.h = initial.h;
        self.cpu.l = initial.l;

        self.cpu.ime = initial.ime != 0;

        // Initialize RAM
        let mut bus = self.bus.borrow_mut();
        for &[addr, value] in initial.ram.iter() {
            bus.write_byte(addr, value as u8);
        }

        Ok(())
    }

    fn validate_final_state(&self, expected: &CPUState) -> Result<(), String> {
        // Validate registers
        if self.cpu.pc != expected.pc {
            return Err(format!(
                "PC mismatch: expected {:04X}, got {:04X}",
                expected.pc, self.cpu.pc
            ));
        }

        if self.cpu.sp != expected.sp {
            return Err(format!(
                "SP mismatch: expected {:04X}, got {:04X}",
                expected.sp, self.cpu.sp
            ));
        }

        if self.cpu.a != expected.a {
            return Err(format!(
                "A mismatch: expected {:02X}, got {:02X}",
                expected.a, self.cpu.a
            ));
        }

        if self.cpu.b != expected.b {
            return Err(format!(
                "B mismatch: expected {:02X}, got {:02X}",
                expected.b, self.cpu.b
            ));
        }

        if self.cpu.c != expected.c {
            return Err(format!(
                "C mismatch: expected {:02X}, got {:02X}",
                expected.c, self.cpu.c
            ));
        }

        if self.cpu.d != expected.d {
            return Err(format!(
                "D mismatch: expected {:02X}, got {:02X}",
                expected.d, self.cpu.d
            ));
        }

        if self.cpu.e != expected.e {
            return Err(format!(
                "E mismatch: expected {:02X}, got {:02X}",
                expected.e, self.cpu.e
            ));
        }

        if self.cpu.f.bits() != expected.f {
            return Err(format!(
                "F mismatch: expected {:02X}, got {:02X}",
                expected.f,
                self.cpu.f.bits()
            ));
        }

        if self.cpu.h != expected.h {
            return Err(format!(
                "H mismatch: expected {:02X}, got {:02X}",
                expected.h, self.cpu.h
            ));
        }

        if self.cpu.l != expected.l {
            return Err(format!(
                "L mismatch: expected {:02X}, got {:02X}",
                expected.l, self.cpu.l
            ));
        }

        if (self.cpu.ime as u8) != expected.ime {
            return Err(format!(
                "IME mismatch: expected {:02X}, got {:02X}",
                expected.ime, self.cpu.ime as u8
            ));
        }

        // Validate RAM contents
        let borrowed_bus = self.bus.borrow();
        for &[addr, expected_value] in expected.ram.iter() {
            let actual_value = borrowed_bus.read_byte(addr);
            if actual_value != expected_value as u8 {
                return Err(format!(
                    "RAM mismatch at {:04X}: expected {:02X}, got {:02X}",
                    addr, expected_value, actual_value
                ));
            }
        }
        // validate ie register
        /* let ei = self.bus.borrow().read_byte(0xFFFF);
        if expected.ei != ei {
            return Err(format!(
                "IE mismatch: expected {:02X}, got {:02X}",
                expected.ei, ei
            ));
        } */

        Ok(())
    }
}
fn print_detailed_state_comparison(gb: &Gameboy, expected: &CPUState, initial: &CPUState) {
    println!("\nDetailed CPU State Comparison:");
    println!("Register  Initial   Expected  Actual");
    println!("----------------------------------");
    println!(
        "PC        0x{:04X}    0x{:04X}    0x{:04X}    {}",
        initial.pc,
        expected.pc,
        gb.cpu.pc,
        if expected.pc != gb.cpu.pc {
            "✗"
        } else {
            "✓"
        }
    );
    println!(
        "SP        0x{:04X}    0x{:04X}    0x{:04X}    {}",
        initial.sp,
        expected.sp,
        gb.cpu.sp,
        if expected.sp != gb.cpu.sp {
            "✗"
        } else {
            "✓"
        }
    );
    println!(
        "A         0x{:02X}      0x{:02X}      0x{:02X}      {}",
        initial.a,
        expected.a,
        gb.cpu.a,
        if expected.a != gb.cpu.a { "✗" } else { "✓" }
    );
    println!(
        "B         0x{:02X}      0x{:02X}      0x{:02X}      {}",
        initial.b,
        expected.b,
        gb.cpu.b,
        if expected.b != gb.cpu.b { "✗" } else { "✓" }
    );
    println!(
        "C         0x{:02X}      0x{:02X}      0x{:02X}      {}",
        initial.c,
        expected.c,
        gb.cpu.c,
        if expected.c != gb.cpu.c { "✗" } else { "✓" }
    );
    println!(
        "D         0x{:02X}      0x{:02X}      0x{:02X}      {}",
        initial.d,
        expected.d,
        gb.cpu.d,
        if expected.d != gb.cpu.d { "✗" } else { "✓" }
    );
    println!(
        "E         0x{:02X}      0x{:02X}      0x{:02X}      {}",
        initial.e,
        expected.e,
        gb.cpu.e,
        if expected.e != gb.cpu.e { "✗" } else { "✓" }
    );
    println!(
        "F         0x{:02X}      0x{:02X}      0x{:02X}      {}",
        initial.f,
        expected.f,
        gb.cpu.f.bits(),
        if expected.f != gb.cpu.f.bits() {
            "✗"
        } else {
            "✓"
        }
    );
    println!(
        "H         0x{:02X}      0x{:02X}      0x{:02X}      {}",
        initial.h,
        expected.h,
        gb.cpu.h,
        if expected.h != gb.cpu.h { "✗" } else { "✓" }
    );
    println!(
        "L         0x{:02X}      0x{:02X}      0x{:02X}      {}",
        initial.l,
        expected.l,
        gb.cpu.l,
        if expected.l != gb.cpu.l { "✗" } else { "✓" }
    );
    println!(
        "IME       0x{:02X}      0x{:02X}      0x{:02X}      {}",
        initial.ime,
        expected.ime,
        gb.cpu.ime as u8,
        if expected.ime != (gb.cpu.ime as u8) {
            "✗"
        } else {
            "✓"
        }
    );

    // Print memory differences
    let bus = gb.bus.borrow();
    let mut has_memory_differences = false;
    println!("\nMemory State:");
    println!("Address   Initial   Expected  Actual");
    println!("----------------------------------");

    // Collect all unique addresses from both initial and expected RAM
    let mut all_addresses: Vec<u16> = initial
        .ram
        .iter()
        .chain(expected.ram.iter())
        .map(|&[addr, _]| addr)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    all_addresses.sort();

    for addr in all_addresses {
        let initial_value = initial
            .ram
            .iter()
            .find(|&&[a, _]| a == addr)
            .map(|&[_, v]| v)
            .unwrap_or(0) as u8;
        let expected_value = expected
            .ram
            .iter()
            .find(|&&[a, _]| a == addr)
            .map(|&[_, v]| v)
            .unwrap_or(0) as u8;
        let actual_value = bus.read_byte(addr);

        if expected_value != actual_value || initial_value != actual_value {
            has_memory_differences = true;
            println!(
                "0x{:04X}   0x{:02X}      0x{:02X}      0x{:02X}      {}",
                addr,
                initial_value,
                expected_value,
                actual_value,
                if expected_value != actual_value {
                    "✗"
                } else {
                    "✓"
                }
            );
        }
    }
    if !has_memory_differences {
        println!("No memory differences found");
    }

    // Print flag bits breakdown if F register differs
    if expected.f != gb.cpu.f.bits() {
        println!("\nFlag Register Breakdown:");
        println!("Flag      Initial   Expected  Actual");
        println!("----------------------------------");
        let initial_flags = crate::cpu::flags::Flags::from_bits_truncate(initial.f);
        let expected_flags = crate::cpu::flags::Flags::from_bits_truncate(expected.f);
        println!(
            "Zero      {}         {}         {}        {}",
            (initial_flags.bits() & 0x80) != 0,
            (expected_flags.bits() & 0x80) != 0,
            (gb.cpu.f.bits() & 0x80) != 0,
            if (expected_flags.bits() & 0x80) == (gb.cpu.f.bits() & 0x80) {
                "✓"
            } else {
                "✗"
            }
        );
        println!(
            "Subtract  {}         {}         {}        {}",
            (initial_flags.bits() & 0x40) != 0,
            (expected_flags.bits() & 0x40) != 0,
            (gb.cpu.f.bits() & 0x40) != 0,
            if (expected_flags.bits() & 0x40) == (gb.cpu.f.bits() & 0x40) {
                "✓"
            } else {
                "✗"
            }
        );
        println!(
            "HalfCarry {}         {}         {}        {}",
            (initial_flags.bits() & 0x20) != 0,
            (expected_flags.bits() & 0x20) != 0,
            (gb.cpu.f.bits() & 0x20) != 0,
            if (expected_flags.bits() & 0x20) == (gb.cpu.f.bits() & 0x20) {
                "✓"
            } else {
                "✗"
            }
        );
        println!(
            "Carry     {}         {}         {}        {}",
            (initial_flags.bits() & 0x10) != 0,
            (expected_flags.bits() & 0x10) != 0,
            (gb.cpu.f.bits() & 0x10) != 0,
            if (expected_flags.bits() & 0x10) == (gb.cpu.f.bits() & 0x10) {
                "✓"
            } else {
                "✗"
            }
        );
    }
}
