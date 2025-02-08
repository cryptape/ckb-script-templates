#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

#[cfg(not(test))]
ckb_std::entry!(program_entry);
// By default, the following heap configuration is used:
// * 16KB fixed heap
// * 1.2MB(rounded up to be 16-byte aligned) dynamic heap
// * Minimal memory block in dynamic heap is 64 bytes
// For more details, please refer to ckb-std's default_alloc macro
// and the buddy-alloc alloc implementation.
ckb_std::default_alloc!(16384, 1258306, 64);

#[allow(unused_variables, unused_assignments)]
pub fn program_entry() -> i8 {
    let mut x: u64;
    unsafe {
        core::arch::asm!(
            "mv {x}, sp",
            x = out(reg) x,
        );
    }

    ckb_std::debug!("Current SP is {:x}", x);

    0
}
