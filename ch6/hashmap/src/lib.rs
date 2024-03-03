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
    Item { hash: u64, key: K, value: V },
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
            vec: Vec::new(),
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
        (MIN_SIZE_SHIFT..).find(|i| (2 << *i) >= newsize).unwrap()
    }

    /// Indicies chain starting from given index, wrapping around the map, visiting every index
    /// once
    fn idx_chain(&self, idx: usize) -> impl Iterator<Item = usize> {
        (idx..self.vec.len()).chain(0..idx)
    }

    fn cells(&self, idx: usize) -> impl Iterator<Item = &'_ Cell<K, V>> {
        let pre = &self.vec[idx..];
        let post = &self.vec[..idx];
        pre.iter().chain(post)
    }

    fn cells_mut(&mut self, idx: usize) -> impl Iterator<Item = &'_ mut Cell<K, V>> {
        let (post, pre) = self.vec.split_at_mut(idx);
        pre.iter_mut().chain(post)
    }
}

impl<K, V> HashMap<K, V>
where
    K: Eq + Hash,
{
    /// After this call, the map should have capacity to fit at least `newcap`.
    fn grow_to(&mut self, newcap: usize) {
        let minsize = self.vec.len().max(2 << MIN_SIZE_SHIFT);
        let newsize = std::iter::successors(Some(minsize), |size| Some(size * 2))
            .find(|size| *size >= newcap * 2)
            .unwrap();

        if newsize > self.vec.len() {
            self.vec.resize_with(newsize, || Cell::Empty);
            self.rehash();
        }
    }

    pub fn rehash(&mut self) {
        for i in 0..self.vec.len() {
            let hash = match &self.vec[i] {
                Cell::Tombstone => {
                    self.vec[i] = Cell::Empty;
                    continue;
                }
                Cell::Empty => continue,
                Cell::Item { hash, .. } => *hash,
            };

            let idx = hash as usize % self.vec.len();
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
        if self.vec.is_empty() {
            return None;
        }

        let h = self.hasher_builder.hash_one(k);
        let idx = h as usize % self.vec.len();

        self.cells(idx)
            .take_while(|cell| !matches!(cell, Cell::Empty))
            .find_map(|cell| match cell {
                Cell::Item { key, hash, value } if h == *hash && key.borrow() == k => Some(value),
                _ => None,
            })
    }

    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if self.vec.is_empty() {
            return None;
        }

        let h = self.hasher_builder.hash_one(k);
        let idx = h as usize % self.vec.len();

        self.cells_mut(idx)
            .take_while(|cell| !matches!(cell, Cell::Empty))
            .find_map(|cell| match cell {
                Cell::Item { key, hash, value } if h == *hash && Borrow::<Q>::borrow(key) == k => {
                    Some(value)
                }
                _ => None,
            })
    }

    pub fn insert(&mut self, k: K, mut v: V) -> Option<V> {
        let h = self.hasher_builder.hash_one(&k);

        if !self.vec.is_empty() {
            let idx = h as usize % self.vec.len();

            for cell in self.cells_mut(idx) {
                match cell {
                    Cell::Empty => break,
                    Cell::Item { key, hash, value } if h == *hash && *key == k => {
                        std::mem::swap(value, &mut v);
                        return Some(v);
                    }
                    _ => (),
                }
            }
        }

        self.grow_to(self.used + 1);

        let idx = h as usize % self.vec.len();

        let cell = self
            .cells_mut(idx)
            .find_map(|cell| if cell.is_empty() { Some(cell) } else { None })
            .unwrap();

        *cell = Cell::Item {
            key: k,
            hash: h,
            value: v,
        };

        self.used += 1;

        None
    }

    pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let h = self.hasher_builder.hash_one(k);
        let idx = h as usize % self.vec.len();

        let res = self
            .cells_mut(idx)
            .take_while(|cell| !matches!(cell, Cell::Empty))
            .find_map(|cell| match cell {
                Cell::Item { key, hash, .. } if h == *hash && (*key).borrow() == k => {
                    let mut result = Cell::Tombstone;
                    std::mem::swap(cell, &mut result);
                    let Cell::Item { value, .. } = result else {
                        unreachable!()
                    };

                    Some(value)
                }
                _ => None,
            });

        if res.is_some() {
            self.used -= 1;
        }

        res
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
        let map = &self.0;

        if map.vec.is_empty() {
            return None;
        }

        let h = map.hasher_builder.hash_one(value);
        let idx = h as usize % map.vec.len();

        map.idx_chain(idx)
            .map_while(|idx| match &map.vec[idx] {
                Cell::Empty => None,
                item => Some(item),
            })
            .find_map(|item| match item {
                Cell::Item { key, hash, .. } if h == *hash && key.borrow() == value => Some(key),
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
