pub fn collect_results(
    it: impl IntoIterator<Item = Result<u64, String>>,
) -> Result<Vec<u64>, String> {
    it.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        let data = [Ok(2), Ok(5), Ok(6)];
        let result = collect_results(data).unwrap();
        assert_eq!(result, vec![2, 5, 6]);
    }

    #[test]
    fn err() {
        let data = [Ok(2), Err("Invalid value".to_owned()), Ok(6)];
        let result = collect_results(data).unwrap_err();
        assert_eq!(result, "Invalid value");
    }
}
