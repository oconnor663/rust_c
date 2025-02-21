use std::ffi::c_int;

unsafe extern "C" {
    fn putchar(c: c_int) -> c_int;
}

fn main() {
    for &byte in b"hello world\n" {
        unsafe {
            putchar(byte as c_int);
        }
    }
}
