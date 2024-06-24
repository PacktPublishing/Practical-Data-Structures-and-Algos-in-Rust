use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Self>>>,
    prev: Weak<RefCell<Self>>,
}

pub struct LinkedList<T> {
    first: Option<Rc<RefCell<Node<T>>>>,
    last: Weak<RefCell<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            first: None,
            last: Weak::new(),
        }
    }

    pub fn push_back(&mut self, value: T) {
        let node = Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: self.last.clone(),
        }));

        if let Some(last) = node.borrow().prev.upgrade() {
            last.borrow_mut().next = Some(Rc::clone(&node));
        } else {
            self.first = Some(Rc::clone(&node));
        }

        self.last = Rc::downgrade(&node);
    }

    pub fn push_front(&mut self, value: T) {
        let node = Rc::new(RefCell::new(Node {
            value,
            next: self.first.take(),
            prev: Weak::new(),
        }));

        if let Some(first) = &node.borrow().next {
            first.borrow_mut().prev = Rc::downgrade(&node);
        } else {
            self.last = Rc::downgrade(&node);
        }

        self.first = Some(Rc::clone(&node));
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let last = self.last.upgrade()?;

        if let Some(prev) = last.borrow().prev.upgrade() {
            prev.borrow_mut().next = None;
            self.last = Rc::downgrade(&prev);
        } else {
            self.first = None;
            self.last = Weak::new();
        }

        let value = Rc::into_inner(last)?.into_inner().value;
        Some(value)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let first = self.first.take()?;

        if let Some(next) = &first.borrow().next {
            next.borrow_mut().prev = Weak::new();
            self.first = Some(Rc::clone(next));
        } else {
            self.last = Weak::new();
        }

        let value = Rc::into_inner(first)?.into_inner().value;
        Some(value)
    }
}

pub struct LLIter<T>(LinkedList<T>);

impl<T> Iterator for LLIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl DoubleEndedIterator for LLIter<i32> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
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
    fn test_push_back() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let iter = list.first.as_ref().unwrap().borrow();
        assert_eq!(iter.value, 1);
        assert_eq!(iter.next.as_ref().unwrap().borrow().value, 2);
        assert_eq!(
            iter.next
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .value,
            3
        );
    }

    #[test]
    fn test_push_front() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let iter = list.first.as_ref().unwrap().borrow();
        assert_eq!(iter.value, 3);
        assert_eq!(iter.next.as_ref().unwrap().borrow().value, 2);
        assert_eq!(
            iter.next
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .value,
            1
        );
    }

    #[test]
    fn test_pop_back() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_pop_front() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_into_iter() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_double_ended_iter() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
}
