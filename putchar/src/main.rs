use std::ffi::c_char;

unsafe extern "C" {
    fn putchar(c: c_char);
}

fn main() {
    for &byte in b"hello world\n" {
        unsafe {
            putchar(byte as c_char);
        }
    }
}
