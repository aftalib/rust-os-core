#![no_std]
#![no_main]
use core::panic::PanicInfo;

mod vga_buffer;
/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello, world!\n";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
    /*  let vga_buffer = 0xb8000 as *mut u8;
    
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // Write the character to the VGA buffer
            *vga_buffer.offset(i as isize * 2) = byte;
            // Write the attribute byte (white on black)
            *vga_buffer.offset(i as isize * 2 + 1) = 0x07;
        }
    }*/

    loop{} 

}



