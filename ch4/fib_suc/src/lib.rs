pub fn fib() -> impl Iterator<Item = u64> {
    let mut prev = 0;

    std::iter::successors(Some(1), move |n| {
        let next = prev + n;
        prev = *n;
        Some(next)
    })
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
