//! vsip register

use crate::{bits::bf_extract, CoreInterruptNumber};

read_only_csr! {
    /// vsip register
    Vsip: 0x244,
    mask: usize::MAX,
}

read_only_csr_field! {
    Vsip,
    /// Supervisor Software Interrupt Pending
    ssip: 1,
}

read_only_csr_field! {
    Vsip,
    /// Supervisor Timer Interrupt Pending
    stip: 5,
}

read_only_csr_field! {
    Vsip,
    /// Supervisor External Interrupt Pending
    seip: 9,
}

impl Vsip {
    /// Returns true when a given interrupt is pending.
    #[inline]
    pub fn is_pending<I: CoreInterruptNumber>(&self, interrupt: I) -> bool {
        bf_extract(self.bits, interrupt.number(), 1) != 0
    }
}

set!(0x244);
clear!(0x244);

set_clear_csr!(
    /// Supervisor Software Interrupt Pending
    , set_ssip, clear_ssip, 1 << 1);

/// Clear the pending state of a specific core interrupt source.
///
/// # Safety
///
/// Not all interrupt sources allow clearing of pending interrupts via the `vsip` register.
/// Instead, it may be necessary to perform an alternative action to clear the interrupt.
/// Check the specification of your specific core for details.
/// In `vsip` only SSIP is clearable.
#[inline]
pub unsafe fn clear_pending<I: CoreInterruptNumber>(interrupt: I) {
    _clear(1 << interrupt.number());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sip() {
        let sip = Vsip::from_bits(0);

        assert!(!sip.ssip());
        assert!(!sip.stip());
        assert!(!sip.seip());

        assert!(Vsip::from_bits(1 << 1).ssip());
        assert!(Vsip::from_bits(1 << 5).stip());
        assert!(Vsip::from_bits(1 << 9).seip());
    }
}
