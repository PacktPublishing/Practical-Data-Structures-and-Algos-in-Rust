#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(test, feature = "bench"))]
extern crate test;

fn merge<T: Ord + Copy>(left: &[T], right: &[T], dst: &mut [T]) {
    debug_assert_eq!(left.len() + right.len(), dst.len());

    let mut left = left.iter().copied().peekable();
    let mut right = right.iter().copied().peekable();
    let mut dst = dst.iter_mut();

    while let (Some(l), Some(r)) = (left.peek(), right.peek()) {
        if l <= r {
            *dst.next().unwrap() = left.next().unwrap();
        } else {
            *dst.next().unwrap() = right.next().unwrap();
        }
    }

    if left.peek().is_some() {
        for (l, d) in left.zip(dst) {
            *d = l;
        }
    } else {
        for (r, d) in right.zip(dst) {
            *d = r;
        }
    }
}

fn merge_sort<T: Ord + Copy>(src: &mut [T], dst: &mut [T], rev: bool) {
    debug_assert_eq!(src.len(), dst.len());

    if src.len() > 1 {
        let mid = src.len() / 2;
        let (left, right) = src.split_at_mut(mid);
        let (leftd, rightd) = dst.split_at_mut(mid);

        merge_sort(left, leftd, !rev);
        merge_sort(right, rightd, !rev);

        if rev {
            merge(left, right, dst);
        } else {
            merge(leftd, rightd, src);
        }
    }
}

pub fn sort<T: Ord + Copy>(arr: &mut [T]) {
    let mut dst = arr.to_owned();
    merge_sort(arr, &mut dst, true);
    arr.copy_from_slice(&dst);
}

#[cfg(test)]
mod tests {
    #[test]
    fn sort() {
        let mut arr = [
            4, 18, 3, 1, 5, 13, 7, 11, 12, 10, 9, 14, 6, 15, 17, 16, 2, 8,
        ];
        super::sort(&mut arr);
        assert_eq!(
            arr,
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]
        );
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
    fn sort(b: &mut Bencher) {
        let data = input();
        b.iter(|| {
            let mut data = data.clone();
            super::sort(&mut data);
            data
        });
    }
}
