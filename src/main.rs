#![no_std] // rm standard library linking
#![no_main] // use the actual entry_start, not the main entry

use core::arch::global_asm;
use core::ptr; // inline asm support

mod panic;

global_asm!(include_str!("start.s")); // inline asm

// #[lang = "eh_personality"]
#[no_mangle] // make it so rust doesnt modify the function name we define during compilation
pub extern "C" fn init() {
    // see: https://en.wikipedia.org/wiki/Calling_convention
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"AARCH Bare Metal\n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
}
#[no_mangle]
pub extern "C" fn el3_entry() -> u8 {
    printk_uart0("this is el3_entry.........\n")
}

#[no_mangle]
pub extern "C" fn el2_entry() -> u8 {
    printk_uart0("This is el2_entry.........\n")
}

fn printk_uart0(str: &str) -> u8 {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    for byte in str.bytes() {
        unsafe {
            ptr::write_volatile(UART0, byte);
        }
    }
    return 0;
}

pub fn boot_hypervisor() -> u8 {
    printk_uart0("Hello Hypervisor\n");
    /*
     * 1. configure related registers;
     * 2. configure page table information;
     * 3. other configurations;
     * 4. vcpu_init;
     * 5. ram_init;
     * 6. irq_init;
     * 7. load_image;
     * 8. vcpu_run;
     */
    return 0;
}
