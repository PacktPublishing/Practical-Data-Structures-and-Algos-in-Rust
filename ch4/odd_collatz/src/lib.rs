pub fn odd_collatz(n: u64) -> impl Iterator<Item = u64> {
    std::iter::successors(Some(n), |prev| match prev {
        1 => None,
        prev if prev % 2 == 0 => Some(prev / 2),
        _ => Some(prev * 3 + 1),
    })
    .filter(|x| x % 2 != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collatz_12() {
        let result: Vec<_> = odd_collatz(12).collect();
        assert_eq!(result, vec![3, 5, 1]);
    }
}
