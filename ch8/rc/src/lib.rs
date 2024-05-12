use std::alloc::{alloc, dealloc, Layout};
use std::cell::Cell;
use std::mem::forget;
use std::ops::Deref;
use std::ptr::NonNull;

struct RcCnt {
    strong: Cell<usize>,
}

pub struct Rc<T> {
    counters: NonNull<RcCnt>,
    value: NonNull<T>,
}

impl<T> Rc<T> {
    fn layout() -> (Layout, usize) {
        let (layout, item_offset) = Layout::new::<RcCnt>().extend(Layout::new::<T>()).unwrap();
        (layout.pad_to_align(), item_offset)
    }

    pub fn new(value: T) -> Self {
        let counters = RcCnt {
            strong: 1.into(),
        };

        let (layout, value_offset) = Self::layout();

        let counters_ptr = unsafe { alloc(layout) };
        let value_ptr = unsafe { counters_ptr.add(value_offset) };

        let counters_ptr = counters_ptr as *mut RcCnt;
        let value_ptr = value_ptr as *mut T;

        unsafe { counters_ptr.write(counters) };
        unsafe { value_ptr.write(value) };

        Self {
            counters: NonNull::new(counters_ptr).unwrap(),
            value: NonNull::new(value_ptr).unwrap(),
        }
    }

    pub fn into_inner(this: Self) -> Option<T> {
        let counters = unsafe { this.counters.as_ref() };
        let strong = counters.strong.get();

        if strong > 1 {
            return None;
        }

        let value = unsafe { this.value.as_ptr().read() };

        let (layout, _) = Self::layout();
        unsafe { dealloc(this.counters.as_ptr() as _, layout) };

        forget(this);
        Some(value)
    }

    pub fn get_mut(this: &mut Self) -> Option<&mut T> {
        let counters = unsafe { this.counters.as_ref() };
        let strong = counters.strong.get();

        match strong {
            1 => Some(unsafe { this.value.as_mut() }),
            _ => None,
        }
    }

    pub fn make_mut(this: &mut Self) -> &mut T
    where
        T: Clone,
    {
        let counters = unsafe { this.counters.as_ref() };
        let strong = counters.strong.get();

        if strong > 1 {
            let mut copied = Rc::new(unsafe { this.value.as_ref().clone() });
            std::mem::swap(this, &mut copied);
        }

        unsafe { this.value.as_mut() }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let counters = unsafe { self.counters.as_ref() };
        let strong = counters.strong.get().checked_add(1).expect("Attempt to clone Rc, but too many references are alive");

        counters.strong.set(strong);

        Self {
            counters: self.counters,
            value: self.value,
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.value.as_ref() }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let counters = unsafe { self.counters.as_ref() };

        let strong = counters.strong.get() - 1;

        if strong == 0 {
            unsafe { self.value.as_ptr().drop_in_place() }

            let (layout, _) = Self::layout();
            unsafe { dealloc(self.counters.as_ptr() as _, layout) }
        } else {
            counters.strong.set(strong);
        }
    }
}

