use std::ffi::c_void;
use std::marker::PhantomData;
use std::mem::MaybeUninit;

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub struct CVec<T> {
    inner: ffi::c_vec,
    _phantom: PhantomData<T>,
}

impl<T> CVec<T> {
    pub fn new() -> Self {
        let mut inner = MaybeUninit::uninit();
        unsafe {
            ffi::vec_init(inner.as_mut_ptr());
            Self {
                inner: inner.assume_init(),
                _phantom: PhantomData,
            }
        }
    }

    pub fn push(&mut self, elem: T) {
        let elem_box = Box::new(elem);
        let elem_ptr = Box::leak(elem_box) as *mut T;
        unsafe {
            ffi::vec_push(&mut self.inner, elem_ptr as *mut c_void);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let elem_ptr = unsafe { ffi::vec_pop(&mut self.inner) as *mut T };
        if elem_ptr.is_null() {
            return None;
        }
        let elem_box = unsafe { Box::from_raw(elem_ptr) };
        Some(*elem_box)
    }
}

impl<T> Drop for CVec<T> {
    fn drop(&mut self) {
        while let Some(elem) = self.pop() {
            drop(elem);
        }
        unsafe {
            ffi::vec_free(&mut self.inner);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut v = CVec::new();
        assert!(v.inner.buffer.is_null());
        assert_eq!(v.inner.length, 0);
        assert_eq!(v.inner.capacity, 0);

        for i in 0..100 {
            v.push(42);
            assert_eq!(v.inner.length, i + 1);
            assert_eq!(v.inner.capacity, (i + 1).next_power_of_two());
        }

        for i in 0..100 {
            assert_eq!(v.pop(), Some(42));
            assert_eq!(v.inner.length, 99 - i);
            assert_eq!(v.inner.capacity, 128);
        }

        assert_eq!(v.pop(), None);
    }
}
