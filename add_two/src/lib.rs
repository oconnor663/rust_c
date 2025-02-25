#[unsafe(no_mangle)]
extern "C" fn add_one(x: i32) -> i32 {
    x + 1
}

unsafe extern "C" {
    pub fn add_two(x: i32) -> i32;
}

#[test]
fn test_add_two() {
    unsafe {
        assert_eq!(add_two(42), 44);
    }
}
