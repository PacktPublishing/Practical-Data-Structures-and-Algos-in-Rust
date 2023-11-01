fn main() {
    let cnt = std::io::stdin()
        .lines()
        .map_while(Result::ok)
        .filter(|line| line.trim().is_empty())
        .count();

    println!("{cnt} blank lines in the input");
}
