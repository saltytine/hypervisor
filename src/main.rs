#![no_std]
#![no_main]
#![feature(naked_functions)] //  surpport naked function
use core::arch::global_asm;
use core::result::Result;
#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64/mod.rs"]
mod arch;
mod consts;
mod header;
mod memory;
mod panic;

fn main() -> Result<(), ()> {
    Ok(())
}

extern "C" fn entry() -> () {
    if let Err(_e) = main() {}
}
