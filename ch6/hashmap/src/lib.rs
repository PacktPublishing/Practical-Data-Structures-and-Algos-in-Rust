use std::alloc::{alloc, dealloc, realloc, Layout};
use std::borrow::{Borrow, BorrowMut};
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash};
use std::mem::size_of;
use std::ptr::{drop_in_place, NonNull};

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
    ptr: NonNull<Cell<K, V>>,
    /// Alloc size is always a power of two
    alloc_size: usize,
    /// How many items are used - when it reaches half an alloc_size, the memory is reallocated
    used: usize,
    hasher_builder: RandomState,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self::assert_zst();

        Self {
            ptr: NonNull::dangling(),
            alloc_size: 0,
            used: 0,
            hasher_builder: RandomState::default(),
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self::assert_zst();

        let alloc_size = Self::next_size(cap);
        let layout = Layout::array::<Cell<K, V>>(alloc_size).unwrap();
        let ptr = unsafe { alloc(layout) as _ };

        Self {
            ptr: NonNull::new(ptr).unwrap(),
            alloc_size,
            used: 0,
            hasher_builder: RandomState::default(),
        }
    }

    pub fn assert_zst() {
        if size_of::<Cell<K, V>>() == 0 {
            panic!("ZSTs are not supported");
        }
    }

    /// Returns proper map size for at least given capacity
    pub fn next_size(cap: usize) -> usize {
        let newsize = cap * 2;
        std::iter::successors(Some(MIN_SIZE_SHIFT), |i| Some(i * 2))
            .find(|i| (2 << *i) >= newsize)
            .unwrap()
    }

    /// Indicies chain starting from given index, wrapping around the map, visiting every index
    /// once
    fn idx_chain(&self, idx: usize) -> impl Iterator<Item = usize> {
        (idx..self.alloc_size).chain(0..idx)
    }

    /// Expected index where given key should be stored.
    fn expected_idx<Q: Hash + ?Sized>(&self, key: &Q) -> usize {
        let hash = self.hasher_builder.hash_one(key) % self.alloc_size as u64;
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

        match self.alloc_size {
            0 => {
                self.alloc_size = newsize;
                let layout = Layout::array::<Cell<K, V>>(self.alloc_size).unwrap();
                let ptr = unsafe { alloc(layout) as _ };
                self.ptr = NonNull::new(ptr).unwrap();
            }
            size if size < newsize => {
                let layout = Layout::array::<Cell<K, V>>(self.alloc_size).unwrap();
                let newlayout = Layout::array::<Cell<K, V>>(newsize).unwrap();
                let ptr = unsafe {
                    realloc(self.ptr.as_ptr() as _, layout, newlayout.size()) as *mut Cell<K, V>
                };
                for i in (self.alloc_size + 1)..newsize {
                    unsafe { ptr.add(i).write(Cell::Empty) }
                }
                self.alloc_size = newsize;
                self.ptr = NonNull::new(ptr).unwrap();
            }
            _ => (),
        }

        self.rehash();
    }

    pub fn rehash(&mut self) {
        for i in 0..self.alloc_size {
            let item = unsafe { &*self.ptr.as_ptr().add(i) };
            let key = match item {
                Cell::Tombstone => {
                    unsafe { self.ptr.as_ptr().add(i).write(Cell::Empty) };
                    continue;
                }
                Cell::Empty => continue,
                Cell::Item { key, .. } => key,
            };
            let idx = self.hasher_builder.hash_one(key) % self.alloc_size as u64;
            let idx = idx as usize;

            if idx == i {
                continue;
            }

            // Find first unoccuppied index. It is guaranteed to find one, as we only fill half the
            // map at most
            let idx = self
                .idx_chain(idx)
                .find(|idx| unsafe { (*self.ptr.as_ptr().add(*idx)).is_empty() })
                .unwrap();

            let from = unsafe { self.ptr.as_ptr().add(i) };
            let to = unsafe { self.ptr.as_ptr().add(idx) };
            unsafe { to.write(Cell::Empty) };
            unsafe { from.swap(to) };
        }
    }

    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let idx = self.expected_idx(k);

        self.idx_chain(idx)
            .map_while(|idx| {
                let item = unsafe { &*self.ptr.as_ptr().add(idx) };
                match item {
                    Cell::Empty => None,
                    item => Some(item),
                }
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
            .map_while(|idx| {
                let item = unsafe { &mut *self.ptr.as_ptr().add(idx) };
                match item {
                    Cell::Empty => None,
                    item => Some(item),
                }
            })
            .find_map(|item| match item {
                Cell::Item { key, value } if Borrow::<Q>::borrow(key) == k => Some(value),
                _ => None,
            })
    }

    pub fn insert(&mut self, k: K, mut v: V) -> Option<V> {
        let idx = self.expected_idx(&k);

        for idx in self.idx_chain(idx) {
            let item = unsafe { &mut *self.ptr.as_ptr().add(idx) };

            match item {
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
        let ptr = self
            .idx_chain(idx)
            .find_map(|idx| {
                let ptr = unsafe { self.ptr.as_ptr().add(idx) };
                if unsafe { (*ptr).is_empty() } {
                    Some(ptr)
                } else {
                    None
                }
            })
            .unwrap();
        unsafe { ptr.write(Cell::Item { key: k, value: v }) };

        None
    }

    pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let idx = self.expected_idx(k);

        self.idx_chain(idx)
            .map_while(|idx| {
                let ptr = unsafe { self.ptr.as_ptr().add(idx) };
                match unsafe { &*ptr } {
                    Cell::Empty => None,
                    _ => Some(ptr),
                }
            })
            .find_map(|ptr| match unsafe { &mut *ptr } {
                Cell::Item { key, value } if (*key).borrow() == k => {
                    let res = unsafe { std::ptr::read(value) };
                    unsafe { drop_in_place(key) };
                    unsafe { ptr.write(Cell::Tombstone) };
                    Some(res)
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

impl<K, V> Drop for HashMap<K, V> {
    fn drop(&mut self) {
        let slice = unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.alloc_size) };
        unsafe { std::ptr::drop_in_place(slice) };

        if self.alloc_size > 0 {
            let layout = Layout::array::<Cell<K, V>>(self.alloc_size).unwrap();
            unsafe {
                dealloc(self.ptr.as_ptr() as _, layout);
            }
        }
    }
}
