//! hideleg register

read_write_csr! {
    /// `hideleg` register
    Hideleg: 0x603,
    mask: 0x0222,
}

read_write_csr_field! {
    Hideleg,
    /// Virtual Supervisor Software Interrupt Delegate
    vssi: 2,
}

read_write_csr_field! {
    Hideleg,
    /// Virtual Supervisor Timer Interrupt Delegate
    vsti: 6,
}

read_write_csr_field! {
    Hideleg,
    /// Virtual Supervisor External Interrupt Delegate
    vsei: 10,
}

set!(0x603);
clear!(0x603);

set_clear_csr!(
    /// Virtual Supervisor Software Interrupt Delegate
    , set_vssi, clear_vssi, 1 << 2);
set_clear_csr!(
    /// Virtual Supervisor Timer Interrupt Delegate
    , set_vsti, clear_vsti, 1 << 5);
set_clear_csr!(
    /// Virtual Supervisor External Interrupt Delegate
    , set_vsei, clear_vsei, 1 << 9);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hideleg() {
        let mut h = Hideleg::from_bits(0);

        test_csr_field!(h, vssi);
        test_csr_field!(h, vsti);
        test_csr_field!(h, vsei);
    }
}
