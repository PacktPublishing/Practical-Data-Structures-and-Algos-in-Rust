pub fn try_sum(it: impl IntoIterator<Item = Result<u64, String>>) -> Result<u64, String> {
    it.into_iter().try_fold(0, |acc, x| -> Result<_, String> {
        let x = x?;
        Ok(acc + x)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        let data = [Ok(2), Ok(5), Ok(6)];
        let result = try_sum(data).unwrap();
        assert_eq!(result, 13);
    }

    #[test]
    fn err() {
        let data = [Ok(2), Err("Invalid value".to_owned()), Ok(6)];
        let result = try_sum(data).unwrap_err();
        assert_eq!(result, "Invalid value");
    }
}
