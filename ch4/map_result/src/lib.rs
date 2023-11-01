pub fn square_result(
    it: impl IntoIterator<Item = Result<u64, String>>,
) -> impl Iterator<Item = Result<u64, String>> {
    it.into_iter().map(|i| i.map(|i| i * i))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify() {
        let data = [Ok(2), Err("Invalid data".to_owned()), Ok(3)];
        let result: Vec<_> = square_result(data).collect();
        assert_eq!(result, vec![Ok(4), Err("Invalid data".to_owned()), Ok(9)]);
    }
}
