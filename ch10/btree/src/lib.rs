const B: usize = 2;
const MIN_ITEMS: usize = B - 1;
const MAX_ITEMS: usize = 2 * B - 1;
const MAX_CHILDREN: usize = MAX_ITEMS + 1;

#[derive(Debug, Clone)]
pub struct BTree<K, V> {
    root: Node<K, V>,
}

impl<K, V> BTree<K, V> {
    pub fn new() -> Self {
        let root = Node {
            items: Vec::with_capacity(MAX_ITEMS),
            children: Vec::with_capacity(MAX_CHILDREN),
        };

        Self { root }
    }

    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: Ord,
    {
        self.root.get(key)
    }

    pub fn iter(&self) -> Iter<K, V> {
        Iter::new(&self.root)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord,
    {
        self.root.split_root();
        self.root.insert(key, value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V>
    where
        K: Ord,
    {
        let result = self.root.remove(key);
        self.root.reduce_root();
        result
    }

    pub fn pop_front(&mut self) -> Option<(K, V)>
    where
        K: Ord,
    {
        let result = self.root.remove_first().unwrap();
        self.root.reduce_root();
        Some(result)
    }

    pub fn pop_back(&mut self) -> Option<(K, V)>
    where
        K: Ord,
    {
        let result = self.root.remove_last();
        self.root.reduce_root();
        result
    }
}

impl<K, V> PartialEq for BTree<K, V>
where
    K: PartialEq,
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<K, V> Default for BTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct Node<K, V> {
    items: Vec<(K, V)>,
    children: Vec<Self>,
}

impl<K, V> Node<K, V>
where
    K: Ord,
{
    /// Return the value for the given index
    pub fn get(&self, key: &K) -> Option<&V> {
        let idx = self.items.binary_search_by(|item| item.0.cmp(key));

        match idx {
            Ok(idx) => Some(&self.items[idx].1),
            Err(_) if self.children.is_empty() => None,
            Err(idx) => Node::get(&self.children[idx], key),
        }
    }

    fn is_full(&self) -> bool {
        self.items.len() >= MAX_ITEMS
    }

    /// Splits the root node - makes it into the node with only two elements after the split. Split
    /// is performed only if the node is full. Returns if the root node was actually split.
    fn split_root(&mut self) {
        if let Some((split, right)) = self.split() {
            let left = Self {
                items: std::mem::replace(&mut self.items, Vec::with_capacity(MAX_ITEMS)),
                children: std::mem::replace(&mut self.children, Vec::with_capacity(MAX_CHILDREN)),
            };

            self.items.push(split);
            self.children.push(left);
            self.children.push(right);
        }
    }

    /// Returns mutable references to the item in the tree and both children split by this item.
    fn siblings(&mut self, idx: usize) -> (&mut (K, V), &mut Self, &mut Self) {
        let item = &mut self.items[idx];
        let [left, right] = &mut self.children[idx..=idx + 1] else {
            unreachable!()
        };
        (item, left, right)
    }

    /// Splits the given node returning the middle element and the right node. The `self` node
    /// remains as a left node after split.
    ///
    /// The node splits only if it contains `2 * B - 1` elements (so it's full), as this is the only
    /// scenario that ensures enough elements for both nodes after the split to be valid. If the
    /// node is not full, this returns `None`.
    fn split(&mut self) -> Option<((K, V), Self)> {
        if !self.is_full() {
            return None;
        }

        let mut right = Self {
            items: Vec::with_capacity(MAX_ITEMS),
            children: Vec::with_capacity(MAX_CHILDREN),
        };

        right.items.extend(self.items.drain(MIN_ITEMS + 1..));
        if !self.children.is_empty() {
            right.children.extend(self.children.drain(MIN_ITEMS + 1..));
        }

        let split = self.items.pop().unwrap();
        Some((split, right))
    }

    /// Insert the element into this node. It assumes this node is not full, so the child node can
    /// be split if necessary without backtracking.
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        assert!(self.items.len() < MAX_ITEMS);
        let idx = self.items.binary_search_by(|item| item.0.cmp(&key));

        let idx = match idx {
            // Key already exists, overwrite
            Ok(idx) => {
                return Some(std::mem::replace(&mut self.items[idx].1, value));
            }
            Err(idx) => idx,
        };

        if self.children.is_empty() {
            // Leaf node, just insert the element
            self.items.insert(idx, (key, value));
            None
        } else if let Some((split, right)) = self.children[idx].split() {
            // Internal node, and the child to insert into was full, so the split succeeded. We
            // need to insert the right node after the split and the middle element - we can do so,
            // as `self` is not a full node.
            self.items.insert(idx, split);
            self.children.insert(idx + 1, right);

            let (split, left, right) = self.siblings(idx);
            match key.cmp(&split.0) {
                std::cmp::Ordering::Less => left.insert(key, value),
                std::cmp::Ordering::Greater => right.insert(key, value),
                // After split the item with matching key ended up in this node - replace and return
                std::cmp::Ordering::Equal => Some(std::mem::replace(&mut split.1, value)),
            }
        } else {
            // Internal node, and the child to insert into is not full
            self.children[idx].insert(key, value)
        }
    }

    /// Merges the `idx` child with its right sibling. That removes one item from the node, so it
    /// should have at least `MIN_ITEMS + 1` if it is not a root node.
    fn merge(&mut self, idx: usize) {
        let right = self.children.remove(idx + 1);
        let item = self.items.remove(idx);
        let left = &mut self.children[idx];

        left.items.push(item);
        left.items.extend(right.items);
        left.children.extend(right.children);
    }

    /// Makes sure that the child with index `idx` has at least `B` items, so it is possible to
    /// remove from it. As the child could be merged with it left sibling, we return the noew child
    /// index in case it changed. Note, that it might remove the element from self so it should
    /// have at least `MIN_ITEMS + 1` items if it is not a root node.
    fn make_removable(&mut self, idx: usize) -> usize {
        if self.children[idx].items.len() > MIN_ITEMS {
            idx
        } else if idx > 0 && self.children[idx - 1].items.len() > MIN_ITEMS {
            // Borrow from the left sibling
            let (split, left, right) = self.siblings(idx - 1);
            let newsplit = left.items.pop().unwrap();
            let item = std::mem::replace(split, newsplit);
            right.items.insert(0, item);

            if !left.children.is_empty() {
                let child = left.children.pop().unwrap();
                right.children.insert(0, child);
            }

            idx
        } else if idx < self.children.len() - 1 && self.children[idx + 1].items.len() > MIN_ITEMS {
            // Borrow from the right sibling
            let (split, left, right) = self.siblings(idx);
            let newsplit = right.items.remove(0);
            let item = std::mem::replace(split, newsplit);
            left.items.push(item);

            if !right.children.is_empty() {
                let child = right.children.remove(0);
                left.children.push(child);
            }

            idx
        } else {
            // Merge with the sibling. We want always merge with the right sibling to maintain the
            // index, so we alignt the index first if we are on the last child.
            let idx = idx.min(self.children.len() - 2);
            self.merge(idx);

            idx
        }
    }

    /// Removes the biggest item from the node if there is any. This function might remove the item from the node,
    /// so if it is not a root node, it should have at least `B` items.
    fn remove_last(&mut self) -> Option<(K, V)> {
        if self.children.is_empty() {
            // Leaf node - remove the last item
            self.items.pop()
        } else {
            // Internal node - make sure that the last child is ready for removal and remove from
            // it
            let idx = self.children.len() - 1;
            let idx = self.make_removable(idx);
            self.children[idx].remove_last()
        }
    }

    /// Removes the smallest item from the node. This function might remove the item from the node,
    /// so if it is not a root node, it should have at least `B` items.
    fn remove_first(&mut self) -> Option<(K, V)> {
        if self.items.is_empty() {
            // Empty root node
            None
        } else if self.children.is_empty() {
            // Leaf node - just remove the first item
            Some(self.items.remove(0))
        } else {
            // Internal node - remove the first item from the first child
            let idx = self.make_removable(0);
            self.children[idx].remove_first()
        }
    }

    /// Removes the item from the tree returning value assigned to it if tke item was existing.
    /// The function assumes that the node has at least B items, so two of its children can be
    /// safely merged that would remove also an item from this node, leaving it in at least `B - 1`
    /// items state.
    fn remove(&mut self, key: &K) -> Option<V> {
        let idx = self.items.binary_search_by(|item| item.0.cmp(key));

        match idx {
            Ok(idx) if self.children.is_empty() => {
                // Item to removed is in this node, and it is a leaf node - just remove it
                Some(self.items.remove(idx).1)
            }
            Ok(idx) if self.children[idx].items.len() > MIN_ITEMS => {
                // Item to remove is in this node, so we need a new element in its place, and we
                // can take it from the child left to the removed item without compromising it.
                let split = self.children[idx].remove_last().unwrap();
                let item = std::mem::replace(&mut self.items[idx], split);
                Some(item.1)
            }
            Ok(idx) if self.children[idx + 1].items.len() > MIN_ITEMS => {
                // Item to remove is in this node, so we need a new element in its place, and we
                // can take it from the child right to the removed item without compromising it.
                let split = self.children[idx + 1].remove_first().unwrap();
                let item = std::mem::replace(&mut self.items[idx], split);
                Some(item.1)
            }
            Ok(idx) => {
                // Item to remove is in this node, so we need a new element in its place, and both
                // children next to removed item are minimal nodes - we will merge them, and remove
                // from the merged node
                self.merge(idx);
                self.children[idx].remove(key)
            }
            // Index not found in th leaf node
            Err(_) if self.children.is_empty() => None,
            // Children to remove from has spare items - remove item from it
            Err(idx) if self.children[idx].items.len() > MIN_ITEMS => {
                self.children[idx].remove(key)
            }
            Err(idx) => {
                // Removing from the minimal child node - first we need to make it at least one
                // element bigger by borrowing an elementnt from the sibling or merging it with the
                // sibling
                let idx = self.make_removable(idx);
                self.children[idx].remove(key)
            }
        }
    }

    /// Reduce height of the tree if the root node has only a single child
    fn reduce_root(&mut self) {
        if self.children.len() == 1 {
            let root = self.children.pop().unwrap();
            *self = root;
        }
    }
}

impl<K, V> Clone for Node<K, V>
where
    K: Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        let mut cloned = Self {
            items: Vec::with_capacity(MAX_ITEMS),
            children: Vec::with_capacity(MAX_CHILDREN),
        };

        cloned.items.extend(self.items.iter().cloned());
        cloned.children.extend(self.children.iter().cloned());

        cloned
    }
}

