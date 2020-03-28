#![crate_type = "staticlib"]
#![no_std]

#[no_mangle]
pub fn duplicate(a: u32) -> u32 {
    return a * 2;
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
