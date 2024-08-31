#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(test, feature = "bench"))]
extern crate test;

pub fn sort<T: Ord>(arr: &mut [T]) {
    for idx in 0..(arr.len() - 1) {
        let (next, _) = arr[idx..]
            .iter()
            .enumerate()
            .min_by_key(|(_, val)| *val)
            .unwrap();

        arr.swap(idx, idx + next);
    }
}

pub fn sort_stable<T: Ord>(arr: &mut [T]) {
    for idx in 0..(arr.len() - 1) {
        let (next, _) = arr[idx..]
            .iter()
            .enumerate()
            .min_by_key(|(_, val)| *val)
            .unwrap();

        arr[idx..][..=next].rotate_right(1);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sort() {
        let mut arr = [4, 2, 3, 1];
        super::sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4]);
    }

    #[test]
    fn sort_stable() {
        let mut arr = [4, 2, 3, 1];
        super::sort_stable(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4]);
    }
}

#[cfg(all(test, feature = "bench"))]
mod bench {
    use test::Bencher;

    fn input100() -> Vec<u64> {
        include_str!("../../numbers100")
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect()
    }

    fn input() -> Vec<u64> {
        include_str!("../../numbers")
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect()
    }

    #[bench]
    fn sort100(b: &mut Bencher) {
        let data = input100();
        b.iter(|| {
            let mut data = data.clone();
            super::sort(&mut data);
            data
        });
    }

    #[bench]
    fn sort_stable100(b: &mut Bencher) {
        let data = input100();
        b.iter(|| {
            let mut data = data.clone();
            super::sort_stable(&mut data);
            data
        });
    }

    #[bench]
    fn sort(b: &mut Bencher) {
        let data = input();
        b.iter(|| {
            let mut data = data.clone();
            super::sort(&mut data);
            data
        });
    }

    #[bench]
    fn sort_stable(b: &mut Bencher) {
        let data = input();
        b.iter(|| {
            let mut data = data.clone();
            super::sort_stable(&mut data);
            data
        });
    }
}
