use std::cmp::Ordering;

pub fn binary_search<T: Ord>(arr: &[T], k: T) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let mid = arr.len() / 2;
    match arr[mid].cmp(&k) {
        Ordering::Equal => Some(mid),
        Ordering::Less => binary_search(&arr[mid + 1..], k).map(|idx| idx + mid + 1),
        Ordering::Greater => binary_search(&arr[..mid], k),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_search_test() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(binary_search(&arr, 5), Some(4));
        assert_eq!(binary_search(&arr, 10), None);
    }
}
