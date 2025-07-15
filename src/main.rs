#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(default_alloc_error_handler)]
use core::arch::global_asm;
use core::result::Result;
#[macro_use]
extern crate alloc;
#[macro_use]
extern crate buddy_system_allocator;
#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64/mod.rs"]
mod arch;
mod consts;
mod error;
mod header;
mod memory;
mod panic;
mod percpu;
use error::HvResult;
use percpu::PerCpu;

fn primary_init_early() -> HvResult {
    memory::init_heap();
    Ok(())
}
fn main(cpuid: u32, cpu_data: &mut PerCpu) -> HvResult {
    let is_primary = cpuid == 0;
    if is_primary {
        primary_init_early()?;
    }
    cpu_data.activate_vmm()
}
fn arch_handle_exit() -> Result<(), ()> {
    Ok(())
}
extern "C" fn entry(cpuid: u32, cpu_data: &mut PerCpu) -> () {
    if let Err(_e) = main(cpuid, cpu_data) {}
}
