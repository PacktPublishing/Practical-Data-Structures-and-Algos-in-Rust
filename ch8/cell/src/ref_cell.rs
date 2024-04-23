use crate::Cell;
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

pub struct Ref<'a, T> {
    counter: &'a Cell<isize>,
    item: &'a T,
}

impl<'a, T> Ref<'a, T> {
    pub fn clone(orig: &Self) -> Ref<'a, T> {
        let counter = orig.counter.replace(0);
        orig.counter.set(counter + 1);

        Ref {
            counter: orig.counter,
            item: orig.item,
        }
    }

    pub fn map<U>(orig: &Self, f: impl FnOnce(&T) -> &U) -> Ref<'a, U> {
        let counter = orig.counter.replace(0);
        orig.counter.set(counter + 1);

        Ref {
            counter: orig.counter,
            item: f(orig.item),
        }
    }

    pub fn map_split<U, V>(
        orig: &Self,
        f: impl FnOnce(&T) -> (&U, &V),
    ) -> (Ref<'a, U>, Ref<'a, V>) {
        let counter = orig.counter.replace(0);
        orig.counter.set(counter + 2);

        let (u, v) = f(orig.item);

        let u = Ref {
            counter: orig.counter,
            item: u,
        };

        let v = Ref {
            counter: orig.counter,
            item: v,
        };

        (u, v)
    }
}

impl<'a, T> Deref for Ref<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.item
    }
}

impl<'a, T> Drop for Ref<'a, T> {
    fn drop(&mut self) {
        let counter = self.counter.replace(0);
        self.counter.set(counter - 1);
    }
}

#[derive(Clone)]
pub struct RefMutCnt<'a>(&'a Cell<isize>);

impl<'a> Drop for RefMutCnt<'a> {
    fn drop(&mut self) {
        let cnt = self.0;
        let counter = cnt.replace(0);
        cnt.set(counter + 1);
    }
}

pub struct RefMut<'a, T> {
    counter: RefMutCnt<'a>,
    item: &'a mut T,
}

impl<'a, T> RefMut<'a, T> {
    pub fn map<U>(orig: Self, f: impl FnOnce(&'a mut T) -> &'a mut U) -> RefMut<'a, U> {
        let item = f(orig.item);
        RefMut {
            counter: orig.counter,
            item,
        }
    }

    pub fn map_split<U, V>(
        orig: Self,
        f: impl FnOnce(&'a mut T) -> (&'a mut U, &'a mut V),
    ) -> (RefMut<'a, U>, RefMut<'a, V>) {
        let counter = orig.counter.0.replace(0);
        let counter = counter - 1;

        if counter > 0 {
            panic!("Trying to borrow RefCell mutably, but too many borrows exist");
        }

        orig.counter.0.set(counter);

        let (u, v) = f(orig.item);

        let u = RefMut {
            counter: orig.counter.clone(),
            item: u,
        };
        let v = RefMut {
            counter: orig.counter,
            item: v,
        };

        (u, v)
    }
}

impl<'a, T> Deref for RefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.item
    }
}

impl<'a, T> DerefMut for RefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.item
    }
}

pub struct RefCell<T> {
    counter: Cell<isize>,
    item: UnsafeCell<T>,
}

impl<T> RefCell<T> {
    pub fn new(item: T) -> Self {
        Self {
            counter: 0.into(),
            item: item.into(),
        }
    }

    pub fn into_inner(self) -> T {
        self.item.into_inner()
    }

    pub fn swap(&self, other: &RefCell<T>) {
        let counter = self.counter.replace(0);
        if counter != 0 {
            panic!("Trying to swap RefCell while it is already borrowed");
        }

        let counter = other.counter.replace(0);
        if counter != 0 {
            panic!("Trying to swap RefCell while it is already borrowed");
        }

        unsafe { self.item.get().swap(other.item.get()) }
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        let counter = self.counter.replace(0);
        if counter < 0 {
            panic!("Trying to borrow RefCell while it is already borrowed for write");
        }

        let counter = counter + 1;
        if counter < 0 {
            panic!("Trying to borrow RefCell, but too many borrows exist");
        }

        self.counter.set(counter);

        Ref {
            counter: &self.counter,
            item: unsafe { &*self.item.get() },
        }
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        let counter = self.counter.replace(-1);
        if counter != 0 {
            panic!("Trying to mutably borrow RefCell while it is already borrowed")
        }

        RefMut {
            counter: RefMutCnt(&self.counter),
            item: unsafe { &mut *self.item.get() },
        }
    }

    pub fn try_borrow(&self) -> Option<Ref<'_, T>> {
        let counter = self.counter.replace(0);
        if counter < 0 {
            return None;
        }

        let counter = counter + 1;
        if counter < 0 {
            return None;
        }

        self.counter.set(counter);

        Some(Ref {
            counter: &self.counter,
            item: unsafe { &*self.item.get() },
        })
    }

    pub fn try_borrow_mut(&self) -> Option<RefMut<'_, T>> {
        let counter = self.counter.replace(-1);
        if counter != 0 {
            return None;
        }

        Some(RefMut {
            counter: RefMutCnt(&self.counter),
            item: unsafe { &mut *self.item.get() },
        })
    }
}

impl<T> From<T> for RefCell<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_inner() {
        let cell = RefCell::new(15);
        assert_eq!(cell.into_inner(), 15);
    }

    #[test]
    fn borrow() {
        let cell = RefCell::new(15);
        {
            let x = cell.borrow();
            assert_eq!(*x, 15);
        }
        assert_eq!(cell.into_inner(), 15);
    }

    #[test]
    fn borrow_map() {
        struct A {
            x: usize,
        }

        let cell = RefCell::new(A { x: 15 });
        {
            let x = cell.borrow();
            let y = Ref::map(&x, |a| &a.x);

            assert_eq!(*y, 15);
            assert_eq!(x.x, 15);
        }
        assert_eq!(cell.into_inner().x, 15);
    }

    #[test]
    fn borrow_map_split() {
        struct A {
            x: usize,
            y: usize,
        }

        let cell = RefCell::new(A { x: 15, y: 20 });
        {
            let a = cell.borrow();
            let (x, y) = Ref::map_split(&a, |a| (&a.x, &a.y));

            assert_eq!(*x, 15);
            assert_eq!(*y, 20);
            assert_eq!(a.x, 15);
            assert_eq!(a.y, 20);
        }

        let a = cell.into_inner();
        assert_eq!(a.x, 15);
        assert_eq!(a.y, 20);
    }

    #[test]
    fn borrow_mut() {
        let cell = RefCell::new(15);
        {
            *cell.borrow_mut() = 20;
        }
    }
}