pub struct Iter<'t, K, V>(Vec<(&'t Node<K, V>, usize)>);

impl<'t, K, V> Iter<'t, K, V> {
    /// Creates a new iterator
    fn new(mut root: &'t Node<K, V>) -> Self {
        let mut iter = Self(vec![]);

        // For empty tree we want to have an empty iterator
        if !root.items.is_empty() {
            iter.0.push((root, 0));
        }

        // Going down the tree for the first item
        while let Some(node) = root.children.first() {
            root = node;
            iter.0.push((root, 0));
        }

        iter
    }
}

impl<'t, K, V> Iterator for Iter<'t, K, V> {
    type Item = &'t (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        let (node, idx) = self.0.last_mut()?;
        let item = &node.items[*idx];
        *idx += 1;

        if !node.children.is_empty() {
            // Internal node - jump to the next iteam going down the tree
            let mut node = &node.children[*idx];
            self.0.push((node, 0));

            while let Some(child) = node.children.first() {
                node = child;
                self.0.push((node, 0));
            }
        } else if *idx >= node.items.len() {
            // Leaf node - go up the tree
            while matches!(self.0.last(), Some((node, idx)) if *idx >= node.items.len()) {
                self.0.pop();
            }
        }

        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty() {
        let btree = BTree::<u32, u32>::new();
        let items: Vec<_> = btree.iter().collect();

        assert_eq!(Vec::<&(u32, u32)>::new(), items);
    }

