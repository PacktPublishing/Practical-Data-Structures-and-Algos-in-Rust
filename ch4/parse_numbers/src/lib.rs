pub fn parse_all(lines: impl IntoIterator<Item = String>) -> impl Iterator<Item = u64> {
    lines.into_iter().filter_map(|line| line.parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify() {
        let data = [
            "21".to_owned(),
            "5".to_owned(),
            "".to_owned(),
            "100".to_owned(),
            "Rust".to_owned(),
            "15".to_owned(),
        ];

        let numbers: Vec<_> = parse_all(data).collect();
        assert_eq!(numbers, vec![21, 5, 100, 15]);
    }
}
