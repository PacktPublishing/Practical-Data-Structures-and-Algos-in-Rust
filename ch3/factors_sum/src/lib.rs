#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(test, feature = "bench"))]
extern crate test;

pub fn sum_flat(data: &[[u64; 3]]) -> u64 {
    let mut sum = 0;

    for x in data {
        sum += x[0] * x[1] * x[2];
    }

    sum
}

pub fn sum_three_groups(data: &[Vec<u64>; 3]) -> u64 {
    let mut sum = 0;

    for idx in 0..data[0].len() {
        sum += data[0][idx] * data[1][idx] * data[2][idx];
    }

    sum
}

pub fn sum_groups_of_three(data: &[Box<[u64; 3]>]) -> u64 {
    let mut sum = 0;

    for x in data {
        sum += x[0] * x[1] * x[2];
    }

    sum
}

pub fn sum_of_singles(data: &[[Box<u64>; 3]]) -> u64 {
    let mut sum = 0;

    for x in data {
        sum += *x[0] * *x[1] * *x[2];
    }

    sum
}

#[cfg(all(test, feature = "bench"))]
mod bench {
    use super::*;
    use test::{black_box, Bencher};

    const SIZE: usize = 300;

    #[bench]
    fn flat(b: &mut Bencher) {
        let data = black_box(vec![[7, 13, 19]; SIZE]);
        b.iter(|| sum_flat(&data));
    }

    #[bench]
    fn little_big_groups(b: &mut Bencher) {
        let data = black_box([vec![7; SIZE], vec![13; SIZE], vec![19; SIZE]]);
        b.iter(|| sum_three_groups(&data));
    }

    #[bench]
    fn many_small_groups(b: &mut Bencher) {
        let data = black_box(vec![Box::new([7, 13, 19]); SIZE]);
        b.iter(|| sum_groups_of_three(&data));
    }

    #[bench]
    fn singles(b: &mut Bencher) {
        let data = black_box(vec![[Box::new(7), Box::new(13), Box::new(19)]; SIZE]);
        b.iter(|| sum_of_singles(&data));
    }
}
