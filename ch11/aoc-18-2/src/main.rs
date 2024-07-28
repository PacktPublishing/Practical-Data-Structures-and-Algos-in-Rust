fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn has_n(id: &str, n: usize) -> bool {
    id.chars().any(|c| id.matches(c).count() == n)
}

fn count_n(ids: &[&str], n: usize) -> usize {
    ids.iter().filter(|id| has_n(id, n)).count()
}

fn solve(ids: &[&str]) -> usize {
    count_n(ids, 2) * count_n(ids, 3)
}

fn main() {
    let input = include_str!("../input");
    let ids = parse(input);
    let result = solve(&ids);

    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = include_str!("../test");
        let ids = parse(input);
        let result = super::solve(&ids);
        assert_eq!(12, result);

        let input = include_str!("../input");
        let ids = parse(input);
        let result = super::solve(&ids);
        assert_eq!(5727, result);
    }
}
