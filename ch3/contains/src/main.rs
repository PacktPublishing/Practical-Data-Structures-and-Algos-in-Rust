#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(test, feature = "bench"))]
extern crate test;

use rand::distributions::Standard;
use rand::Rng;
use std::time::Instant;

fn contains(data: &[u64], elem: u64) -> bool {
    for x in data {
        if elem == *x {
            return true;
        }
    }

    false
}

fn main() {
    let mut rng = rand::thread_rng();
    let elem = rng.gen();
    let data: Vec<_> = rng.sample_iter(Standard).take(1000).collect();

    let start = Instant::now();
    let result = contains(&data, elem);
    let d = Instant::now() - start;

    println!("contains(...) = {result} [{d:?}]");
}

#[cfg(all(test, feature = "bench"))]
mod bench {
    use super::*;
    use test::Bencher;

    fn contains_bench(b: &mut Bencher, n: usize) {
        let mut rng = rand::thread_rng();
        let elem = rng.gen();
        let data: Vec<_> = rng.sample_iter(Standard).take(n).collect();

        b.iter(|| contains(&data, elem));
    }

    #[bench]
    fn contains_1000(b: &mut Bencher) {
        contains_bench(b, 1000);
    }

    #[bench]
    fn contains_10000(b: &mut Bencher) {
        contains_bench(b, 10000);
    }

    #[bench]
    fn contains_100000(b: &mut Bencher) {
        contains_bench(b, 100000);
    }
}
