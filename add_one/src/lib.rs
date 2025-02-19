unsafe extern "C" {
    pub fn add_one(x: i32) -> i32;
}

#[test]
fn test_add_one() {
    unsafe {
        assert_eq!(add_one(42), 43);
    }
}
