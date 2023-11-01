fn parse_line(input: String) -> Option<[u64; 3]> {
    let mut items = input.split('x');
    let w = items.next()?.parse().ok()?;
    let h = items.next()?.parse().ok()?;
    let d = items.next()?.parse().ok()?;

    Some([w, h, d])
}

fn faces(present: [u64; 3]) -> [u64; 3] {
    [
        present[0] * present[1],
        present[1] * present[2],
        present[0] * present[2],
    ]
}

fn paper_needed(faces: [u64; 3]) -> u64 {
    let min = *faces.iter().min().unwrap();
    let area: u64 = faces.into_iter().sum();
    2 * area + min
}

fn main() {
    let total: u64 = std::io::stdin()
        .lines()
        .map_while(Result::ok)
        .filter_map(parse_line)
        .map(faces)
        .map(paper_needed)
        .sum();

    println!("{total}")
}
