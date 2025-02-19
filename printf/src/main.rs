use std::ffi::c_char;

unsafe extern "C" {
    fn printf(format: *const c_char, ...);
}

fn main() {
    unsafe {
        printf(c"hello\nworld\n%d\n".as_ptr(), 42);
    }
}
