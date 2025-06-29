//! hstatus register

pub use super::misa::XLEN;
#[cfg(not(target_arch = "riscv32"))]
use crate::bits::{bf_extract, bf_insert};

#[cfg(not(target_arch = "riscv32"))]
read_write_csr! {
    /// Hypervisor Status Register (hstatus)
    Hstatus: 0x600,
    mask: 0x0006_0006_0073_F3E0,
}

#[cfg(target_arch = "riscv32")]
read_write_csr! {
    /// Hypervisor Status Register (hstatus)
    Hstatus: 0x300,
    mask: 0x0073_F3E0,
}

set!(0x600);
clear!(0x600);

csr_field_enum! {
    /// VSXL field for VS-mode XLEN.
    VSXL {
        default: VSXLEN32,
        VSXLEN32 = 1,
        VSXLEN64 = 2,
    }
}

csr_field_enum! {
    /// Endianness for VS-mode
    VSBE {
        default: LittleEndian,
        LittleEndian = 0,
        BigEndian = 1,
    }
}

csr_field_enum! {
    /// Virtual address attribute for stval
    GVA {
        default: NoAddressProvided,
        NoAddressProvided = 0,
        AddressTrap = 1,
    }
}

csr_field_enum! {
    /// Supervisor Previous Virtualization mode
    SPV {
        default: VSModeOff,
        VSModeOff = 0,
        VSModeOn = 1,
    }
}

csr_field_enum! {
    /// Supervisor Previous Virtual Privilege mode
    SPVP {
        default: User,
        User = 0,
        Supervisor = 1,
    }
}

read_write_csr_field! {
    Hstatus,
    /// Trap SRET in VS-mode
    vtsr: 22,
}
set_clear_csr!(
    /// Trap SRET in VS-mode
    , set_vtsr, clear_vtsr, 1 << 22);


read_write_csr_field! {
    Hstatus,
    /// Trap WFI in VS-mode
    vtw: 21,
}
set_clear_csr!(
    /// Trap WFI in VS-mode
    , set_vtw, clear_vtw, 1 << 21);

read_write_csr_field! {
    Hstatus,
    /// Trap VMA instructions or CSR satp in VS-mode
    vtvm: 20,
}
set_clear_csr!(
    /// Trap VMA instructions or CSR satp in VS-mode
    , set_vtvm, clear_vtvm, 1 << 20);

read_write_csr_field! {
    Hstatus,
    /// VM Load/Store Guest Mode User Privilege
    hu: 9,
}

read_write_csr_field! {
    Hstatus,
    /// Supervisor Previous Virtualization mode
    spv,
    SPV: [7:7],
}

/// Supervisor Previous Virtualization Mode
/// todo:: ask about this
#[inline]
pub unsafe fn set_spv(spv: SPV) {
    match spv {
        SPV::VSModeOn => _set(1 << 7),
        SPV::VSModeOff => _clear(1 << 7),
    }
}

read_write_csr_field! {
    Hstatus,
    /// Supervisor Previous Virtual Privilege
    spvp,
    SPVP: [8:8],
}

#[cfg(not(target_arch = "riscv32"))]
read_write_csr_field! {
    Hstatus,
    /// Effective XLEN in VS-mode
    vsxl,
    VSXL: [32:33],
}

#[cfg(not(target_arch = "riscv32"))]
read_write_csr_field! {
    Hstatus,
    /// Virtual Guest External Interrupt Number
    vgein: [12:17],
}

impl Hstatus {
    //// Set VGEIN
//    #[inline]
//    pub fn set_vgein(&mut self, val: usize) {
//      self.bits = bf_insert(self.bits, 12, 6, val);
//    }
//
//    /// Get VSXLEN field
//    #[inline]
//    pub fn get_vgein(&self) -> usize {
//      #[cfg(target_arch = "riscv32")]
//      {
//          bf_extract(self.bits, 12, 6)
//      }
//
//      #[cfg(not(target_arch = "riscv32"))]
//      {
//        bf_extract(self.bits, 12, 6)
//      }
//    }
//    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hstatus_fields() {
        let mut hstatus = Hstatus { bits: 0 };

      #[cfg(target_arch = "riscv32")]
      {
        test_csr_field!(hstatus, vsxl: VSXL::VSXLEN32);
      }

      #[cfg(not(target_arch = "riscv32"))]
      {
        test_csr_field!(hstatus, vsxl: VSXL::VSXLEN64);
        test_csr_field!(hstatus, vsxl: VSXL::VSXLEN32);
      }

        test_csr_field!(hstatus, vtsr);
        test_csr_field!(hstatus, vtw);
        test_csr_field!(hstatus, vtvm);
        test_csr_field!(hstatus, hu);
        test_csr_field!(hstatus, vgein);
        test_csr_field!(hstatus, spv: SPV::VSModeOff);
        test_csr_field!(hstatus, spv: SPV::VSModeOn);
        test_csr_field!(hstatus, spvp: SPVP::User);
        test_csr_field!(hstatus, spvp: SPVP::Supervisor);

    }
}
