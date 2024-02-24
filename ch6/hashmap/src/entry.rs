use crate::{Cell, HashMap};
use std::hash::Hash;

pub struct VacantEntry<'map, K, V> {
    map: &'map mut Cell<K, V>,
    key: K,
}

impl<'map, K, V> VacantEntry<'map, K, V> {
    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn into_key(self) -> K {
        self.key
    }

    pub fn insert(self, value: V) -> &'map mut V {
        *self.map = Cell::Item {
            key: self.key,
            value,
        };

        match self.map {
            Cell::Item { value, .. } => value,
            _ => unreachable!(),
        }
    }
}

pub struct OccupiedEntry<'map, K, V> {
    cell: &'map mut Cell<K, V>,
}

impl<'map, K, V> OccupiedEntry<'map, K, V> {
    pub fn get(&self) -> &V {
        match &self.cell {
            Cell::Item { value, .. } => value,
            _ => unreachable!(),
        }
    }

    pub fn get_mut(&mut self) -> &mut V {
        match &mut self.cell {
            Cell::Item { value, .. } => value,
            _ => unreachable!(),
        }
    }

    pub fn into_mut(self) -> &'map mut V {
        match self.cell {
            Cell::Item { value, .. } => value,
            _ => unreachable!(),
        }
    }
}
