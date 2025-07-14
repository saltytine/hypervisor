#![no_std] // rm standard library linking
#![no_main] // use the actual entry_start, not the main entry

use core::arch::global_asm;
use core::ptr; // inline asm support

mod panic;

global_asm!(include_str!("start.s")); // inline asm

// #[lang = "eh_personality"]
#[no_mangle] // make it so rust doesnt modify the function name we define during compilation
pub extern "C" fn not_main() {
    // see: https://en.wikipedia.org/wiki/Calling_convention
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"AARCH Bare Metal";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
}
