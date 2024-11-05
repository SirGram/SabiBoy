use crate::cpu::registers::{Condition, Register16, Register16Mem, Register8};
use crate::cpu::CPU;

impl CPU {
    pub fn ld_r8_r8(&mut self, register1: Register8, register2: Register8) {
        self.set_r8(&register1, self.get_r8(&register2));
    }
    pub fn halt(&mut self) {
        /*
        TODO: Implement HALT instruction
        Enter CPU low-power consumption mode until an interrupt occurs. The exact behavior of this instruction depends on the state of the IME flag.

        IME set
        The CPU enters low-power mode until after an interrupt is about to be serviced. The handler is executed normally, and the CPU resumes execution after the HALT when that returns.
        IME not set
        The behavior depends on whether an interrupt is pending (i.e. ‘[IE] & [IF]’ is non-zero).
        None pending
        As soon as an interrupt becomes pending, the CPU resumes execution. This is like the above, except that the handler is not called.
        Some pending
        The CPU continues execution after the HALT, but the byte after it is read twice in a row (PC is not incremented, due to a hardware bug). */
    }
}
