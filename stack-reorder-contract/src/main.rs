#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

#[cfg(not(test))]
use ckb_std::default_alloc;
#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();

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
