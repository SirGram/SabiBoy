pub mod block0;
pub mod block1;
pub mod block2;
pub mod block3;
pub mod opcode_cycles;

use crate::cpu::flags::Condition;
use crate::cpu::{Flags, CPU};

impl CPU {
    pub fn should_jump(&self, condition: Condition) -> bool {
        /* Jump to address relative to PC based on condition
        * Conditions:
        - NZ: Not zero
        - Z: Zero
        - NC: Not carry
        - C: Carry
        */
        match condition {
            Condition::NZ => !self.f.contains(Flags::Z),
            Condition::Z => self.f.contains(Flags::Z),
            Condition::NC => !self.f.contains(Flags::C),
            Condition::C => self.f.contains(Flags::C),
        }
    }
}
