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

pub struct LinkedList<T> {
    inner: ffi::linked_list,
    _phantom: PhantomData<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        let mut inner = MaybeUninit::uninit();
        unsafe {
            ffi::linked_list_init(inner.as_mut_ptr());
            Self {
                inner: inner.assume_init(),
                _phantom: PhantomData,
            }
        }
    }

    pub fn len(&self) -> usize {
        unsafe { ffi::linked_list_len(&self.inner) }
    }

    pub fn push(&mut self, elem: T) {
        let elem_box = Box::new(elem);
        let elem_ptr = Box::leak(elem_box) as *mut T;
        unsafe {
            ffi::linked_list_push(&mut self.inner, elem_ptr as *mut c_void);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let elem_ptr = unsafe { ffi::linked_list_pop(&mut self.inner) as *mut T };
        if elem_ptr.is_null() {
            return None;
        }
        let elem_box = unsafe { Box::from_raw(elem_ptr) };
        Some(*elem_box)
    }

    pub fn concat(&mut self, other: &mut Self) {
        unsafe {
            ffi::linked_list_concat(&mut self.inner, &mut other.inner);
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {
            // The Option<T> returned by .pop() is a "temporary" that gets dropped automatically.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut list = LinkedList::new();
        assert!(list.inner.head.is_null());
        assert!(list.inner.tail.is_null());

        for i in 0..10 {
            list.push(i);
            assert_eq!(list.len(), i + 1);
        }

        for i in (0..10).rev() {
            assert_eq!(list.pop(), Some(i));
            assert_eq!(list.len(), i);
        }

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_concat() {
        let mut list1 = LinkedList::new();
        list1.push(1);
        list1.push(2);
        list1.push(3);
        let mut list2 = LinkedList::new();
        list2.push(4);
        list2.push(5);
        list2.push(6);
        list1.concat(&mut list2);
        assert_eq!(list2.len(), 0);
        assert_eq!(list2.pop(), None);
        assert_eq!(list1.len(), 6);
        assert_eq!(list1.pop(), Some(6));
        assert_eq!(list1.pop(), Some(5));
        assert_eq!(list1.pop(), Some(4));
        assert_eq!(list1.pop(), Some(3));
        assert_eq!(list1.pop(), Some(2));
        assert_eq!(list1.pop(), Some(1));
        assert_eq!(list1.pop(), None);
        assert_eq!(list2.len(), 0);
    }

    // #[test]
    // fn test_concat_self() {
    //     let mut list = LinkedList::new();
    //     list.push(42);
    //     list.concat(&mut list);
    // }
}
