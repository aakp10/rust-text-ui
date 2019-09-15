#![no_std]
#![no_main]
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

    //https://wiki.osdev.org/Text_UI
    let greeting = b"Text UI";
    //version 1
    unsafe {
        let vga_buffer_ptr = (0xB8000 as *mut u8);

        for (index, byte) in greeting.iter().enumerate() { 
            *vga_buffer_ptr.offset(index as isize *2) = *byte;
            //foreground color
            *vga_buffer_ptr.offset(index as isize *2 + 1 ) = 0x0;
        }
    }

    // INDEXING RAW POINTERS ISNT ALLOWED
    /*unsafe {
        let vga_buffer_ptr = &mut (*(0xB8000 as *mut &mut[u8]));

        for (index, byte) in greeting.iter().enumerate() { 
            vga_buffer_ptr[index*2] = *byte;
            //foreground color
            vga_buffer_ptr[index*2 + 1] = 0x0e;
        }
    }*/
    loop {}
}
