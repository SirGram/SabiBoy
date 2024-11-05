pub mod block0;
pub mod block1;
pub mod block2;
pub mod block3;

use crate::cpu::{Condition, Flags, CPU};

impl CPU {
    pub fn should_jump(&mut self, condition: Condition) -> bool {
        match condition {
            Condition::NZ => !self.f.contains(Flags::Z),
            Condition::Z => self.f.contains(Flags::Z),
            Condition::NC => !self.f.contains(Flags::C),
            Condition::C => self.f.contains(Flags::C),
        }
    }
}
