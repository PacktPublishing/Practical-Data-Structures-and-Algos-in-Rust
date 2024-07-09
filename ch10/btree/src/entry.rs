use crate::Node;

pub enum Entry<'t, K, V> {
    Vacant(VacantEntry<'t, K, V>),
    Occupied(OccupiedEntry<'t, K, V>),
}

impl<'t, K, V> Entry<'t, K, V> {
    pub(crate) fn new(root: &'t mut Node<K, V>, key: K) -> Self
    where
        K: Ord,
    {
        root.split_root();
        let (node, idx) = root.entry(&key);

        if idx < node.items.len() && node.items[idx].0 == key {
            Entry::Occupied(OccupiedEntry { node, index: idx })
        } else {
            Entry::Vacant(VacantEntry {
                node,
                index: idx,
                key,
            })
        }
    }

    pub fn key(&self) -> &K {
        match self {
            Entry::Vacant(entry) => entry.key(),
            Entry::Occupied(entry) => entry.key(),
        }
    }

    pub fn or_insert(self, value: V) -> &'t mut V {
        self.or_insert_with(move || value)
    }

    pub fn or_default(self) -> &'t mut V
    where
        V: Default,
    {
        #[allow(clippy::unwrap_or_default)]
        self.or_insert_with(Default::default)
    }

    pub fn or_insert_with(self, f: impl FnOnce() -> V) -> &'t mut V {
        match self {
            Entry::Vacant(entry) => entry.insert(f()),
            Entry::Occupied(entry) => &mut entry.node.items[entry.index].1,
        }
    }

    pub fn or_insert_with_key(self, f: impl FnOnce(&K) -> V) -> &'t mut V {
        match self {
            Entry::Vacant(entry) => entry.insert_with_key(f),
            Entry::Occupied(entry) => &mut entry.node.items[entry.index].1,
        }
    }

    pub fn and_modify(self, f: impl FnOnce(&mut V)) -> Self {
        match self {
            Entry::Vacant(entry) => Entry::Vacant(entry),
            Entry::Occupied(mut entry) => {
                f(entry.get_mut());
                Entry::Occupied(entry)
            }
        }
    }
}

pub struct VacantEntry<'t, K, V> {
    node: &'t mut Node<K, V>,
    index: usize,
    key: K,
}

impl<'t, K, V> VacantEntry<'t, K, V> {
    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn insert(self, value: V) -> &'t mut V {
        self.node.items.insert(self.index, (self.key, value));
        &mut self.node.items[self.index].1
    }

    pub fn insert_with_key(self, f: impl FnOnce(&K) -> V) -> &'t mut V {
        let value = f(&self.key);
        self.node.items.insert(self.index, (self.key, value));
        &mut self.node.items[self.index].1
    }
}

pub struct OccupiedEntry<'t, K, V> {
    node: &'t mut Node<K, V>,
    index: usize,
}

impl<'t, K, V> OccupiedEntry<'t, K, V> {
    pub fn key(&self) -> &K {
        &self.node.items[self.index].0
    }

    pub fn get_mut(&mut self) -> &mut V {
        &mut self.node.items[self.index].1
    }
}
