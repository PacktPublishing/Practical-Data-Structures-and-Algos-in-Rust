pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify() {
        assert_eq!(factorial(6), 720);
    }
}
