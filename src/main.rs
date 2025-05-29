#![no_std]
#![no_main]
use core::panic::PanicInfo;

mod vga_buffer;
/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// static test: &[u8] = b"testrustos!\n";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("hello{}", "!");
    panic!("oh no");
    
    /* 
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("test").unwrap();
    write!(vga_buffer::WRITER.lock(), "some numb {} {}", 55, 1.2351).unwrap();


     vga_buffer::print_something();


     let vga_buffer = 0xb8000 as *mut u8;
    
    for (i, &byte) in test.iter().enumerate() {
        unsafe {
            // Write the character to the VGA buffer
            *vga_buffer.offset(i as isize * 2) = byte;
            // Write the attribute byte (white on black)
            *vga_buffer.offset(i as isize * 2 + 1) = 0x07;
        }
    }*/

    loop{} 

}



