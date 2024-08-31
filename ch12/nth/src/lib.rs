use std::fmt::Debug;

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

pub fn nth<T: Ord + Debug>(arr: &mut [T], idx: usize) {
    if arr.len() < 2 {
        return;
    }

    let pivot = pivot(arr);
    arr.swap(0, pivot);
    let pivot = reorganize(arr);

    match idx.cmp(&pivot) {
        std::cmp::Ordering::Less => nth(&mut arr[..pivot], idx),
        std::cmp::Ordering::Greater => nth(&mut arr[pivot + 1..], idx - pivot - 1),
        std::cmp::Ordering::Equal => (),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn nth() {
        let mut arr = [3, 2, 1, 5, 4];
        super::nth(&mut arr, 2);

        for x in &arr[..2] {
            assert!(*x <= arr[2])
        }

        for x in &arr[3..] {
            assert!(*x >= arr[2])
        }
    }
}
