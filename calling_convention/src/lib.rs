#[repr(C)]
pub struct MyStruct {
    x: i32,
    y: i32,
    z: i32,
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn foo(my_struct: MyStruct) -> i32 {
    my_struct.x
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn bar() -> i32 {
    let my_struct = MyStruct { x: 1, y: 2, z: 3 };
    foo(my_struct)
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn foo_cabi(my_struct: MyStruct) -> i32 {
    my_struct.x
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn bar_cabi() -> i32 {
    let my_struct = MyStruct { x: 1, y: 2, z: 3 };
    foo_cabi(my_struct)
}
