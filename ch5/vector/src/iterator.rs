use std::alloc::{dealloc, Layout};
use std::mem::ManuallyDrop;
use std::ptr::NonNull;

use crate::Vector;

pub struct IntoIter<T> {
    vec: ManuallyDrop<Vector<T>>,
    curr: NonNull<T>,
    end: *const T,
}

impl<T> IntoIter<T> {
    fn new(vec: Vector<T>) -> Self {
        Self {
            curr: vec.ptr,
            end: unsafe { vec.ptr.as_ptr().add(vec.len) },
            vec: ManuallyDrop::new(vec),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.as_ptr() as *const _ != self.end {
            let item = unsafe { self.curr.as_ptr().read() };
            self.curr = unsafe { NonNull::new(self.curr.as_ptr().add(1)).unwrap() };
            Some(item)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = unsafe { self.end.offset_from(self.curr.as_ptr()) as _ };
        (len, Some(len))
    }

    fn count(self) -> usize {
        self.size_hint().0
    }
}

impl<T> IntoIterator for Vector<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        let len = unsafe { self.end.offset_from(self.curr.as_ptr()) as usize };
        let slice = unsafe { std::slice::from_raw_parts_mut(self.curr.as_ptr(), len) };
        unsafe { std::ptr::drop_in_place(slice) }

        if self.vec.cap > 0 {
            let layout = Layout::array::<T>(self.vec.cap).unwrap();
            // Safety:
            // The layout for `ptr` is guaranted to be an array for `self.cap` alements of type `T`.
            // It is also checked `self.cap` is non-zero.
            unsafe {
                dealloc(self.vec.ptr.as_ptr() as _, layout);
            }
        }
    }
}

impl<A> FromIterator<A> for Vector<A> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = A>,
    {
        let iter = iter.into_iter();
        let (hint_low, hint_high) = iter.size_hint();
        let hint = hint_high.unwrap_or(hint_low);
        let mut this = Self::with_capacity(hint);

        for elem in iter {
            this.push(elem);
        }

        this
    }
}
