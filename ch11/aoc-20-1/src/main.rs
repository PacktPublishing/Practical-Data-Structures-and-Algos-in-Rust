fn parse(input: &str) -> impl Iterator<Item = u64> + '_ {
    input.lines().filter_map(|n| n.parse().ok())
}

fn solve(numbers: &[u64]) -> u64 {
    numbers
        .iter()
        .flat_map(move |n| std::iter::repeat(n).zip(numbers))
        .filter(|&(m, n)| m != n)
        .find(|&(n, m)| n + m == 2020)
        .map(|(n, m)| n * m)
        .unwrap_or(0)
}

fn main() {
    let input = include_str!("../input");
    let input: Vec<_> = parse(input).collect();
    let result = solve(&input);

    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve() {
        let input = include_str!("../test");
        let input: Vec<_> = super::parse(input).collect();
        let result = super::solve(&input);

        assert_eq!(514579, result);
    }
}
