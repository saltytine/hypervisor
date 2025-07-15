#![no_std] // disable standard library linking
#![no_main] // use the actual entry_start that we define, not main

use core::arch::global_asm; // inline asm support

mod driver;
mod lib;
mod panic;
global_asm!(include_str!("/arch/aarch64/start.s")); // inline asm

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::lib::_print(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[no_mangle]
pub extern "C" fn init(cpu_id: usize) {
    println!("Welcome AArch64 Bare Metal Hypervisor\n");
    boot_hypervisor(cpu_id);
}

pub fn boot_hypervisor(cpu_id: usize) {
    println!("Hello Hypervisor...\n");
    /* 原始方案：(deprecated)
     * 1. configure related registers；
     * 2. configure page table info；
     * 3. other congigurations；
     * 4. vcpu_init;
     * 5. ram_init;
     * 6. irq_init;
     * 7. load_image;
     * 8. vcpu_run;
     */

    /*
     * 1. check if its core_0
     * 2.
     */
    // printk_uart0(usize);
    println!("cpu_id: {}", cpu_id);
    if cpu_id == 0 {
        println!("Welcome to RVM hypervisor...\n");
        // heap::init();
        // mem_init();
    }
    loop {}
}
