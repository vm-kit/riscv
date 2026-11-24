//! vsie register

use crate::{
    bits::{bf_extract, bf_insert},
    CoreInterruptNumber,
};

read_write_csr! {
/// vsie register
    Vsie: 0x204,
    mask: usize::MAX,
}

read_write_csr_field! {
    Vsie,
    /// Supervisor Software Interrupt Enable
    ssie: 1,
}

read_write_csr_field! {
    Vsie,
    /// Supervisor Timer Interrupt Enable
    stie: 5,
}

read_write_csr_field! {
    Vsie,
    /// Supervisor External Interrupt Enable
    seie: 9,
}

impl Vsie {
    /// Check if a specific core interrupt source is enabled.
    #[inline]
    pub fn is_enabled<I: CoreInterruptNumber>(&self, interrupt: I) -> bool {
        bf_extract(self.bits, interrupt.number(), 1) != 0
    }

    /// Enable a specific core interrupt source.
    #[inline]
    pub fn enable<I: CoreInterruptNumber>(&mut self, interrupt: I) {
        self.bits = bf_insert(self.bits, interrupt.number(), 1, 1);
    }

    /// Disable a specific core interrupt source.
    #[inline]
    pub fn disable<I: CoreInterruptNumber>(&mut self, interrupt: I) {
        self.bits = bf_insert(self.bits, interrupt.number(), 1, 0);
    }
}

set!(0x204);
clear!(0x204);

set_clear_csr!(
    /// Supervisor Software Interrupt Enable
    , set_ssie, clear_ssie, 1 << 1);
set_clear_csr!(
    /// Supervisor Timer Interrupt Enable
    , set_stie, clear_stie, 1 << 5);
set_clear_csr!(
    /// Supervisor External Interrupt Enable
    , set_seie, clear_seie, 1 << 9);

/// Disables a specific core interrupt source.
#[inline]
pub fn disable<I: CoreInterruptNumber>(interrupt: I) {
    // SAFETY: it is safe to disable an interrupt source
    unsafe { _clear(1 << interrupt.number()) };
}

/// Enables a specific core interrupt source.
///
/// # Safety
///
/// Enabling interrupts might break critical sections or other synchronization mechanisms.
/// Ensure that this is called in a safe context where interrupts can be enabled.
#[inline]
pub unsafe fn enable<I: CoreInterruptNumber>(interrupt: I) {
    unsafe { _set(1 << interrupt.number()) };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interrupt::supervisor::Interrupt;

    #[test]
    fn test_vsie() {
        let mut vsie = Vsie::from_bits(0);

        test_csr_field!(vsie, ssie);
        test_csr_field!(vsie, stie);
        test_csr_field!(vsie, seie);
    }

    #[test]
    fn test_vsie_interrupt() {
        let mut s = Vsie::from_bits(0);

        s.enable(Interrupt::SupervisorSoft);
        assert!(s.is_enabled(Interrupt::SupervisorSoft));
        s.disable(Interrupt::SupervisorSoft);
        assert!(!s.is_enabled(Interrupt::SupervisorSoft));

        s.enable(Interrupt::SupervisorTimer);
        assert!(s.is_enabled(Interrupt::SupervisorTimer));
        s.disable(Interrupt::SupervisorTimer);
        assert!(!s.is_enabled(Interrupt::SupervisorTimer));

        s.enable(Interrupt::SupervisorExternal);
        assert!(s.is_enabled(Interrupt::SupervisorExternal));
        s.disable(Interrupt::SupervisorExternal);
        assert!(!s.is_enabled(Interrupt::SupervisorExternal));
    }
}
