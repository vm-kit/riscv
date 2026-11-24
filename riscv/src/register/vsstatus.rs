//! vsstatus register
//! TODO:: check mask

pub use super::misa::XLEN;
pub use super::mstatus::{FS, VS, XS};

#[cfg(target_arch = "riscv32")]
const MASK: usize = 0x800d_e762;
#[cfg(not(target_arch = "riscv32"))]
const MASK: usize = 0x8000_0003_000d_e762;

read_write_csr! {
    /// Virtual Supervisor Status Register
    Vsstatus: 0x200,
    mask: MASK,
}

csr_field_enum! {
    /// Virtual Supervisor Previous Privilege Mode
    SPP {
        default: VirtualUser,
        /// Previous privilege mode is Virtual User mode.
        VirtualUser = 0,
        /// Previous privilege mode is Virtual Supervisor mode.
        VirtualSupervisor = 1,
    }
}

read_write_csr_field! {
    Vsstatus,
    /// Virtual Supervisor Interrupt Enable
    sie: 1,
}

read_write_csr_field! {
    Vsstatus,
    /// Virtual Supervisor Previous Interrupt Enable
    spie: 5,
}

read_write_csr_field! {
    Vsstatus,
    /// VU-mode non-instruction-fetch memory endianness
    ube: 6,
}

read_write_csr_field! {
    Vsstatus,
    /// Virtual Supervisor Previous Privilege Mode
    spp,
    SPP: [8:8],
}

read_write_csr_field! {
    Vsstatus,
    /// Vector extension state
    vs,
    VS: [9:10],
}

read_write_csr_field! {
    Vsstatus,
    /// The status of the floating-point unit
    fs,
    FS: [13:14],
}

read_only_csr_field! {
    Vsstatus,
    /// The status of additional user-mode extensions
    /// and associated state
    xs,
    XS: [15:16],
}

read_write_csr_field! {
    Vsstatus,
    /// Permit Supervisor User Memory access
    sum: 18,
}

read_write_csr_field! {
    Vsstatus,
    /// Make Executable Readable
    mxr: 19,
}

#[cfg(not(target_arch = "riscv32"))]
read_write_csr_field! {
    Vsstatus,
    /// Effective xlen in VU-mode (i.e., `UXLEN`).
    uxl,
    XLEN: [32:33],
}

#[cfg(target_arch = "riscv32")]
read_only_csr_field! {
    Vsstatus,
    /// Whether either the FS field or XS field signals the presence of some dirty state.
    ///
    /// This is a read-only bit computed by hardware as `(FS == Dirty) || (XS == Dirty) || (VS == Dirty)`.
    sd: 31,
}

#[cfg(not(target_arch = "riscv32"))]
read_only_csr_field! {
    Vsstatus,
    /// Whether either the FS field or XS field signals the presence of some dirty state.
    ///
    /// This is a read-only bit computed by hardware as `(FS == Dirty) || (XS == Dirty) || (VS == Dirty)`.
    sd: 63,
}

impl Vsstatus {
    /// Effective xlen in VU-mode (i.e., `UXLEN`).
    ///
    /// In RISCV-32, UXL does not exist, and `UXLEN` is always [`XLEN::XLEN32`].
    #[inline]
    #[cfg(target_arch = "riscv32")]
    pub fn uxl(&self) -> XLEN {
        XLEN::XLEN32
    }
}

set!(0x200);
clear!(0x200);

set_clear_csr!(
    /// Virtual Supervisor Interrupt Enable
    , set_sie, clear_sie, 1 << 1);
set_csr!(
    /// Virtual Supervisor Previous Interrupt Enable
    , set_spie, 1 << 5);
set_clear_csr!(
    /// VU-mode non-instruction-fetch memory endianness
    , set_ube, clear_ube, 1 << 6);
set_clear_csr!(
    /// Permit Virtual Supervisor User Memory access
    , set_sum, clear_sum, 1 << 18);
set_clear_csr!(
    /// Make eXecutable Readable
    , set_mxr, clear_mxr, 1 << 19);

/// Virtual Supervisor Previous Privilege Mode
#[inline]
pub unsafe fn set_spp(spp: SPP) {
    match spp {
        SPP::VirtualSupervisor => _set(1 << 8),
        SPP::VirtualUser => _clear(1 << 8),
    }
}

/// The status of the floating-point unit
#[inline]
pub unsafe fn set_fs(fs: FS) {
    let mut value = _read();
    value &= !(0x3 << 13); // clear previous value
    value |= (fs as usize) << 13;
    _write(value);
}

/// Vector extension state
#[inline]
pub unsafe fn set_vs(vs: VS) {
    let mut value = _read();
    value &= !(0x3 << 9); // clear previous value
    value |= (vs as usize) << 9;
    _write(value);
}

/// Effective xlen in U-mode (i.e., `UXLEN`)
///
/// # Note
///
/// In RISCV-32, `UXL` does not exist.
#[inline]
#[cfg(not(target_arch = "riscv32"))]
pub unsafe fn set_uxl(uxl: XLEN) {
    let mut value = _read();
    value &= !(0x3 << 32); // clear previous value
    value |= (uxl as usize) << 32;
    _write(value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vsstatus() {
        let mut vsstatus = Vsstatus::from_bits(0);

        test_csr_field!(vsstatus, sie);
        test_csr_field!(vsstatus, spie);
        test_csr_field!(vsstatus, ube);

        [SPP::VirtualUser, SPP::VirtualSupervisor].into_iter().for_each(|spp| {
            test_csr_field!(vsstatus, spp: spp);
        });

        [VS::Off, VS::Initial, VS::Clean, VS::Dirty]
            .into_iter()
            .for_each(|vs| {
                test_csr_field!(vsstatus, vs: vs);
            });

        [FS::Off, FS::Initial, FS::Clean, FS::Dirty]
            .into_iter()
            .for_each(|fs| {
                test_csr_field!(vsstatus, fs: fs);
            });

        [
            XS::AllOff,
            XS::NoneDirtyOrClean,
            XS::NoneDirtySomeClean,
            XS::SomeDirty,
        ]
        .into_iter()
        .for_each(|xs| {
            let vsstatus = Vsstatus::from_bits(xs.into_usize() << 15);
            assert_eq!(vsstatus.xs(), xs);
            assert_eq!(vsstatus.try_xs(), Ok(xs));
        });

        test_csr_field!(vsstatus, sum);
        test_csr_field!(vsstatus, mxr);

        [XLEN::XLEN32, XLEN::XLEN64, XLEN::XLEN128]
            .into_iter()
            .for_each(|xlen| {
                test_csr_field!(vsstatus, uxl: xlen);
            });

        // SD is read-only: hardware sets it whenever FS, VS, or XS == Dirty.
        assert!(!Vsstatus::from_bits(0).sd());
        assert!(Vsstatus::from_bits(usize::MAX).sd());
    }
}