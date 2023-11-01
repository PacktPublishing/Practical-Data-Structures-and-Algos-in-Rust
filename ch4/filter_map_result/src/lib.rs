pub fn is_even_result(
    it: impl IntoIterator<Item = Result<u64, String>>,
) -> impl Iterator<Item = Result<u64, String>> {
    it.into_iter().filter_map(|i| {
        i.map(|i| match i % 2 {
            0 => Some(i * i),
            _ => None,
        })
        .transpose()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify() {
        let data = [Ok(2), Err("Invalid data".to_owned()), Ok(3)];
        let result: Vec<_> = is_even_result(data).collect();
        assert_eq!(result, vec![Ok(4), Err("Invalid data".to_owned())]);
    }
}
