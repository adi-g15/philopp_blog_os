#![no_std]
// we don't want to link with standard library
//
// Errors then, and after fixing them:
// 1. #[panic_handler] not found
// 2. eh_personality language item not found (used internally by compiler for
//    stack unwinding)
//    - Disable unwinding itself (add panic="abort" in Cargo.toml)
// 3. requires 'start' language item
//    - No point in adding that, since it will still require crt0... so instead
//    we ovewrite the crt0 entry point directly

#![no_main] // overwrite entry point, tell Rust we don't want normal entrypoint
            // chain

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {  // the function returns 'never' type ('!')
    // This is marked as 'diverging function' (never returns)
    loop{};
}

/* Generally for a rust binary, execution stats in C runtime (called `crt0`) */


// 'extern C' also tells to use the C calling convention for this function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

