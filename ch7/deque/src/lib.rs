use std::mem::{transmute, MaybeUninit};
use std::ptr::drop_in_place;

const MIN_SHIFT: usize = 3;

mod iterator;

// Safety:
//
// * `head < buffer.len()`
// * `len <= buffer.len()`
//
// If `head + len <= buffer.len()` buffer doesnt wrap, and all the items between
// `buffer[head..head+len]` are always initialized, and all the other items are not.
//
// If `head + len >= buffer.len()`, that mans that the buffer wraps - so `buffer[head..]` and
// `buffer[..head + len - buffer.len()]` are always initialized, and all the other items are not.`
//
// The `buffer.len()` is always a power of 2, so `& (buffer.len() - 1)` can be used to calculate
// the modulo length.
pub struct Deque<T> {
    buffer: Vec<MaybeUninit<T>>,
    head: usize,
    len: usize,
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Deque {
            buffer: vec![],
            head: 0,
            len: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let shift = Self::make_shift(0, capacity);
        let buffer = std::iter::repeat_with(MaybeUninit::uninit)
            .take(shift)
            .collect();

        Self {
            buffer,
            head: 0,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn grow_to(&mut self, capacity: usize) {
        let len = self.buffer.len();
        if capacity <= len {
            return;
        }

        let newshift = Self::make_shift(self.buffer.len(), capacity);
        self.buffer.resize_with(newshift, MaybeUninit::uninit);

        // Wrapping case
        if self.head + self.len > len {
            let newlen = self.buffer.len();
            let head_len = len - self.head;
            let tail_len = self.head + self.len - len;

            if head_len <= tail_len {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        &self.buffer[self.head] as _,
                        &mut self.buffer[newlen - head_len] as _,
                        head_len,
                    )
                };
                self.head = newlen - head_len;
            } else {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        &self.buffer[0] as _,
                        &mut self.buffer[len] as _,
                        tail_len,
                    )
                };
            }
        }
    }

    pub fn push_back(&mut self, value: T) {
        self.grow_to(self.len() + 1);
        let mask = self.buffer.len() - 1;
        self.buffer[(self.head + self.len) & mask].write(value);
        self.len += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if !self.is_empty() {
            self.len -= 1;
            let mask = self.buffer.len() - 1;
            let item = unsafe { self.buffer[(self.head + self.len) & mask].assume_init_read() };
            Some(item)
        } else {
            None
        }
    }

    pub fn push_front(&mut self, value: T) {
        self.grow_to(self.len() + 1);
        self.head = self.head.wrapping_sub(1) & (self.buffer.len() - 1);
        self.buffer[self.head].write(value);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if !self.is_empty() {
            let item = unsafe { self.buffer[self.head].assume_init_read() };
            self.head = (self.head + 1) & (self.buffer.len() - 1);
            self.len -= 1;
            Some(item)
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            let mask = self.buffer.len() - 1;
            let item = unsafe { self.buffer[(index + self.head) & mask].assume_init_ref() };
            Some(item)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&T> {
        if index < self.len {
            let mask = self.buffer.len() - 1;
            let item = unsafe { self.buffer[(index + self.head) & mask].assume_init_mut() };
            Some(item)
        } else {
            None
        }
    }

    pub fn as_slices(&self) -> (&[T], &[T]) {
        if self.head + self.len <= self.buffer.len() {
            let slice = &self.buffer[self.head..self.head + self.len];
            let slice: &[T] = unsafe { transmute(slice) };
            (slice, &[])
        } else {
            let fst = &self.buffer[self.head..];
            let fst: &[T] = unsafe { transmute(fst) };

            let lst = &self.buffer[..self.head + self.len - self.buffer.len()];
            let lst: &[T] = unsafe { transmute(lst) };

            (fst, lst)
        }
    }

    pub fn as_slices_mut(&mut self) -> (&mut [T], &mut [T]) {
        if self.head + self.len <= self.buffer.len() {
            let slice = &mut self.buffer[self.head..self.head + self.len];
            let slice: &mut [T] = unsafe { transmute(slice) };
            (slice, &mut [])
        } else {
            let len = self.buffer.len();

            let (lst, fst) = self.buffer.split_at_mut(self.head);
            let fst: &mut [T] = unsafe { transmute(fst) };

            let lst = &mut lst[..self.head + self.len - len];
            let lst: &mut [T] = unsafe { transmute(lst) };

            (fst, lst)
        }
    }

    pub fn make_contiguous(&mut self) -> &mut [T] {
        if self.head + self.len < self.buffer.len() {
            let slice = &mut self.buffer[self.head..self.head + self.len];
            let slice: &mut [T] = unsafe { transmute(slice) };
            return slice;
        }

        let space = self.buffer.len() - self.len();
        let head_len = self.buffer.len() - self.head;
        let tail_len = self.len + self.head - self.buffer.len();

        if space >= head_len {
            // There is a space to copy the head
            unsafe {
                std::ptr::copy(
                    &self.buffer[0] as *const _,
                    &mut self.buffer[head_len] as *mut _,
                    tail_len,
                );
            }

            unsafe {
                std::ptr::copy_nonoverlapping(
                    &self.buffer[self.head] as *const _,
                    &mut self.buffer[0] as *mut _,
                    head_len,
                );
            }

            self.head = 0;
        } else if space >= tail_len {
            // There is a space to copy the tail
            unsafe {
                std::ptr::copy(
                    &self.buffer[self.head] as *const _,
                    &mut self.buffer[tail_len] as *mut _,
                    head_len,
                );
            }

            unsafe {
                std::ptr::copy_nonoverlapping(
                    &self.buffer[0] as *const _,
                    &mut self.buffer[head_len + tail_len] as *mut _,
                    tail_len,
                )
            }

            self.head = tail_len;
        } else if head_len <= tail_len {
            // cannot fit all the items, and the "head" is shorter than the "tail" - copying head
            // to glue it with tail, and then rotating right to fix the order
            if space != 0 {
                unsafe {
                    std::ptr::copy(
                        &self.buffer[self.head] as *const _,
                        &mut self.buffer[tail_len] as *mut _,
                        head_len,
                    );
                }
            }

            self.head = 0;
            self.buffer[0..self.len].rotate_right(head_len);
        } else {
            // cannot fit all the items, and the "tail" is shorter than the "head" - copying tail
            // to glue it with head, and then rotating left to fix the order
            if space != 0 {
                unsafe {
                    std::ptr::copy(
                        &self.buffer[0] as *const _,
                        &mut self.buffer[space] as *mut _,
                        tail_len,
                    );
                }
            }

            self.head = space;
            self.buffer[self.head..self.head + self.len].rotate_left(tail_len);
        }

        let slice = &mut self.buffer[self.head..self.head + self.len];
        let slice: &mut [T] = unsafe { transmute(slice) };
        slice
    }

    pub fn iter(&self) -> iterator::Iter<T> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> iterator::IterMut<T> {
        self.into_iter()
    }

    fn make_shift(base: usize, capacity: usize) -> usize {
        std::iter::successors(Some(base.max(1 << MIN_SHIFT)), |shift| Some(shift << 1))
            .take_while(|shift| *shift != 0)
            .find(|shift| *shift >= capacity)
            .unwrap()
    }
}

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for Deque<T> {
    fn drop(&mut self) {
        let len = self.buffer.len();

        if self.head + self.len <= len {
            let slice = &mut self.buffer[self.head..self.head + self.len];
            let slice: &mut [T] = unsafe { transmute(slice) };
            unsafe { drop_in_place(slice) };
        } else {
            let slice = &mut self.buffer[..self.head + self.len - len];
            let slice: &mut [T] = unsafe { transmute(slice) };
            unsafe { drop_in_place(slice) };

            let slice = &mut self.buffer[self.head..];
            let slice: &mut [T] = unsafe { transmute(slice) };
            unsafe { drop_in_place(slice) };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        // No capacity
        let deque: Deque<u32> = Deque::new();
        assert_eq!(deque.len(), 0);

        let items: Vec<_> = deque.into_iter().collect();
        assert_eq!(items, vec![]);

        // With capacity
        let deque: Deque<u32> = Deque::with_capacity(100);
        assert_eq!(deque.len(), 0);

        let items: Vec<_> = deque.into_iter().collect();
        assert_eq!(items, vec![]);
    }

    #[test]
    fn push_back() {
        // No capacity
        let mut deque: Deque<u32> = Deque::new();

        for i in 0..100 {
            deque.push_back(i);
        }

        assert_eq!(deque.len(), 100);

        let items: Vec<_> = deque.into_iter().collect();
        let expected: Vec<_> = (0..100).collect();
        assert_eq!(items, expected);

        // Not enough capacity
        let mut deque: Deque<u32> = Deque::with_capacity(50);

        for i in 0..100 {
            deque.push_back(i);
        }

        assert_eq!(deque.len(), 100);

        let items: Vec<_> = deque.into_iter().collect();
        let expected: Vec<_> = (0..100).collect();
        assert_eq!(items, expected);

        // Enough capacity
        let mut deque: Deque<u32> = Deque::with_capacity(50);

        for i in 0..100 {
            deque.push_back(i);
        }

        assert_eq!(deque.len(), 100);

        let items: Vec<_> = deque.into_iter().collect();
        let expected: Vec<_> = (0..100).collect();
        assert_eq!(items, expected);
    }

    #[test]
    fn pop_back() {
        let mut deque: Deque<u32> = Deque::new();
        assert_eq!(deque.pop_back(), None);

        let mut deque: Deque<u32> = Deque::with_capacity(100);
        assert_eq!(deque.pop_back(), None);

        let mut deque: Deque<_> = (0u32..100).collect();
        for i in 0..100 {
            assert_eq!(deque.pop_back(), Some(99 - i));
        }

        assert_eq!(deque.pop_back(), None);
    }

    #[test]
    fn push_front() {
        // No capacity
        let mut deque: Deque<u32> = Deque::new();

        for i in 0..100 {
            deque.push_front(99 - i);
        }

        assert_eq!(deque.len(), 100);

        let items: Vec<_> = deque.into_iter().collect();
        let expected: Vec<_> = (0..100).collect();
        assert_eq!(items, expected);

        // Not enough capacity
        let mut deque: Deque<u32> = Deque::with_capacity(50);

        for i in 0..100 {
            deque.push_front(99 - i);
        }

        assert_eq!(deque.len(), 100);

        let items: Vec<_> = deque.into_iter().collect();
        let expected: Vec<_> = (0..100).collect();
        assert_eq!(items, expected);

        // Enough capacity
        let mut deque: Deque<u32> = Deque::with_capacity(50);

        for i in 0..100 {
            deque.push_front(99 - i);
        }

        assert_eq!(deque.len(), 100);

        let items: Vec<_> = deque.into_iter().collect();
        let expected: Vec<_> = (0..100).collect();
        assert_eq!(items, expected);
    }

    #[test]
    fn pop_front() {
        let mut deque: Deque<u32> = Deque::new();
        assert_eq!(deque.pop_front(), None);

        let mut deque: Deque<u32> = Deque::with_capacity(100);
        assert_eq!(deque.pop_front(), None);

        let mut deque: Deque<_> = (0u32..100).collect();
        for i in 0..100 {
            assert_eq!(deque.pop_front(), Some(i));
        }

        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn push_back_pop_back() {
        let mut deque: Deque<u32> = Deque::new();
        for i in 0..100 {
            deque.push_back(i * 2);
            deque.push_back(i * 2 + 1);
            assert_eq!(deque.pop_back(), Some(i * 2 + 1))
        }

        let items: Vec<_> = deque.into_iter().collect();
        let expected: Vec<_> = (0..100).map(|i| i * 2).collect();
        assert_eq!(items, expected);
    }

    #[test]
    fn push_front_pop_front() {
        let mut deque: Deque<u32> = Deque::new();
        for i in 0..100 {
            deque.push_front(i * 2);
            deque.push_front(i * 2 + 1);
            assert_eq!(deque.pop_front(), Some(i * 2 + 1));
        }

        let items: Vec<_> = deque.into_iter().collect();
        let expected: Vec<_> = (0..100).map(|i| i * 2).rev().collect();
        assert_eq!(items, expected);
    }

    #[test]
    fn push_back_pop_front() {
        let mut deque: Deque<u32> = Deque::new();
        for i in 0..100 {
            deque.push_back(i * 2);
            deque.push_back(i * 2 + 1);
            assert_eq!(deque.pop_front(), Some(i));
        }

        let items: Vec<_> = deque.into_iter().collect();
        let expected: Vec<_> = (100..200).collect();
        assert_eq!(items, expected);
    }

    #[test]
    fn push_front_pop_back() {
        let mut deque: Deque<u32> = Deque::new();
        for i in 0..100 {
            deque.push_front(i * 2);
            deque.push_front(i * 2 + 1);
            assert_eq!(deque.pop_back(), Some(i));
        }

        let items: Vec<_> = deque.into_iter().collect();
        let expected: Vec<_> = (100..200).rev().collect();
        assert_eq!(items, expected);
    }

    #[test]
    fn as_slices() {
        let mut deque: Deque<u32> = Deque::new();

        // Push back only - everything should be continous
        for i in 0..100 {
            deque.push_back(i);
        }

        let (head, tail) = deque.as_slices();
        let expected: Vec<_> = (0..100).collect();
        assert_eq!(head, &expected);
        assert_eq!(tail, &[]);

        // removing element should not change it
        for _ in 0..20 {
            deque.pop_back();
            deque.pop_front();
        }

        let (head, tail) = deque.as_slices();
        let expected: Vec<_> = (20..80).collect();
        assert_eq!(head, &expected);
        assert_eq!(tail, &[]);

        let mut deque: Deque<u32> = Deque::with_capacity(100);

        // Push front only - everything should be continous (there is enough capacity)
        for i in 0..100 {
            deque.push_front(i);
        }

        let (head, tail) = deque.as_slices();
        let expected: Vec<_> = (0..100).rev().collect();
        assert_eq!(head, &expected);
        assert_eq!(tail, &[]);

        // removing element should not change it
        for _ in 0..20 {
            deque.pop_back();
            deque.pop_front();
        }

        let (head, tail) = deque.as_slices();
        let expected: Vec<_> = (20..80).rev().collect();
        assert_eq!(head, &expected);
        assert_eq!(tail, &[]);

        let mut deque: Deque<u32> = Deque::with_capacity(100);

        // Push front and back, not filling capacity - slices should be split
        for i in 0..20 {
            deque.push_back(i);
            deque.push_front(100 + i);
        }

        let (head, tail) = deque.as_slices();
        let expected_head: Vec<_> = (100..120).rev().collect();
        let expected_tail: Vec<_> = (0..20).collect();
        assert_eq!(head, &expected_head);
        assert_eq!(tail, &expected_tail);

        // Removing part of elements, still expecting split
        for _ in 0..5 {
            deque.pop_back();
            deque.pop_front();
        }

        let (head, tail) = deque.as_slices();
        let expected_head: Vec<_> = (100..115).rev().collect();
        let expected_tail: Vec<_> = (0..15).collect();
        assert_eq!(head, &expected_head);
        assert_eq!(tail, &expected_tail);

        // Removing whole tail
        for _ in 0..15 {
            deque.pop_back();
        }

        let (head, tail) = deque.as_slices();
        let expected_head: Vec<_> = (100..115).rev().collect();
        assert_eq!(head, &expected_head);
        assert_eq!(tail, &[]);
    }

    #[test]
    fn as_slices_mut() {
        let mut deque: Deque<u32> = Deque::new();

        // Push back only - everything should be continous
        for i in 0..100 {
            deque.push_back(i);
        }

        let (head, tail) = deque.as_slices_mut();
        let expected: Vec<_> = (0..100).collect();
        assert_eq!(head, &expected);
        assert_eq!(tail, &[]);

        for item in head {
            *item += 1;
        }

        // removing element should not change it
        for _ in 0..20 {
            deque.pop_back();
            deque.pop_front();
        }

        let (head, tail) = deque.as_slices_mut();
        let expected: Vec<_> = (21..81).collect();
        assert_eq!(head, &expected);
        assert_eq!(tail, &[]);

        for item in head {
            *item += 1;
        }

        let items: Vec<_> = deque.into_iter().collect();
        let expected: Vec<_> = (22..82).collect();
        assert_eq!(items, expected);

        let mut deque: Deque<u32> = Deque::with_capacity(100);

        // Push front only - everything should be continous (there is enough capacity)
        for i in 0..100 {
            deque.push_front(i);
        }

        let (head, tail) = deque.as_slices_mut();
        let expected: Vec<_> = (0..100).rev().collect();
        assert_eq!(head, &expected);
        assert_eq!(tail, &[]);

        for item in head {
            *item += 1;
        }

        // removing element should not change it
        for _ in 0..20 {
            deque.pop_back();
            deque.pop_front();
        }

        let (head, tail) = deque.as_slices_mut();
        let expected: Vec<_> = (21..81).rev().collect();
        assert_eq!(head, &expected);
        assert_eq!(tail, &[]);

        for item in head {
            *item += 1;
        }

        let items: Vec<_> = deque.into_iter().collect();
        let expected: Vec<_> = (22..82).rev().collect();
        assert_eq!(items, expected);

        let mut deque: Deque<u32> = Deque::with_capacity(100);

        // Push front and back, not filling capacity - slices should be split
        for i in 0..20 {
            deque.push_back(i);
            deque.push_front(100 + i);
        }

        let (head, tail) = deque.as_slices_mut();
        let expected_head: Vec<_> = (100..120).rev().collect();
        let expected_tail: Vec<_> = (0..20).collect();
        assert_eq!(head, &expected_head);
        assert_eq!(tail, &expected_tail);

        for item in head {
            *item += 1;
        }

        for item in tail {
            *item += 2;
        }

        // Removing part of elements, still expecting split
        for _ in 0..5 {
            deque.pop_back();
            deque.pop_front();
        }

        let (head, tail) = deque.as_slices_mut();
        let expected_head: Vec<_> = (101..116).rev().collect();
        let expected_tail: Vec<_> = (2..17).collect();
        assert_eq!(head, &expected_head);
        assert_eq!(tail, &expected_tail);

        for item in head {
            *item += 1;
        }

        for item in tail {
            *item += 2;
        }

        // Removing whole tail
        for _ in 0..15 {
            deque.pop_back();
        }

        let (head, tail) = deque.as_slices_mut();
        let expected_head: Vec<_> = (102..117).rev().collect();
        assert_eq!(head, &expected_head);
        assert_eq!(tail, &[]);

        for item in head {
            *item += 1;
        }

        let items: Vec<_> = deque.into_iter().collect();
        let expected: Vec<_> = (103..118).rev().collect();
        assert_eq!(items, expected);
    }

    #[test]
    fn make_contiguous() {
        // Initially contigous
        let mut deque: Deque<u32> = Deque::new();

        for i in 0..100 {
            deque.push_back(i);
        }

        let expected: Vec<_> = (0..100).collect();
        assert_eq!(deque.make_contiguous(), &expected);
        let (head, tail) = deque.as_slices();
        assert_eq!(head, &expected);
        assert_eq!(tail, &[]);

        // Space for head, no space for tail
        let mut deque: Deque<u32> = Deque::with_capacity(32);

        for i in 0..16 {
            deque.push_back(i);
        }

        for i in 100..102 {
            deque.push_front(i);
        }

        let expected_head: Vec<_> = (100..102).rev().collect();
        let expected_tail: Vec<_> = (0..16).collect();
        let (head, tail) = deque.as_slices();
        assert_eq!(head, &expected_head);
        assert_eq!(tail, &expected_tail);

        let expected: Vec<_> = [expected_head, expected_tail].concat();
        assert_eq!(deque.make_contiguous(), &expected);
        let (head, tail) = deque.as_slices();
        assert_eq!(head, &expected);
        assert_eq!(tail, &[]);

        // Space for tail, no space for head
        let mut deque: Deque<u32> = Deque::with_capacity(32);

        for i in 0..2 {
            deque.push_back(i);
        }

        for i in 100..116 {
            deque.push_front(i);
        }

        let expected_head: Vec<_> = (100..116).rev().collect();
        let expected_tail: Vec<_> = (0..2).collect();
        let (head, tail) = deque.as_slices();
        assert_eq!(head, &expected_head);
        assert_eq!(tail, &expected_tail);

        let expected: Vec<_> = [expected_head, expected_tail].concat();
        assert_eq!(deque.make_contiguous(), &expected);
        let (head, tail) = deque.as_slices();
        assert_eq!(head, &expected);
        assert_eq!(tail, &[]);

        // No space for one-shot copy, head < tail
        let mut deque: Deque<u32> = Deque::with_capacity(32);

        for i in 0..16 {
            deque.push_back(i);
        }

        for i in 100..110 {
            deque.push_front(i);
        }

        let expected_head: Vec<_> = (100..110).rev().collect();
        let expected_tail: Vec<_> = (0..16).collect();
        let (head, tail) = deque.as_slices();
        assert_eq!(head, &expected_head);
        assert_eq!(tail, &expected_tail);

        let expected: Vec<_> = [expected_head, expected_tail].concat();
        assert_eq!(deque.make_contiguous(), &expected);
        let (head, tail) = deque.as_slices();
        assert_eq!(head, &expected);
        assert_eq!(tail, &[]);

        // No space for one-shot copy, tail < head
        let mut deque: Deque<u32> = Deque::with_capacity(32);

        for i in 0..10 {
            deque.push_back(i);
        }

        for i in 100..116 {
            deque.push_front(i);
        }

        let expected_head: Vec<_> = (100..116).rev().collect();
        let expected_tail: Vec<_> = (0..10).collect();
        let (head, tail) = deque.as_slices();
        assert_eq!(head, &expected_head);
        assert_eq!(tail, &expected_tail);

        let expected: Vec<_> = [expected_head, expected_tail].concat();
        assert_eq!(deque.make_contiguous(), &expected);
        let (head, tail) = deque.as_slices();
        assert_eq!(head, &expected);
        assert_eq!(tail, &[]);
    }
}
