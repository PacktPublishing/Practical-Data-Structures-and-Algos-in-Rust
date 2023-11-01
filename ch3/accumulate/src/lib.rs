#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(test, feature = "bench"))]
extern crate test;

pub fn accumulate(data: &[u64]) -> u64 {
    let mut acc = 0;

    for item in data {
        acc += *item;
    }

    acc
}

pub fn bad_accumulate(data: &[u64]) -> u64 {
    let mut acc = Box::new(0);

    for item in data {
        acc = Box::new(*acc * *item);
    }

    *acc
}

#[cfg(all(test, feature = "bench"))]
mod bench {
    use super::*;
    use test::{black_box, Bencher};

    const SIZE: usize = 100000;

    #[bench]
    fn regular(b: &mut Bencher) {
        let data: Vec<_> = black_box(vec![1; SIZE]);
        b.iter(|| accumulate(&data));
    }

    #[bench]
    fn bad(b: &mut Bencher) {
        let data: Vec<_> = black_box(vec![1; SIZE]);
        b.iter(|| bad_accumulate(&data));
    }
}
