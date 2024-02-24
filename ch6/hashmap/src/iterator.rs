use std::{
    alloc::{dealloc, Layout},
    hash::Hash,
    marker::PhantomData,
    mem::ManuallyDrop,
    ptr::NonNull,
};

use crate::{Cell, HashMap};

pub struct IntoIter<K, V> {
    map: ManuallyDrop<HashMap<K, V>>,
    curr: NonNull<Cell<K, V>>,
    end: *const Cell<K, V>,
}

impl<K, V> IntoIter<K, V> {
    fn new(map: HashMap<K, V>) -> Self {
        Self {
            curr: map.ptr,
            end: unsafe { map.ptr.as_ptr().add(map.alloc_size) },
            map: ManuallyDrop::new(map),
        }
    }
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        while self.curr.as_ptr() as *const _ != self.end {
            let curr = self.curr;
            self.curr = unsafe { NonNull::new(self.curr.as_ptr().add(1)).unwrap() };

            if let Cell::Item { key, value } = unsafe { &*curr.as_ptr() } {
                let k = unsafe { std::ptr::read(key) };
                let v = unsafe { std::ptr::read(value) };
                return Some((k, v));
            }
        }

        None
    }
}

impl<K, V> IntoIterator for HashMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<K, V> Drop for IntoIter<K, V> {
    fn drop(&mut self) {
        let len = unsafe { self.end.offset_from(self.curr.as_ptr()) as usize };
        let slice = unsafe { std::slice::from_raw_parts_mut(self.curr.as_ptr(), len) };
        unsafe { std::ptr::drop_in_place(slice) }

        if self.map.alloc_size > 0 {
            let layout = Layout::array::<Cell<K, V>>(self.map.alloc_size).unwrap();
            unsafe { dealloc(self.map.ptr.as_ptr() as _, layout) }
        }
    }
}

pub struct Iter<'map, K, V> {
    map: PhantomData<&'map HashMap<K, V>>,
    curr: NonNull<Cell<K, V>>,
    end: *const Cell<K, V>,
}

impl<'map, K, V> Iter<'map, K, V> {
    fn new(map: &'map HashMap<K, V>) -> Self {
        Self {
            curr: map.ptr,
            end: unsafe { map.ptr.as_ptr().add(map.alloc_size) },
            map: PhantomData,
        }
    }
}

impl<'map, K, V> Iterator for Iter<'map, K, V> {
    type Item = (&'map K, &'map V);

    fn next(&mut self) -> Option<Self::Item> {
        while self.curr.as_ptr() as *const _ != self.end {
            let curr = self.curr;
            self.curr = unsafe { NonNull::new(self.curr.as_ptr().add(1)).unwrap() };

            if let Cell::Item { key, value } = unsafe { &*curr.as_ptr() } {
                return Some((key, value));
            }
        }

        None
    }
}

impl<'map, K, V> IntoIterator for &'map HashMap<K, V> {
    type Item = (&'map K, &'map V);
    type IntoIter = Iter<'map, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

pub struct IterMut<'map, K, V> {
    map: PhantomData<&'map mut HashMap<K, V>>,
    curr: NonNull<Cell<K, V>>,
    end: *const Cell<K, V>,
}

impl<'map, K, V> IterMut<'map, K, V> {
    fn new(map: &'map mut HashMap<K, V>) -> Self {
        Self {
            curr: map.ptr,
            end: unsafe { map.ptr.as_ptr().add(map.alloc_size) },
            map: PhantomData,
        }
    }
}

impl<'map, K, V> Iterator for IterMut<'map, K, V> {
    type Item = (&'map K, &'map mut V);

    fn next(&mut self) -> Option<Self::Item> {
        while self.curr.as_ptr() as *const _ != self.end {
            let curr = self.curr;
            self.curr = unsafe { NonNull::new(self.curr.as_ptr().add(1)).unwrap() };

            if let Cell::Item { key, value } = unsafe { &mut *curr.as_ptr() } {
                return Some((key, value));
            }
        }

        None
    }
}

impl<'map, K, V> IntoIterator for &'map mut HashMap<K, V> {
    type Item = (&'map K, &'map mut V);
    type IntoIter = IterMut<'map, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut::new(self)
    }
}

impl<K, V> FromIterator<(K, V)> for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let (hint_low, hint_high) = iter.size_hint();
        let hint = hint_high.unwrap_or(hint_low);
        let mut this = Self::with_capacity(hint);

        for (key, value) in iter {
            this.insert(key, value);
        }

        this
    }
}
