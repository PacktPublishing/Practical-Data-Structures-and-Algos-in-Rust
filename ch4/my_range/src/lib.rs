#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(test, feature = "bench"))]
extern crate test;

pub struct MyRange {
    pub start: u64,
    pub end: u64,
}

impl MyRange {
    pub fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }
}

impl Iterator for MyRange {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let val = self.start;
            self.start += 1;
            Some(val)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterate() {
        let mut sum = 0;
        for i in MyRange::new(1, 5) {
            sum += i;
        }
        assert_eq!(sum, 10);
    }

    #[test]
    fn one_to_ten() {
        let range: Vec<_> = (1..10).collect();
        let my_range: Vec<_> = MyRange::new(1, 10).collect();
        assert_eq!(range, my_range);
    }

    #[test]
    fn empty() {
        let range: Vec<_> = (10..10).collect();
        let my_range: Vec<_> = MyRange::new(10, 10).collect();
        assert_eq!(range, my_range);
    }

    #[test]
    fn start_after_end() {
        let my_range: Vec<_> = MyRange::new(11, 10).collect();
        assert!(my_range.is_empty());
    }
}

#[cfg(all(test, feature = "bench"))]
mod bench {
    use super::*;
    use test::{black_box, Bencher};

    const SIZE: u64 = 100000;

    #[bench]
    fn iter_while(b: &mut Bencher) {
        b.iter(|| {
            let mut i = black_box(0);
            let limit = black_box(SIZE);

            while i < limit {
                i = black_box(i + 1);
            }

            i
        })
    }

    #[bench]
    fn my_range(b: &mut Bencher) {
        b.iter(|| {
            let mut i = black_box(0);
            let limit = black_box(SIZE);

            for v in black_box(MyRange::new(0, limit)) {
                i = black_box(v);
            }

            i
        })
    }
}
