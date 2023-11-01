pub fn range(mut start: u64, end: u64) -> impl Iterator<Item = u64> {
    std::iter::from_fn(move || {
        if start < end {
            let val = start;
            start += 1;
            Some(val)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_to_ten() {
        let std_range: Vec<_> = (1..10).collect();
        let my_range: Vec<_> = range(1, 10).collect();
        assert_eq!(std_range, my_range);
    }

    #[test]
    fn empty() {
        let std_range: Vec<_> = (10..10).collect();
        let my_range: Vec<_> = range(10, 10).collect();
        assert_eq!(std_range, my_range);
    }

    #[test]
    fn start_after_end() {
        let my_range: Vec<_> = range(11, 10).collect();
        assert!(my_range.is_empty());
    }
}
