//! hgatp register

use crate::result::{Error, Result};

read_write_csr! {
    /// `hgatp` register
    Hgatp: 0x680,
    mask: usize::MAX,
}

#[cfg(target_pointer_width = "32")]
csr_field_enum! {
    /// 32-bit hgatp mode
    Mode {
        default: Bare,
        /// No translation or protection
        Bare = 0,
        /// Page-based 34-bit virtual addressing
        Sv32x4 = 1,
    }
}

#[cfg(target_pointer_width = "64")]
csr_field_enum! {
    /// 64-bit hgatp mode
    Mode {
        default: Bare,
        /// No translation or protection
        Bare = 0,
        /// Page-based 41-bit virtual addressing
        Sv39x4 = 8,
        /// Page-based 50-bit virtual addressing
        Sv48x4 = 9,
        /// Page-based 59-bit virtual addressing
        Sv57x4 = 10,
    }
}

#[cfg(target_pointer_width = "32")]
read_write_csr_field! {
    Hgatp,
    /// Physical page number
    ppn: [0:21],
}

#[cfg(target_pointer_width = "64")]
read_write_csr_field! {
    Hgatp,
    /// Physical page number
    ppn: [0:43],
}

#[cfg(target_pointer_width = "32")]
read_write_csr_field! {
    Hgatp,
    /// Virtual machine identifier
    vmid: [22:28],
}

#[cfg(target_pointer_width = "64")]
read_write_csr_field! {
    Hgatp,
    /// Virtual machine identifier
    vmid: [44:57],
}

#[cfg(target_pointer_width = "32")]
read_write_csr_field! {
    Hgatp,
    /// Current address-translation scheme.
    mode,
    Mode: [31:31],
}

#[cfg(target_pointer_width = "64")]
read_write_csr_field! {
    Hgatp,
    /// Current address-translation scheme.
    mode,
    Mode: [60:63],
}

/// Sets the register to corresponding page table mode, physical page number and address space id.
///
/// **WARNING**: panics on:
///
/// - non-`riscv` targets
/// - invalid field values
#[inline]
#[cfg(target_pointer_width = "32")]
pub unsafe fn set(mode: Mode, vmid: usize, ppn: usize) {
    try_set(mode, vmid, ppn).unwrap();
}

/// Attempts to set the register to corresponding page table mode, physical page number and address space id.
#[inline]
#[cfg(target_pointer_width = "32")]
pub unsafe fn try_set(mode: Mode, vmid: usize, ppn: usize) -> Result<()> {
    if vmid != vmid & 0x7F {
        Err(Error::InvalidFieldValue {
            field: "vmid",
            value: vmid,
            bitmask: 0x7F,
        })
    } else if ppn != ppn & 0x3F_FFFF {
        Err(Error::InvalidFieldValue {
            field: "ppn",
            value: ppn,
            bitmask: 0x3F_FFFF,
        })
    } else {
        let bits = (mode as usize) << 31 | (vmid << 22) | ppn;
        _try_write(bits)
    }
}

/// Sets the register to corresponding page table mode, physical page number and address space id.
///
/// **WARNING**: panics on:
///
/// - non-`riscv` targets
/// - invalid field values
#[inline]
#[cfg(target_pointer_width = "64")]
pub unsafe fn set(mode: Mode, vmid: usize, ppn: usize) {
    try_set(mode, vmid, ppn).unwrap()
}

/// Attempts to set the register to corresponding page table mode, physical page number and address space id.
#[inline]
#[cfg(target_pointer_width = "64")]
pub unsafe fn try_set(mode: Mode, vmid: usize, ppn: usize) -> Result<()> {
    if vmid != vmid & 0x3FFF {
        Err(Error::InvalidFieldValue {
            field: "vmid",
            value: vmid,
            bitmask: 0x3FFF,
        })
    } else if ppn != ppn & 0xFFF_FFFF_FFFF {
        Err(Error::InvalidFieldValue {
            field: "ppn",
            value: ppn,
            bitmask: 0xFFF_FFFF_FFFF,
        })
    } else {
        let bits = ((mode as usize) << 60) | (vmid << 44) | ppn;
        _try_write(bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_pointer_width = "32")]
    const VMID_START: usize = 22;
    #[cfg(target_pointer_width = "32")]
    const VMID_END_INCLUSIVE: usize = 29;
    #[cfg(target_pointer_width = "64")]
    const VMID_START: usize = 44;
    #[cfg(target_pointer_width = "64")]
    const VMID_END_INCLUSIVE: usize = 58;

    #[cfg(target_pointer_width = "32")]
    const MODES: [Mode; 2] = [Mode::Bare, Mode::Sv32x2];
    #[cfg(target_pointer_width = "64")]
    const MODES: [Mode; 4] = [Mode::Bare, Mode::Sv39x4, Mode::Sv48x4, Mode::Sv57x4];

    #[test]
    fn test_hgatp() {
        let new_mode = Mode::new();

        (1..=usize::BITS)
            .map(|r| ((1u128 << r) - 1) as usize)
            .for_each(|raw| {
                let mut hgatp = Hgatp::from_bits(raw);

                let exp_ppn = raw & ((1usize << VMID_START) - 1);
                let exp_vmid = (raw & ((1usize << VMID_END_INCLUSIVE) - 1)) >> VMID_START;

                assert_eq!(hgatp.ppn(), exp_ppn);

                hgatp.set_ppn(0);
                assert_eq!(hgatp.ppn(), 0);

                hgatp.set_ppn(exp_ppn);
                assert_eq!(hgatp.ppn(), exp_ppn);

                assert_eq!(hgatp.vmid(), exp_vmid);

                hgatp.set_vmid(0);
                assert_eq!(hgatp.vmid(), 0);

                hgatp.set_vmid(exp_vmid);
                assert_eq!(hgatp.vmid(), exp_vmid);

                match Mode::from_usize(raw >> 60) {
                    Ok(exp_mode) => {
                        assert_eq!(hgatp.try_mode(), Ok(exp_mode));
                        assert_eq!(hgatp.mode(), exp_mode);

                        hgatp.set_mode(new_mode);

                        assert_eq!(hgatp.try_mode(), Ok(new_mode));
                        assert_eq!(hgatp.mode(), new_mode);

                        hgatp.set_mode(exp_mode);

                        assert_eq!(hgatp.try_mode(), Ok(exp_mode));
                        assert_eq!(hgatp.mode(), exp_mode);
                    }
                    Err(exp_err) => {
                        assert_eq!(hgatp.try_mode(), Err(exp_err));
                    }
                }
            });

        let mut hgatp = Hgatp::from_bits(0);
        MODES
            .into_iter()
            .for_each(|mode| test_csr_field!(hgatp, mode: mode));
    }
}
