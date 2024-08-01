#![cfg_attr(not(feature = "simulator"), no_std)]
#![cfg_attr(not(any(feature = "simulator", test)), no_main)]

#[cfg(any(feature = "simulator", test))]
extern crate alloc;

#[cfg(not(any(feature = "simulator", test)))]
use ckb_std::default_alloc;
#[cfg(not(any(feature = "simulator", test)))]
ckb_std::entry!(program_entry);
#[cfg(not(any(feature = "simulator", test)))]
default_alloc!();

pub fn program_entry() -> i8 {
    ckb_std::debug!("This is a sample contract!");

    0
}
