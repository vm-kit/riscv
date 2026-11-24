//! vstval register

read_csr_as_usize!(0x243);
write_csr!(0x243);

/// Writes the CSR
#[inline]
pub unsafe fn write(bits: usize) {
    _write(bits)
}
