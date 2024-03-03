use crate::{Cell, HashMap};
use std::hash::BuildHasher;
use std::hash::Hash;

pub struct VacantEntry<'map, K, V> {
    cell: &'map mut Cell<K, V>,
    used: &'map mut usize,
    hash: u64,
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
        *self.used += 1;
        *self.cell = Cell::Item {
            key: self.key,
            hash: self.hash,
            value,
        };

        let Cell::Item { value, .. } = self.cell else {
            unreachable!()
        };

        value
    }
}

pub struct OccupiedEntry<'map, K, V> {
    cell: &'map mut Cell<K, V>,
    used: &'map mut usize,
}

impl<'map, K, V> OccupiedEntry<'map, K, V> {
    pub fn get(&self) -> &V {
        let Cell::Item { value, .. } = &self.cell else {
            unreachable!()
        };
        value
    }

    pub fn get_mut(&mut self) -> &mut V {
        let Cell::Item { value, .. } = &mut self.cell else {
            unreachable!()
        };
        value
    }

    pub fn into_mut(self) -> &'map mut V {
        let Cell::Item { value, .. } = self.cell else {
            unreachable!()
        };
        value
    }

    pub fn insert(&mut self, mut value: V) -> V {
        let Cell::Item { value: v, .. } = self.cell else {
            unreachable!()
        };
        std::mem::swap(v, &mut value);
        value
    }

    pub fn key(&self) -> &K {
        let Cell::Item { key, .. } = &self.cell else {
            unreachable!()
        };
        key
    }

    pub fn remove(self) -> V {
        *self.used -= 1;
        let mut cell = Cell::Tombstone;
        std::mem::swap(self.cell, &mut cell);

        let Cell::Item { value, .. } = cell else {
            unreachable!()
        };

        value
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
        #[allow(clippy::unwrap_or_default)]
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
        if self.vec.is_empty() {
            self.grow_to(1);
        }

        let hash = self.hasher_builder.hash_one(&key);
        let idx = hash as usize % self.vec.len();

        for idx in self.idx_chain(idx) {
            match &self.vec[idx] {
                Cell::Item { key: k, .. } if *k == key => {
                    return Entry::Occupied(OccupiedEntry {
                        cell: &mut self.vec[idx],
                        used: &mut self.used,
                    })
                }
                Cell::Empty | Cell::Tombstone => {
                    self.grow_to(self.used + 1);
                    return Entry::Vacant(VacantEntry {
                        cell: &mut self.vec[idx],
                        used: &mut self.used,
                        hash,
                        key,
                    });
                }
                _ => (),
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn or_insert() {
        let mut map: HashMap<u32, u32> = HashMap::new();
        map.insert(4, 2);
        map.insert(5, 9);

        let four = *map.entry(4).or_insert(10);
        let six = *map.entry(6).or_insert(11);

        assert_eq!(four, 2);
        assert_eq!(six, 11);

        *map.entry(5).or_insert(12) = 13;
        *map.entry(7).or_insert(14) = 15;

        assert_eq!(map.get(&5), Some(&13));
        assert_eq!(map.get(&7), Some(&15));
    }
}
