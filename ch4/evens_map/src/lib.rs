#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(test, feature = "bench"))]
extern crate test;

pub fn evens(start: u64) -> impl Iterator<Item = u64> {
    ((start + 1) / 2..).map(|x| x * 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify() {
        let result: Vec<_> = evens(5).take(6).collect();
        assert_eq!(result, vec![6, 8, 10, 12, 14, 16]);
    }
}

#[cfg(all(test, feature = "bench"))]
mod bench {
    use super::*;
    use test::{black_box, Bencher};

    const SIZE: usize = 100000;

    #[bench]
    fn lot_evens(b: &mut Bencher) {
        b.iter(|| {
            for x in evens(0).take(SIZE) {
                let _ = black_box(x);
            }
        });
    }
}
