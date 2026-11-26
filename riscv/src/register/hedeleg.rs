//! hedeleg register

read_write_csr! {
    /// `hedeleg` register
    Hedeleg: 0x602,
    mask: 0x000_CB1FF,
}

read_write_csr_field! {
    Hedeleg,
    /// Instruction Address Misaligned Delegate
    instruction_misaligned: 0,
}

read_write_csr_field! {
    Hedeleg,
    /// Instruction Access Fault Delegate
    instruction_fault: 1,
}

read_write_csr_field! {
    Hedeleg,
    /// Illegal Instruction Delegate
    illegal_instruction: 2,
}

read_write_csr_field! {
    Hedeleg,
    /// Breakpoint Delegate
    breakpoint: 3,
}

read_write_csr_field! {
    Hedeleg,
    /// Load Address Misaligned Delegate
    load_misaligned: 4,
}

read_write_csr_field! {
    Hedeleg,
    /// Load Access Fault Delegate
    load_fault: 5,
}

read_write_csr_field! {
    Hedeleg,
    /// Store/AMO Address Misaligned Delegate
    store_misaligned: 6,
}

read_write_csr_field! {
    Hedeleg,
    /// Store/AMO Access Fault Delegate
    store_fault: 7,
}

read_write_csr_field! {
    Hedeleg,
    /// Environment Call from U-mode Delegate
    user_env_call: 8,
}

read_write_csr_field! {
    Hedeleg,
    /// Instruction Page Fault Delegate
    instruction_page_fault: 12,
}

read_write_csr_field! {
    Hedeleg,
    /// Load Page Fault Delegate
    load_page_fault: 13,
}

read_write_csr_field! {
    Hedeleg,
    /// Store/AMO Page Fault Delegate
    store_page_fault: 15,
}

read_write_csr_field! {
    Hedeleg,
    /// Software Check
    software_check_fault: 18,
}

read_write_csr_field! {
    Hedeleg,
    /// Hardware Error
    hardware_error_fault: 19,
}

set!(0x602);
clear!(0x602);

set_clear_csr!(
    /// Instruction Address Misaligned Delegate
    , set_instruction_misaligned, clear_instruction_misaligned, 1 << 0);
set_clear_csr!(
    /// Instruction Access Fault Delegate
    , set_instruction_fault, clear_instruction_fault, 1 << 1);
set_clear_csr!(
    /// Illegal Instruction Delegate
    , set_illegal_instruction, clear_illegal_instruction, 1 << 2);
set_clear_csr!(
    /// Breakpoint Delegate
    , set_breakpoint, clear_breakpoint, 1 << 3);
set_clear_csr!(
    /// Load Address Misaligned Delegate
    , set_load_misaligned, clear_load_misaligned, 1 << 4);
set_clear_csr!(
    /// Load Access Fault Delegate
    , set_load_fault, clear_load_fault, 1 << 5);
set_clear_csr!(
    /// Store/AMO Address Misaligned Delegate
    , set_store_misaligned, clear_store_misaligned, 1 << 6);
set_clear_csr!(
    /// Store/AMO Access fault
    , set_store_fault, clear_store_fault, 1 << 7);
set_clear_csr!(
    /// Environment Call from U-mode Delegate
    , set_user_env_call, clear_user_env_call, 1 << 8);
set_clear_csr!(
    /// Instruction Page Fault Delegate
    , set_instruction_page_fault, clear_instruction_page_fault, 1 << 12);
set_clear_csr!(
    /// Load Page Fault Delegate
    , set_load_page_fault, clear_load_page_fault, 1 << 13);
set_clear_csr!(
    /// Store/AMO Page Fault Delegate
    , set_store_page_fault, clear_store_page_fault, 1 << 15);
set_clear_csr!(
    /// Software Check
    , set_software_check, clear_software_check, 1 << 18);
set_clear_csr!(
    /// Hardware Error
    , set_hardware_error, clear_hardware_error, 1 << 19);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hedeleg() {
        let mut h = Hedeleg::from_bits(0);

        test_csr_field!(h, instruction_misaligned);
        test_csr_field!(h, instruction_fault);
        test_csr_field!(h, illegal_instruction);
        test_csr_field!(h, breakpoint);
        test_csr_field!(h, load_misaligned);
        test_csr_field!(h, load_fault);
        test_csr_field!(h, store_misaligned);
        test_csr_field!(h, store_fault);
        test_csr_field!(h, user_env_call);
        test_csr_field!(h, instruction_page_fault);
        test_csr_field!(h, load_page_fault);
        test_csr_field!(h, store_page_fault);
    }
}
