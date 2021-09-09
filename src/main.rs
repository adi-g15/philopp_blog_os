#![no_std]
#![no_main]

use core::panic::PanicInfo;
extern crate vga_print;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {  // the function returns 'never' type ('!')
    // This is marked as 'diverging function' (never returns)
    loop{};
}

// 'extern C' also tells to use the C calling convention for this function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Shouldn't it require 'unsafe' block... or atleast show a warning
    vga_print::vga_print("Namaste :)");

    loop {}
}

