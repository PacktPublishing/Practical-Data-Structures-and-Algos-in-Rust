use std::cell::UnsafeCell;

pub struct Cell<T> {
    item: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(item: T) -> Self {
        Self { item: item.into() }
    }

    pub fn into_inner(self) -> T {
        self.item.into_inner()
    }

    pub fn set(&self, mut val: T) {
        unsafe { self.item.get().swap(&mut val) }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.item.get() }
    }

    pub fn replace(&self, mut val: T) -> T {
        unsafe { self.item.get().swap(&mut val) }
        val
    }

    pub fn swap(&self, other: &Cell<T>) {
        unsafe { self.item.get().swap(other.item.get()) }
    }
}

impl<T> From<T> for Cell<T> {
    fn from(value: T) -> Self {
        Cell::new(value)
    }
}

impl<T> Clone for Cell<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        let t = unsafe { (*self.item.get()).clone() };
        Cell::new(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_inner() {
        let cell = Cell::new(15);
        assert_eq!(cell.into_inner(), 15);
    }

    #[test]
    fn set() {
        let cell = Cell::new(15);
        cell.set(20);
        assert_eq!(cell.into_inner(), 20);
    }

    #[test]
    fn get() {
        let cell = Cell::new(15);
        let got = cell.get();
        assert_eq!(got, 15);
        assert_eq!(cell.into_inner(), 15);
    }

    #[test]
    fn replace() {
        let cell = Cell::new(15);
        let x = cell.replace(20);
        assert_eq!(x, 15);
        assert_eq!(cell.into_inner(), 20);
    }

    #[test]
    fn swap() {
        let cell1 = Cell::new(15);
        let cell2 = Cell::new(20);
        cell1.swap(&cell2);
        assert_eq!(cell1.into_inner(), 20);
        assert_eq!(cell2.into_inner(), 15);
    }

    #[test]
    fn clone() {
        let cell1 = Cell::new(15);
        let cell2 = cell1.clone();
        cell1.set(20);
        assert_eq!(cell1.into_inner(), 20);
        assert_eq!(cell2.into_inner(), 15);
    }
}
