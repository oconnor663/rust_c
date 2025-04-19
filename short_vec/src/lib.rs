use std::ffi::c_void;
use std::marker::PhantomData;

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

const CAPACITY: usize = ffi::CAPACITY as usize;

pub struct ShortVec<T> {
    inner: ffi::short_vec,
    phantom: PhantomData<T>,
}

impl<T> ShortVec<T> {
    pub fn new() -> Self {
        ShortVec {
            inner: ffi::short_vec {
                buffer: [std::ptr::null_mut(); CAPACITY],
                length: 0,
            },
            phantom: PhantomData,
        }
    }

    pub fn push(&mut self, elem: T) {
        assert!(self.inner.length < CAPACITY);
        let elem_box = Box::new(elem);
        let elem_ptr = Box::into_raw(elem_box) as *mut c_void;
        unsafe {
            ffi::short_vec_push(&mut self.inner, elem_ptr);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let elem_ptr = unsafe { ffi::short_vec_pop(&mut self.inner) };
        if elem_ptr.is_null() {
            return None;
        }
        let elem_box = unsafe { Box::from_raw(elem_ptr as *mut T) };
        Some(*elem_box)
    }
}

impl<T> Drop for ShortVec<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut v = ShortVec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        assert_eq!(v.pop(), Some(3));
        assert_eq!(v.pop(), Some(2));
        assert_eq!(v.pop(), Some(1));
        assert_eq!(v.pop(), None);
    }

    #[test]
    #[should_panic]
    fn test_over_capacity() {
        let mut v = ShortVec::new();
        for i in 0..CAPACITY + 1 {
            v.push(i);
        }
    }

    // If we don't implement Drop properly, ASan will complain about a memory leak here.
    #[test]
    fn test_drop() {
        let mut v = ShortVec::new();
        v.push(1);
        v.push(2);
        v.push(3);
    }
}
