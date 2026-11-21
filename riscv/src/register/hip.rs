//! hip register

use crate::{bits::bf_extract, CoreInterruptNumber};

read_only_csr! {
    /// hip register
    Hip: 0x644,
    mask: usize::MAX,
}

read_only_csr_field! {
    Hip,
    /// Virtual Supervisor Software Interrupt Pending
    vssip: 2,
}

read_only_csr_field! {
    Hip,
    /// Virtual Supervisor Timer Interrupt Pending
    vstip: 6,
}

read_only_csr_field! {
    Hip,
    /// Virtual Supervisor External Interrupt Pending
    vseip: 10,
}

read_only_csr_field! {
    Hip,
    /// Supervisor Guest External Interrupt Pending
    sgeip: 12,
}

impl Hip {
    /// Returns true when a given interrupt is pending.
    #[inline]
    pub fn is_pending<I: CoreInterruptNumber>(&self, interrupt: I) -> bool {
        bf_extract(self.bits, interrupt.number(), 1) != 0
    }
}

set!(0x144);
clear!(0x144);

set_clear_csr!(
    /// Virtual Supervisor Software Interrupt Pending
    , set_vssip, clear_vssip, 1 << 1);

/// Clear the pending state of a specific core interrupt source.
///
/// # Safety
///
/// Not all interrupt sources allow clearing of pending interrupts via the `hip` register.
/// Instead, it may be necessary to perform an alternative action to clear the interrupt.
/// Check the specification of your specific core for details.
#[inline]
pub unsafe fn clear_pending<I: CoreInterruptNumber>(interrupt: I) {
    _clear(1 << interrupt.number());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hip() {
        let hip = Hip::from_bits(0);

        assert!(!hip.vssip());
        assert!(!hip.vstip());
        assert!(!hip.vseip());
        assert!(!hip.sgeip());

        assert!(Hip::from_bits(1 << 6).vstip());
        assert!(Hip::from_bits(1 << 10).vseip());
    }
}
