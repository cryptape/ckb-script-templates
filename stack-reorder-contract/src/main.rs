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
    let mut _x: u64;
    unsafe {
        core::arch::asm!(
            "mv {_x}, sp",
            _x = out(reg) _x,
        );
    }

    ckb_std::debug!("Current SP is {:x}", _x);

    0
}
