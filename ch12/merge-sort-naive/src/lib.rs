#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(test, feature = "bench"))]
extern crate test;

fn merge<T: Ord>(left: Vec<T>, right: Vec<T>) -> Vec<T> {
    let mut left = left.into_iter().peekable();
    let mut right = right.into_iter().peekable();
    let mut result = Vec::new();

    while let (Some(l), Some(r)) = (left.peek(), right.peek()) {
        if l <= r {
            let item = left.next().unwrap();
            result.push(item);
        } else {
            let item = right.next().unwrap();
            result.push(item);
        }
    }

    if left.peek().is_some() {
        result.extend(left);
    } else {
        result.extend(right);
    }

    result
}

pub fn sort<T: Ord>(mut arr: Vec<T>) -> Vec<T> {
    if arr.len() < 2 {
        arr
    } else {
        let mid = arr.len() / 2;
        let right = arr.split_off(mid);
        let left = sort(arr);
        let right = sort(right);
        merge(left, right)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sort() {
        let arr = vec![4, 2, 3, 1];
        let arr = super::sort(arr);
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
        b.iter(||super::sort(data.clone()));
    }

    #[bench]
    fn sort(b: &mut Bencher) {
        let data = input();
        b.iter(||super::sort(data.clone()));
    }
}
