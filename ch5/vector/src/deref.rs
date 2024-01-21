use super::Vector;

impl<T> std::ops::Deref for Vector<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> std::ops::DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deref() {
        let mut v = Vector::new();
        v.push(3);
        v.push(5);
        v.push(8);

        let r: &[u64] = &v;
        assert_eq!(r.len(), 3);
        assert_eq!(r[0], 3);
        assert_eq!(v[1], 5);
        assert_eq!(v[2], 8);
    }

    #[test]
    fn deref_mut() {
        let mut v = Vector::new();
        v.push(5);
        v[0] = 10;
        assert_eq!(v[0], 10);
    }
}
