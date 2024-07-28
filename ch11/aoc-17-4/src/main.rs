fn solve(input: &str) -> usize {
    input
        .lines()
        .filter(|phrase| {
            let phrase = phrase.trim().split(' ');

            phrase
                .clone()
                .all(|word| phrase.clone().filter(|w| *w == word).count() == 1)
        })
        .count()
}

fn main() {
    let input = include_str!("../input");
    let result = solve(input);

    println!("Result: {result}");
}
