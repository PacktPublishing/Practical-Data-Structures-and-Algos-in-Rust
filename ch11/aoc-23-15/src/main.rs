fn parse(input: &str) -> impl Iterator<Item = &str> {
    input.trim().split(',')
}

fn hash(value: &str) -> u8 {
    value
        .bytes()
        .reduce(|curr, c| curr.wrapping_add(c).wrapping_mul(17))
        .unwrap_or(0)
}

fn solve<'a, I>(input: I) -> u64
where
    I: IntoIterator<Item = &'a str>,
{
    input.into_iter().map(hash).map(u64::from).sum()
}

fn main() {
    let input = include_str!("../input");
    let result = solve(parse(input));
    println!("Result: {result}");
}
