fn main() {
    for &byte in b"hello world\n" {
        unsafe {
            libc::putchar(byte.into());
        }
    }
}
