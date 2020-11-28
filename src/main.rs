#![feature(llvm_asm)]

#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern fn main() {
}

/// A small busy loop.
fn small_delay() {
  for _ in 0..400000 {
    unsafe { llvm_asm!("" :::: "volatile")}
  }
}
