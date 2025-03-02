use std::ffi::{c_int, c_void};

unsafe extern "C" {
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
}

fn main() {
    let buf = b"Hello, world!\n";
    let ret = unsafe { write(1, buf.as_ptr() as *const c_void, buf.len()) };
    assert_eq!(ret, buf.len() as isize);
}
