pub struct Node<T> {
    value: T,
    next: Option<Box<Self>>,
}

pub struct LinkedList<T> {
    first: Option<Box<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { first: None }
    }

    pub fn push_front(&mut self, value: T) {
        let next = self.first.take();
        let node = Box::new(Node { value, next });

        self.first = Some(node);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let node = self.first.take()?;
        self.first = node.next;
        Some(node.value)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let mut current = self.first.as_ref()?;
        for _ in 0..index {
            current = match current.next {
                Some(ref next) => next,
                None => return None,
            };
        }

        Some(&current.value)
    }

    pub fn insert(&mut self, after: usize, value: T) {
        let Some(mut current) = self.first.as_mut() else {
            return self.push_front(value);
        };

        for _ in 0..after {
            current = match current.next {
                Some(ref mut next) => next,
                None => break,
            };
        }

        let node = Box::new(Node {
            value,
            next: current.next.take(),
        });

        current.next = Some(node);
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        let mut current = self.first.as_mut()?;
        for _ in 0..(index - 1) {
            current = match current.next {
                Some(ref mut next) => next,
                None => return None,
            };
        }

        let node = current.next.take()?;
        current.next = node.next;
        Some(node.value)
    }
}

pub struct LLIter<T>(LinkedList<T>);

impl<T> Iterator for LLIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl IntoIterator for LinkedList<i32> {
    type Item = i32;
    type IntoIter = LLIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        LLIter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_into_iter() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
}
