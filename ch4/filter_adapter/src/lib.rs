pub fn evens(it: impl IntoIterator<Item = u64>) -> impl Iterator<Item = u64> {
    it.into_iter().filter(|x| x % 2 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify() {
        let result: Vec<_> = evens(5..).take(6).collect();
        assert_eq!(result, vec![6, 8, 10, 12, 14, 16]);
    }

    #[test]
    fn vec() {
        let result: Vec<_> = evens(vec![1, 2, 3, 5, 6, 9, 10]).take(6).collect();
        assert_eq!(result, vec![2, 6, 10]);
    }
}
