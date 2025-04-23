fn main() {
    let bytes = b"hello world\n";
    unsafe {
        libc::write(libc::STDOUT_FILENO, bytes.as_ptr().cast(), bytes.len());
    }
}
