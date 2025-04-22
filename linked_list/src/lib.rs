use std::ffi::c_void;
use std::marker::PhantomData;

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub struct LinkedList<T> {
    head: *mut ffi::node,
    phantom: PhantomData<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: std::ptr::null_mut(),
            phantom: PhantomData,
        }
    }

    pub fn push(&mut self, elem: T) {
        let elem_box = Box::new(elem);
        let elem_ptr = Box::into_raw(elem_box) as *mut c_void;
        unsafe {
            ffi::linked_list_push(&mut self.head, elem_ptr);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let elem_ptr = unsafe { ffi::linked_list_pop(&mut self.head) };
        if elem_ptr.is_null() {
            return None;
        }
        let elem_box = unsafe { Box::from_raw(elem_ptr as *mut T) };
        Some(*elem_box)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut v = LinkedList::new();
        v.push(1);
        v.push(2);
        v.push(3);
        assert_eq!(v.pop(), Some(3));
        assert_eq!(v.pop(), Some(2));
        assert_eq!(v.pop(), Some(1));
        assert_eq!(v.pop(), None);
    }

    // If we don't implement Drop properly, ASan will complain about a memory leak here.
    #[test]
    fn test_drop() {
        let mut v = LinkedList::new();
        v.push(1);
        v.push(2);
        v.push(3);
    }
}
