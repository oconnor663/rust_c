use std::ffi::{c_int, c_void};

unsafe extern "C" {
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
}

fn main() {
    let bytes = b"hello world\n";
    unsafe {
        write(1, bytes.as_ptr().cast(), bytes.len());
    }
}
