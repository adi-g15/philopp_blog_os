#![no_std]

pub fn vga_print(s: &str) {
    let vga_buffer = 0xb8000 as *mut u8;

    let bytes = s.as_bytes();

    // @adi Learned - This .enumerate is a great thing here in rust,
    //                ie. instead of just 'for range' on for(int i=0; ....)
    //                it provides the iter.enumerate(), now we can iterate
    //                with (index,value) pairs, which provides both good things
    for (i,&byte) in bytes.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize*2) = byte;     // char
            *vga_buffer.offset(i as isize*2 + 1) = 0xb;  // bg+fg+blink bits
        }
    }
}

