#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(test, feature = "bench"))]
extern crate test;

pub fn concat_slow(data: &[&str]) -> String {
    let mut res = String::new();

    for s in data {
        res = s.to_string() + res.as_str();
    }

    res
}

pub fn concat_fast(data: &[&str]) -> String {
    let mut res = String::new();

    for s in data.iter().rev() {
        res = res + s;
    }

    res
}

#[cfg(all(test, feature = "bench"))]
mod bench {
    use super::*;
    use test::{black_box, Bencher};

    const SIZE: usize = 1000;

    #[bench]
    fn slow(b: &mut Bencher) {
        let data: Vec<_> = black_box(vec!["foo"; SIZE]);
        b.iter(|| concat_slow(&data));
    }

    #[bench]
    fn fast(b: &mut Bencher) {
        let data: Vec<_> = black_box(vec!["foo"; SIZE]);
        b.iter(|| concat_fast(&data));
    }
}
