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
        default: WasVModeOff,
        WasVModeOff = 0,
        WasVModeOn = 1,
    }
}

csr_field_enum! {
    /// Supervisor Previous Virtual Privilege mode
    SPVP {
        default: WasVUMode,
        WasVUMode = 0,
        WasVSMode = 1,
    }
}

csr_field_enum! {
    /// Hypervisor U-mode memory access enable
    HU {
        default: NoAccess,
        NoAccess = 0,
        Access = 1,
    }
}

read_write_csr_field! {
    Hstatus,
    /// Trap SRET in VS-mode
    vtsr: 22,
}

read_write_csr_field! {
    Hstatus,
    /// Trap WFI in VS-mode
    vtw: 21,
}

read_write_csr_field! {
    Hstatus,
    /// Trap VMA instructions or CSR satp in VS-mode
    vtvm: 20,
}

read_write_csr_field! {
    Hstatus,
    /// VM Load/Store Guest Mode User Privilege
    hu,
    HU: 9,
}

read_write_csr_field! {
    Hstatus,
    /// Supervisor Previous Virtualization mode
    spv,
    SPV: [7,7],
}

read_write_csr_field! {
    Hstatus,
    /// Supervisor Previous Virtual Privilege
    spvp,
    SPVP: 8,
}

#[cfg(not(target_arch = "riscv32"))]
read_write_csr_field! {
    Hstatus,
    /// Effective XLEN in VS-mode
    vsxl: [32:33],
}

impl Hstatus {
    /// Get VSXLEN field
    #[inline]
    pub fn vsxl(&self) -> XLEN {
        #[cfg(target_arch = "riscv32")]
        {
            XLEN::XLEN32
        }

        #[cfg(not(target_arch = "riscv32"))]
        {
            XLEN::try_from(bf_extract(self.bits, 32, 2)).unwrap_or_default()
        }
    }

    /// Set VSXLEN field
    #[inline]
    #[cfg(not(target_arch = "riscv32"))]
    pub fn set_vsxl(&mut self, xlen: XLEN) {
        self.bits = bf_insert(self.bits, 32, 2, xlen as usize);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hstatus_fields() {
        let mut hstatus = Hstatus { bits: 0 };

        #[cfg(not(target_arch = "riscv32"))]
        {
            test_csr_field!(hstatus, vsxl: XLEN::XLEN32);
            test_csr_field!(hstatus, vsxl: XLEN::XLEN64);
        }

        test_csr_field!(hstatus, vtsr);
        test_csr_field!(hstatus, vtw);
        test_csr_field!(hstatus, vtvm);
        test_csr_field!(hstatus, hu: HU::NoAccess);
        test_csr_field!(hstatus, hu: HU::Access);
        test_csr_field!(hstatus, spv: SPV::WasVModeOff);
        test_csr_field!(hstatus, spv: SPV::WasVModeOn);
        test_csr_field!(hstatus, spvp: SPVP::WasVUMode);
        test_csr_field!(hstatus, spvp: SPVP::WasVSMode);
    }
}
