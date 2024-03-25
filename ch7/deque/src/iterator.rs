use std::mem::MaybeUninit;

use crate::{Deque, MIN_SHIFT};

pub struct IntoIter<T>(Deque<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> IntoIterator for Deque<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

pub type Iter<'a, T> = std::iter::Chain<std::slice::Iter<'a, T>, std::slice::Iter<'a, T>>;

impl<'a, T> IntoIterator for &'a Deque<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let (fst, lst) = self.as_slices();
        fst.iter().chain(lst)
    }
}

pub type IterMut<'a, T> = std::iter::Chain<std::slice::IterMut<'a, T>, std::slice::IterMut<'a, T>>;

impl<'a, T> IntoIterator for &'a mut Deque<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let (fst, lst) = self.as_slices_mut();
        fst.iter_mut().chain(lst)
    }
}

impl<T> FromIterator<T> for Deque<T> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let mut buffer: Vec<_> = iter
            .into_iter()
            .map(|item| MaybeUninit::new(item))
            .collect();
        let len = buffer.len();
        let shift = Self::make_shift(MIN_SHIFT, buffer.len());
        buffer.resize_with(shift, || MaybeUninit::uninit());

        Self {
            buffer,
            head: 0,
            len,
        }
    }
}
