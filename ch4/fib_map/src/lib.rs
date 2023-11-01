pub fn fib() -> impl Iterator<Item = u64> {
    std::iter::successors(Some((0, 1)), move |(n, m)| Some((*m, n + m))).map(|(_, m)| m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_12() {
        let result: Vec<_> = fib().take(10).collect();
        assert_eq!(result, vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);
    }
}
