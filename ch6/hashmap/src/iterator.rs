use std::hash::Hash;
use std::{slice, vec};

use crate::{Cell, HashMap};

pub struct IntoIter<K, V>(vec::IntoIter<Cell<K, V>>);

impl<K, V> IntoIter<K, V> {
    fn new(map: HashMap<K, V>) -> Self {
        Self(map.vec.into_iter())
    }
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        #[allow(clippy::while_let_on_iterator)]
        while let Some(cell) = self.0.next() {
            if let Cell::Item { key, value, .. } = cell {
                return Some((key, value));
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

pub struct Iter<'map, K, V>(slice::Iter<'map, Cell<K, V>>);

impl<'map, K, V> Iter<'map, K, V> {
    fn new(map: &'map HashMap<K, V>) -> Self {
        Self(map.vec.iter())
    }
}

impl<'map, K, V> Iterator for Iter<'map, K, V> {
    type Item = (&'map K, &'map V);

    fn next(&mut self) -> Option<Self::Item> {
        #[allow(clippy::while_let_on_iterator)]
        while let Some(cell) = self.0.next() {
            if let Cell::Item { key, value, .. } = cell {
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

pub struct IterMut<'map, K, V>(slice::IterMut<'map, Cell<K, V>>);

impl<'map, K, V> IterMut<'map, K, V> {
    fn new(map: &'map mut HashMap<K, V>) -> Self {
        Self(map.vec.iter_mut())
    }
}

impl<'map, K, V> Iterator for IterMut<'map, K, V> {
    type Item = (&'map K, &'map mut V);

    fn next(&mut self) -> Option<Self::Item> {
        #[allow(clippy::while_let_on_iterator)]
        while let Some(cell) = self.0.next() {
            if let Cell::Item { key, value, .. } = cell {
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
