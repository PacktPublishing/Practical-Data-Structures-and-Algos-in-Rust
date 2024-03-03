use std::collections::hash_map::RandomState;
use std::hash::BuildHasher;

fn main() {
    let input = "Some input string";

    let state = RandomState::default();
    let hash = state.hash_one(input);

    println!("{}", hash);
}
