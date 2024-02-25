use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash};

mod entry;
mod iterator;

pub const MIN_SIZE_SHIFT: usize = 3;

#[derive(Debug)]
enum Cell<K, V> {
    Empty,
    Tombstone,
    Item { key: K, value: V },
}

impl<K, V> Cell<K, V> {
    /// `Empty` or `Tombstone` - unoccupied entry
    fn is_empty(&self) -> bool {
        match self {
            Self::Empty | Self::Tombstone => true,
            Self::Item { .. } => false,
        }
    }
}

pub struct HashMap<K, V> {
    vec: Vec<Cell<K, V>>,
    /// How many items are used - when it reaches half an alloc_size, the memory is reallocated
    used: usize,
    hasher_builder: RandomState,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            vec: vec![],
            used: 0,
            hasher_builder: RandomState::default(),
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            vec: std::iter::repeat_with(|| Cell::Empty)
                .take(Self::next_size(cap))
                .collect(),
            used: 0,
            hasher_builder: RandomState::default(),
        }
    }

    /// Returns proper map size for at least given capacity
    fn next_size(cap: usize) -> usize {
        let newsize = cap * 2;
        std::iter::successors(Some(MIN_SIZE_SHIFT), |i| Some(i * 2))
            .find(|i| (2 << *i) >= newsize)
            .unwrap()
    }

    /// Indicies chain starting from given index, wrapping around the map, visiting every index
    /// once
    fn idx_chain(&self, idx: usize) -> impl Iterator<Item = usize> {
        (idx..self.vec.len()).chain(0..idx)
    }

    /// Expected index where given key should be stored.
    fn expected_idx<Q: Hash + ?Sized>(&self, key: &Q) -> usize {
        let hash = self.hasher_builder.hash_one(key) % self.vec.len() as u64;
        hash as _
    }
}

impl<K, V> HashMap<K, V>
where
    K: Eq + Hash,
{
    /// After this call, the map should have capacity to fit at least `newcap`.
    fn grow_to(&mut self, newcap: usize) {
        if newcap == 0 {
            return;
        }

        let newsize = Self::next_size(newcap);
        if newsize > self.vec.len() {
            self.vec.resize_with(newsize, || Cell::Empty);
        }

        self.rehash();
    }

    pub fn rehash(&mut self) {
        for i in 0..self.vec.len() {
            let item = &mut self.vec[i];
            let key = match item {
                Cell::Tombstone => {
                    *item = Cell::Empty;
                    continue;
                }
                Cell::Empty => continue,
                Cell::Item { key, .. } => key,
            };
            let idx = self.expected_idx(key);

            if idx == i {
                continue;
            }

            // Find first unoccuppied index. It is guaranteed to find one, as we only fill half the
            // map at most
            let idx = self
                .idx_chain(idx)
                .find(|idx| self.vec[*idx].is_empty())
                .unwrap();

            self.vec[idx] = Cell::Empty;
            self.vec.swap(i, idx);
        }
    }

    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let idx = self.expected_idx(k);

        self.idx_chain(idx)
            .map_while(|idx| match &self.vec[idx] {
                Cell::Empty => None,
                item => Some(item),
            })
            .find_map(|item| match item {
                Cell::Item { key, value } if key.borrow() == k => Some(value),
                _ => None,
            })
    }

    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let idx = self.expected_idx(k);

        self.idx_chain(idx)
            .take_while(|idx| !matches!(&self.vec[*idx], Cell::Empty))
            .find_map(|idx| match &mut self.vec[idx] {
                Cell::Item { key, value } if Borrow::<Q>::borrow(key) == k => Some(value),
                _ => None,
            })
    }

    pub fn insert(&mut self, k: K, mut v: V) -> Option<V> {
        let idx = self.expected_idx(&k);

        for idx in self.idx_chain(idx) {
            match &mut self.vec[idx] {
                Cell::Empty => break,
                Cell::Item { key, value } if *key == k => {
                    std::mem::swap(value, &mut v);
                    return Some(v);
                }
                _ => (),
            }
        }

        self.grow_to(self.used + 1);

        let idx = self.expected_idx(&k);
        let cell = self
            .idx_chain(idx)
            .find_map(|idx| {
                if self.vec[idx].is_empty() {
                    Some(&mut self.vec[idx])
                } else {
                    None
                }
            })
            .unwrap();

        *cell = Cell::Item { key: k, value: v };
        None
    }

    pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let idx = self.expected_idx(k);

        self.idx_chain(idx)
            .take_while(|idx| !matches!(&self.vec[*idx], Cell::Empty))
            .find_map(|idx| match &self.vec[idx] {
                Cell::Item { key, .. } if (*key).borrow() == k => {
                    let mut cell = Cell::Tombstone;
                    std::mem::swap(&mut self.vec[idx], &mut cell);
                    let Cell::Item { value, .. } = cell else {
                        unreachable!()
                    };

                    Some(value)
                }
                _ => None,
            })
    }
}

impl<K, V> Default for HashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct HashSet<T>(HashMap<T, ()>);

impl<T> HashSet<T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self(HashMap::with_capacity(cap))
    }
}

impl<T> HashSet<T>
where
    T: Eq + Hash,
{
    pub fn rehash(&mut self) {
        self.0.rehash()
    }

    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let map = self.0;
        let idx = map.expected_idx(value);

        map.idx_chain(idx)
            .map_while(|idx| match &map.vec[idx] {
                Cell::Empty => None,
                item => Some(item),
            })
            .find_map(|item| match item {
                Cell::Item { key, .. } if key.borrow() == value => Some(key),
                _ => None,
            })
    }

    pub fn insert(&mut self, value: T) -> bool {
        self.0.insert(value, ()).is_some()
    }

    pub fn remove<Q>(&mut self, k: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.0.remove(k).is_some()
    }
}

impl<T> Default for HashSet<T> {
    fn default() -> Self {
        Self::new()
    }
}
