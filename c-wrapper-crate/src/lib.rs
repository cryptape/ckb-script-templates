#![cfg_attr(not(test), no_std)]

#[cfg(target_arch = "riscv64")]
#[link(name = "c-impl", kind = "static")]
extern "C" {
    fn bar() -> core::ffi::c_int;
}

pub fn value() -> u32 {
    (unsafe { bar() }) as u32
}