    #[test]
    fn get() {
        let mut btree = BTree::new();

        btree.insert(1, 2);
        btree.insert(3, 4);
        btree.insert(4, 5);
        btree.insert(7, 8);
        btree.insert(2, 3);
        btree.insert(15, 16);
        btree.insert(8, 9);
        btree.insert(9, 10);
        btree.insert(10, 11);
        btree.insert(12, 13);
        btree.insert(6, 7);
        btree.insert(11, 10);
        btree.insert(13, 14);
        btree.insert(5, 6);
        btree.insert(14, 15);

        assert_eq!(*btree.get(&1).unwrap(), 2);
        assert_eq!(*btree.get(&2).unwrap(), 3);
        assert_eq!(*btree.get(&3).unwrap(), 4);
        assert_eq!(*btree.get(&4).unwrap(), 5);
        assert_eq!(*btree.get(&5).unwrap(), 6);
        assert_eq!(*btree.get(&6).unwrap(), 7);
        assert_eq!(*btree.get(&7).unwrap(), 8);
        assert_eq!(*btree.get(&8).unwrap(), 9);
        assert_eq!(*btree.get(&9).unwrap(), 10);
        assert_eq!(*btree.get(&10).unwrap(), 11);
        assert_eq!(*btree.get(&11).unwrap(), 10);
        assert_eq!(*btree.get(&12).unwrap(), 13);
        assert_eq!(*btree.get(&13).unwrap(), 14);
        assert_eq!(*btree.get(&14).unwrap(), 15);
        assert_eq!(*btree.get(&15).unwrap(), 16);
    }

