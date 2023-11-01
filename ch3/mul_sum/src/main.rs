#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(test, feature = "bench"))]
extern crate test;

use rand::distributions::Standard;
use rand::Rng;
use std::time::Instant;

fn mul_sum(data: &[u64]) -> u64 {
    let mut sum = 0;

    for x in data {
        for y in data {
            sum += x * y;
        }
    }

    sum
}

fn main() {
    let rng = rand::thread_rng();
    let data: Vec<_> = rng.sample_iter(Standard).take(100000).collect();

    let start = Instant::now();
    let result = mul_sum(&data);
    let d = Instant::now() - start;

    println!("mul_sum(...) = {result} [{d:?}]");
}

#[cfg(all(test, feature = "bench"))]
mod bench {
    use super::*;
    use test::{black_box, Bencher};

    #[bench]
    fn mul_sum_1000(b: &mut Bencher) {
        let data: Vec<_> = black_box((1..=1000).collect());
        b.iter(|| mul_sum(&data));
    }

    #[bench]
    fn mul_sum_10000(b: &mut Bencher) {
        let data: Vec<_> = black_box((1..=10000).collect());
        b.iter(|| mul_sum(&data));
    }
}
