#[unsafe(no_mangle)]
extern "C" fn add_one(x: i32) -> i32 {
    x + 1
}

unsafe extern "C" {
    pub fn add_two(x: i32) -> i32;
}

fn main() {
    unsafe {
        println!("{}", add_two(40));
    }
}
