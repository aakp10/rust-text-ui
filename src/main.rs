#![no_std]
#![no_main]
mod vga_buffer;
use vga_buffer::*;
use core::panic::PanicInfo;
//https://doc.rust-lang.org/nomicon/panic-handler.html
// being a no_std application it doesn't have linkage to std funcs.
//implement panic handler for panic! macro
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}



//avoid name mangling in case of the `_start` function name
#[no_mangle]
//foreign function interface; use C style calling convention
pub extern "C" fn _start() -> ! {
    let greeting = "Text UI";
    VGA_WRITER.lock().write_string(greeting);
    loop {}
}
