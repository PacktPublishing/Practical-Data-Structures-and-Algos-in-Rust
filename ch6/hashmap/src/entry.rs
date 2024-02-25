use crate::{Cell, HashMap};
use std::hash::Hash;

pub struct VacantEntry<'map, K, V> {
    cell: &'map mut Cell<K, V>,
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
        *self.cell = Cell::Item {
            key: self.key,
            value,
        };

        match self.cell {
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

    pub fn insert(&mut self, mut value: V) -> V {
        match self.cell {
            Cell::Item { value: v, .. } => std::mem::swap(v, &mut value),
            _ => unreachable!(),
        }

        value
    }

    pub fn key(&self) -> &K {
        match &self.cell {
            Cell::Item { key, .. } => key,
            _ => unreachable!(),
        }
    }

    pub fn remove(self) -> V {
        let mut cell = Cell::Tombstone;
        std::mem::swap(self.cell, &mut cell);

        match cell {
            Cell::Item { value, .. } => value,
            _ => unreachable!(),
        }
    }
}

pub enum Entry<'map, K, V> {
    Occupied(OccupiedEntry<'map, K, V>),
    Vacant(VacantEntry<'map, K, V>),
}

impl<'map, K, V> Entry<'map, K, V> {
    pub fn and_modify(mut self, f: impl FnOnce(&mut V)) -> Self {
        match &mut self {
            Entry::Occupied(entry) => f(entry.get_mut()),
            Entry::Vacant(_) => (),
        }

        self
    }

    pub fn key(&self) -> &K {
        match self {
            Entry::Occupied(entry) => entry.key(),
            Entry::Vacant(entry) => entry.key(),
        }
    }

    pub fn or_default(self) -> &'map mut V
    where
        V: Default,
    {
        self.or_insert_with(V::default)
    }

    pub fn or_insert(self, default: V) -> &'map mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }

    pub fn or_insert_with(self, default: impl FnOnce() -> V) -> &'map mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default()),
        }
    }

    pub fn or_insert_with_key(self, default: impl FnOnce(&K) -> V) -> &'map mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let default = default(entry.key());
                entry.insert(default)
            }
        }
    }
}

impl<K, V> HashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        let idx = self.expected_idx(&key);

        for idx in self.idx_chain(idx) {
            match &self.vec[idx] {
                Cell::Item { key: k, .. } if *k == key => {
                    return Entry::Occupied(OccupiedEntry {
                        cell: &mut self.vec[idx],
                    })
                }
                Cell::Empty | Cell::Tombstone => {
                    return Entry::Vacant(VacantEntry {
                        cell: &mut self.vec[idx],
                        key,
                    })
                }
                _ => (),
            }
        }

        unreachable!()
    }
}
