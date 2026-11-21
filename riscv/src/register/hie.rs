//! hie register

use crate::{
    bits::{bf_extract, bf_insert},
    CoreInterruptNumber,
};

read_write_csr! {
    /// `hie` register
    Hie: 0x604,
    mask: usize::MAX,
}

read_write_csr_field! {
    Hie,
    /// Virtual Supervisor Software Interrupt Enable
    vssie: 2,
}

read_write_csr_field! {
    Hie,
    /// Virtual Supervisor Timer Interrupt Enable
    vstie: 3,
}

read_write_csr_field! {
    Hie,
    /// Virtual Supervisor External Interrupt Enable
    vseie: 10,
}

read_write_csr_field! {
    Hie,
    /// Supervisor Guest External Interrupt Enable
    sgeie: 12,
}

impl Hie {
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

set!(0x604);
clear!(0x604);

set_clear_csr!(
    /// Virtual Supervisor Software Interrupt Enable
    , set_vssie, clear_vssie, 1 << 2);
set_clear_csr!(
    /// Virtual Supervisor Timer Interrupt Enable
    , set_vstie, clear_vstie, 1 << 3);
set_clear_csr!(
    /// Virtual Supervisor External Interrupt Enable
    , set_vseie, clear_vseie, 1 << 10);
set_clear_csr!(
    /// Supervisor Guest External Interrupt Enable
    , set_sgeie, clear_sgeie, 1 << 12);

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
    use crate::interrupt::machine::Interrupt;

    #[test]
    fn test_hie() {
        let mut h = Hie::from_bits(0);
        test_csr_field!(h, vssie);
        test_csr_field!(h, vstie);
        test_csr_field!(h, vseie);
        test_csr_field!(h, sgeie);
    }

    #[test]
    fn test_hie_interrupt() {
        let mut h = Hie::from_bits(0);

        h.enable(Interrupt::SupervisorSoft);
        assert!(h.is_enabled(Interrupt::SupervisorSoft));
        h.disable(Interrupt::SupervisorSoft);
        assert!(!h.is_enabled(Interrupt::SupervisorSoft));

        h.enable(Interrupt::MachineSoft);
        assert!(h.is_enabled(Interrupt::MachineSoft));
        h.disable(Interrupt::MachineSoft);
        assert!(!h.is_enabled(Interrupt::MachineSoft));

        h.enable(Interrupt::SupervisorTimer);
        assert!(h.is_enabled(Interrupt::SupervisorTimer));
        h.disable(Interrupt::SupervisorTimer);
        assert!(!h.is_enabled(Interrupt::SupervisorTimer));

        h.enable(Interrupt::MachineTimer);
        assert!(h.is_enabled(Interrupt::MachineTimer));
        h.disable(Interrupt::MachineTimer);
        assert!(!h.is_enabled(Interrupt::MachineTimer));

        h.enable(Interrupt::SupervisorExternal);
        assert!(h.is_enabled(Interrupt::SupervisorExternal));
        h.disable(Interrupt::SupervisorExternal);
        assert!(!h.is_enabled(Interrupt::SupervisorExternal));

        h.enable(Interrupt::MachineExternal);
        assert!(h.is_enabled(Interrupt::MachineExternal));
        h.disable(Interrupt::MachineExternal);
        assert!(!h.is_enabled(Interrupt::MachineExternal));
    }
}
