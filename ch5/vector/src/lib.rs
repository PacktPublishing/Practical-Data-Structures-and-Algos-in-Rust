use std::alloc::{alloc, dealloc, realloc, Layout};
use std::mem::size_of;
use std::ptr::NonNull;

use drain::Drain;

mod deref;
mod drain;
mod iterator;

/// Custom `std::Vec<T>` remiplementation.
///
/// Safety:
/// The whole implementation assumes that:
/// * T is not a ZST
/// * If `cap` is non-zero, the `ptr` is a valid pointer to `cap` elements of type `T`
/// * First `len` elements of `ptr` are properly initialized, and not yet dropped
/// * `len <= cap`
#[derive(Debug)]
pub struct Vector<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

impl<T> Vector<T> {
    pub fn assert_zst() {
        if size_of::<T>() == 0 {
            panic!("ZSTs are not supported");
        }
    }

    pub fn new() -> Self {
        Self::assert_zst();

        Self {
            ptr: NonNull::dangling(),
            cap: 0,
            len: 0,
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self::assert_zst();

        let mut this = Self::new();
        this.grow_to(cap);
        this
    }

    fn grow_to(&mut self, newcap: usize) {
        match self.cap {
            0 if newcap > 0 => {
                self.cap = newcap;
                let layout = Layout::array::<T>(self.cap).unwrap();
                // Safety:
                // `newcap > 0`, so layout has more than zero elements, and `T` is not ZST
                let ptr = unsafe { alloc(layout) as _ };
                self.ptr = NonNull::new(ptr).unwrap();
            }
            cap if cap < newcap => {
                let layout = Layout::array::<T>(cap).unwrap();
                let newlayout = Layout::array::<T>(newcap).unwrap();
                // Safety:
                // `ptr` is allocated as an array of `self.cap` objects of type `T`, as for
                // calculated layout. The `newcap` is non-zero as `self.cap` is also non-zero.
                // `T` is checked to be not ZST
                let ptr = unsafe { realloc(self.ptr.as_ptr() as _, layout, newlayout.size()) as _ };
                self.ptr = NonNull::new(ptr).unwrap();
                self.cap = newcap;
            }
            _ => (),
        }
    }

    fn grow_for(&mut self, elements: usize) {
        let newlen = self.len.checked_add(elements).unwrap();
        if newlen > self.cap {
            let newcap = newlen.max(self.cap.checked_mul(2).unwrap());
            self.grow_to(newcap)
        }
    }

    pub fn shrink_to_fit(&mut self) {
        if self.cap == 0 {
            return;
        }

        let layout = Layout::array::<T>(self.cap).unwrap();
        match self.len {
            0 => unsafe { dealloc(self.ptr.as_ptr() as _, layout) },
            len => {
                let newlayout = Layout::array::<T>(len).unwrap();
                let ptr = unsafe { realloc(self.ptr.as_ptr() as _, layout, newlayout.size()) as _ };
                self.ptr = NonNull::new(ptr).unwrap();
            }
        }

        self.cap = self.len;
    }

    pub fn push(&mut self, item: T) {
        self.grow_for(1);
        unsafe { self.ptr.as_ptr().add(self.len).write(item) };
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.len {
            0 => None,
            _ => {
                self.len -= 1;
                unsafe { Some(self.ptr.as_ptr().add(self.len).read()) }
            }
        }
    }

    pub fn insert(&mut self, index: usize, element: T) {
        if index > self.len {
            panic!("Index out of bounds");
        }
        self.grow_for(1);
        unsafe {
            // Safety:
            // `ptr` is a pointer with `len` elements initialized and `cap` elements valid for
            // write, and due to `grow_for(1)`, `cap >= len + 1`.
            // `index <= len` by validation.
            // The copy source is `(ptr+index)..(ptr+index+len-index)`, that is
            // `ptr+index..ptr+len`, all are valid for read.
            // The copy destination is `(ptr+index+1)..(ptr+index+1+len-index)`, that is
            // `ptr+index+1..ptr+len+1`, all valid for write.
            //
            // After this call, the *(ptr+index) is moved from and never written back, it is
            // considered uninitialized temprarly. The *(ptr+len) element is written to, so
            // it is considered initialized since now.
            std::ptr::copy(
                self.ptr.as_ptr().add(index),
                self.ptr.as_ptr().add(index + 1),
                self.len - index,
            );
            // Safety:
            // The `*(ptr+index)` is valid for write, as `index <= len+1`
            //
            // This initializes `*(ptr+index)`, so all `len+1` elements are initialized again
            self.ptr.as_ptr().add(index).write(element);
        }
        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        if index >= self.len {
            panic!("Index out of bounds");
        }
        let item = unsafe { self.ptr.as_ptr().add(index).read() };
        unsafe {
            std::ptr::copy(
                self.ptr.as_ptr().add(index + 1),
                self.ptr.as_ptr().add(index),
                self.len - index - 1,
            );
        };
        self.len -= 1;
        item
    }

    pub fn swap_remove(&mut self, index: usize) -> T {
        let last_idx = self.len - 1;
        self.swap(index, last_idx);
        self.pop().unwrap()
    }

    pub fn into_boxed_slice(mut self) -> Box<[T]> {
        let boxed = unsafe { Box::from_raw(&mut self as &mut [T]) };
        std::mem::forget(self);
        boxed
    }

    pub fn drain(&mut self, range: impl std::ops::RangeBounds<usize>) -> Drain<'_, T> {
        Drain::new(self, range)
    }
}

impl<T> Default for Vector<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        let slice = unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) };
        unsafe { std::ptr::drop_in_place(slice) };

        if self.cap > 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            // Safety:
            // The layout for `ptr` is guaranted to be an array for `self.cap` alements of type `T`.
            // It is also checked `self.cap` is non-zero.
            unsafe {
                dealloc(self.ptr.as_ptr() as _, layout);
            }
        }
    }
}
