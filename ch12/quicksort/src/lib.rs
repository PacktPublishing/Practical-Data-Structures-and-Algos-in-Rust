#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(test, feature = "bench"))]
extern crate test;

fn pivot<T: Ord>(arr: &[T]) -> usize {
    let end = arr.len() - 1;
    let mid = arr.len() / 2;

    let a = &arr[0];
    let b = &arr[mid];
    let c = &arr[end];

    if a <= b && b <= c || c <= b && b <= a {
        mid
    } else if b <= a && a <= c || c <= a && a <= b {
        0
    } else {
        end
    }
}

fn reorganize_step<T: Ord>(arr: &mut [T]) -> (usize, usize) {
    let (pivot, rest) = arr.split_at_mut(1);
    let high = rest.iter().rev().position(|x| *x < pivot[0]);

    let high = match high {
        Some(high) => {
            let high = rest.len() - high - 1;
            std::mem::swap(&mut pivot[0], &mut rest[high]);
            high
        }
        None => return (0, 0),
    };

    let (rest, pivot) = rest.split_at_mut(high);
    let low = rest.iter().position(|x| *x > pivot[0]);

    let low = match low {
        Some(low) => {
            std::mem::swap(&mut pivot[0], &mut rest[low]);
            low
        }
        None => return (high + 1, high + 1),
    };

    (low + 1, high)
}

fn reorganize<T: Ord>(arr: &mut [T]) -> usize {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low < high {
        let (nlow, nhigh) = reorganize_step(&mut arr[low..=high]);
        (low, high) = (nlow + low, nhigh + low);
    }

    low
}

pub fn sort<T: Ord>(arr: &mut [T]) {
    if arr.len() < 2 {
        return;
    }

    let pivot = pivot(arr);
    arr.swap(0, pivot);
    let pivot = reorganize(arr);

    sort(&mut arr[..pivot]);
    sort(&mut arr[pivot + 1..]);
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