    #[test]
    fn insert() {
        let mut btree = BTree::new();

        btree.insert(1, 2);
        btree.insert(3, 4);
        btree.insert(4, 5);
        btree.insert(7, 8);
        btree.insert(2, 3);
        btree.insert(15, 16);
        btree.insert(8, 9);
        btree.insert(9, 10);
        btree.insert(10, 11);
        btree.insert(12, 13);
        btree.insert(11, 12);
        btree.insert(6, 7);
        btree.insert(11, 10);
        btree.insert(13, 14);
        btree.insert(5, 6);
        btree.insert(14, 15);

        assert_eq!(
            btree.iter().collect::<Vec<_>>(),
            vec![
                &(1, 2),
                &(2, 3),
                &(3, 4),
                &(4, 5),
                &(5, 6),
                &(6, 7),
                &(7, 8),
                &(8, 9),
                &(9, 10),
                &(10, 11),
                &(11, 10),
                &(12, 13),
                &(13, 14),
                &(14, 15),
                &(15, 16),
            ]
        );
    }

    #[test]
    fn remove() {
        let mut btree = BTree::new();

        for i in 1..=15 {
            btree.insert(i, i + 1);
        }

        assert_eq!(Some(2), btree.remove(&1));
        assert_eq!(Some(4), btree.remove(&3));
        assert_eq!(Some(5), btree.remove(&4));
        assert_eq!(Some(8), btree.remove(&7));
        assert_eq!(Some(3), btree.remove(&2));
        assert_eq!(Some(16), btree.remove(&15));
        assert_eq!(Some(9), btree.remove(&8));
        assert_eq!(Some(10), btree.remove(&9));
        assert_eq!(Some(11), btree.remove(&10));
        assert_eq!(Some(13), btree.remove(&12));
        assert_eq!(Some(7), btree.remove(&6));
        assert_eq!(Some(12), btree.remove(&11));
        assert_eq!(Some(14), btree.remove(&13));
        assert_eq!(Some(6), btree.remove(&5));
        assert_eq!(Some(15), btree.remove(&14));
    }

    #[test]
    fn pop_front() {
        let mut btree = BTree::new();

        btree.insert(1, 2);
        btree.insert(3, 4);
        btree.insert(4, 5);
        btree.insert(7, 8);
        btree.insert(2, 3);
        btree.insert(15, 16);
        btree.insert(8, 9);
        btree.insert(9, 10);
        btree.insert(10, 11);
        btree.insert(12, 13);
        btree.insert(11, 12);
        btree.insert(6, 7);
        btree.insert(13, 14);
        btree.insert(5, 6);
        btree.insert(14, 15);

        for i in 1..=15 {
            assert_eq!(Some((i, i + 1)), btree.pop_front());
        }
    }

    #[test]
    fn pop_back() {
        let mut btree = BTree::new();

        btree.insert(1, 2);
        btree.insert(3, 4);
        btree.insert(4, 5);
        btree.insert(7, 8);
        btree.insert(2, 3);
        btree.insert(15, 16);
        btree.insert(8, 9);
        btree.insert(9, 10);
        btree.insert(10, 11);
        btree.insert(12, 13);
        btree.insert(11, 12);
        btree.insert(6, 7);
        btree.insert(13, 14);
        btree.insert(5, 6);
        btree.insert(14, 15);

        for i in (1..=15).rev() {
            assert_eq!(Some((i, i + 1)), btree.pop_back());
        }
    }
}
