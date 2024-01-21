use std::ptr::NonNull;

use crate::Vector;

pub struct Drain<'vec, T> {
    vec: &'vec mut Vector<T>,
    start: NonNull<T>,
    curr: NonNull<T>,
    end: *const T,
}

impl<'vec, T> Drain<'vec, T> {
    pub(crate) fn new(vec: &'vec mut Vector<T>, range: impl std::ops::RangeBounds<usize>) -> Self {
        use std::ops::Bound;

        let start = match range.start_bound() {
            Bound::Included(start) => *start,
            Bound::Excluded(start) => *start + 1,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(end) => *end + 1,
            Bound::Excluded(end) => *end,
            Bound::Unbounded => vec.len(),
        };
        let end = end.min(vec.len());

        let start = unsafe { NonNull::new(vec.ptr.as_ptr().add(start)).unwrap() };
        let end = unsafe { vec.ptr.as_ptr().add(end) };

        Self {
            start,
            end,
            curr: start,
            vec,
        }
    }
}

impl<T> Iterator for Drain<'_, T> {
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

impl<T> Drop for Drain<'_, T> {
    fn drop(&mut self) {
        let len = unsafe { self.end.offset_from(self.curr.as_ptr()) as usize };
        let slice = unsafe { std::slice::from_raw_parts_mut(self.curr.as_ptr(), len) };
        unsafe { std::ptr::drop_in_place(slice) }

        let end = unsafe { self.vec.as_ptr().add(self.vec.len) };
        let tail_len = unsafe { end.offset_from(self.end) } as usize;
        unsafe { std::ptr::copy(self.end, self.start.as_ptr(), tail_len) };

        self.vec.len -= len
    }
}
