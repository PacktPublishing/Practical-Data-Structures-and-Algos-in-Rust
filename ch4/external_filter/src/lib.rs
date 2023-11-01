fn is_even(n: &u64) -> bool {
    n % 2 == 0
}

pub fn evens(start: u64) -> impl Iterator<Item = u64> {
    (start..).filter(is_even)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify() {
        let result: Vec<_> = evens(5).take(6).collect();
        assert_eq!(result, vec![6, 8, 10, 12, 14, 16]);
    }
}
