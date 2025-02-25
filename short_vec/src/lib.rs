use std::ffi::c_void;
use std::marker::PhantomData;
use std::ptr;

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
    _phantom: PhantomData<T>,
}

impl<T> ShortVec<T> {
    pub fn new() -> Self {
        ShortVec {
            inner: ffi::short_vec {
                buffer: [ptr::null_mut(); CAPACITY],
                length: 0,
            },
            _phantom: PhantomData,
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.inner.length == CAPACITY {
            return;
        }
        let elem_box = Box::new(elem);
        let elem_ptr = Box::leak(elem_box) as *mut T;
        unsafe {
            ffi::short_vec_push(&mut self.inner, elem_ptr as *mut c_void);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let elem_ptr = unsafe { ffi::short_vec_pop(&mut self.inner) as *mut T };
        if elem_ptr.is_null() {
            return None;
        }
        let elem_box = unsafe { Box::from_raw(elem_ptr) };
        Some(*elem_box)
    }

    pub fn take_all(&mut self, other: &mut Self) {
        unsafe {
            ffi::short_vec_take_all(&mut self.inner, &mut other.inner);
        }
    }
}

impl<T> Drop for ShortVec<T> {
    fn drop(&mut self) {
        while let Some(elem) = self.pop() {
            drop(elem);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut v = ShortVec::new();
        assert_eq!(v.inner.length, 0);

        // Fill the ShortVec.
        for i in 0..10 {
            v.push(i);
            assert_eq!(v.inner.length, i + 1);
        }

        // Trying to add more elements to a full ShortVec does nothing.
        assert_eq!(v.inner.length, CAPACITY);
        v.push(42);
        assert_eq!(v.inner.length, CAPACITY);

        // Pop all the elements back out.
        for i in (0..10).rev() {
            assert_eq!(v.pop(), Some(i));
            assert_eq!(v.inner.length, i);
        }

        // Popping an empty ShortVec returns None.
        assert_eq!(v.pop(), None);
    }

    #[test]
    fn test_take_all() {
        let mut v1 = ShortVec::new();
        v1.push(1);
        v1.push(2);
        v1.push(3);
        let mut v2 = ShortVec::new();
        v2.push(4);
        v2.push(5);
        v2.push(6);
        v1.take_all(&mut v2);
        assert_eq!(v1.pop(), Some(6));
        assert_eq!(v1.pop(), Some(5));
        assert_eq!(v1.pop(), Some(4));
        assert_eq!(v1.pop(), Some(3));
        assert_eq!(v1.pop(), Some(2));
        assert_eq!(v1.pop(), Some(1));
        assert_eq!(v1.pop(), None);
        assert_eq!(v2.pop(), None);
    }

    // #[test]
    // fn test_take_all_self() {
    //     let mut v = ShortVec::new();
    //     v.push(42);
    //     v.take_all(&mut v);
    // }

    #[test]
    fn test_take_all_self_ffi() {
        let mut v1 = ShortVec::new();
        v1.push(42);
        let v1_ptr = &mut v1.inner as *mut ffi::short_vec;
        unsafe {
            ffi::short_vec_take_all(v1_ptr, v1_ptr);
        }
        assert_eq!(v1.inner.length, 0, "That's kind of weird!");
    }
}
