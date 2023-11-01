pub fn collatz(n: u64) -> impl Iterator<Item = u64> {
    std::iter::successors(Some(n), |prev| match prev {
        1 => None,
        prev if prev % 2 == 0 => Some(prev / 2),
        _ => Some(prev * 3 + 1),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collatz_12() {
        let result: Vec<_> = collatz(12).collect();
        assert_eq!(result, vec![12, 6, 3, 10, 5, 16, 8, 4, 2, 1]);
    }
}
