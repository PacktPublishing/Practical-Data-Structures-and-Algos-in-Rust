#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(feature = "bench", test))]
extern crate test;

use rayon::prelude::*;
use rayon::str::ParallelString;

fn parse_triple(line: &str) -> Option<[u64; 3]> {
    let mut sides = line
        .split_whitespace()
        .map(str::parse)
        .filter_map(Result::ok);
    let triangle = [sides.next()?, sides.next()?, sides.next()?];

    Some(triangle)
}

fn parse<'a, I>(input: I) -> impl ParallelIterator<Item = [u64; 3]>
where
    I: IntoParallelIterator<Item = &'a str>,
{
    input.into_par_iter().filter_map(parse_triple)
}

fn is_triangle([a, b, c]: [u64; 3]) -> bool {
    [(a, b, c), (b, a, c), (c, a, b)]
        .into_iter()
        .all(|(a, b, c)| a < b + c)
}

fn solve(triangles: impl IntoParallelIterator<Item = [u64; 3]>) -> usize {
    triangles
        .into_par_iter()
        .filter(|t| is_triangle(*t))
        .count()
}

fn main() {
    let input = include_str!("../input");
    let triangles = parse(input.par_lines());
    let result = solve(triangles);

    println!("Result: {result}");
}

#[cfg(all(test, feature = "bench"))]
mod bench {
    use super::*;
    use test::{black_box, Bencher};

    #[bench]
    fn no_chaching(b: &mut Bencher) {
        let input = include_str!("../input");

        b.iter(|| {
            let triangles = parse(input.par_lines());
            solve(triangles)
        })
    }

    #[bench]
    fn lines_chaching(b: &mut Bencher) {
        let input = include_str!("../input");
        let input: Vec<_> = input.lines().collect();

        b.iter(|| {
            let triangles = parse(input.par_iter().copied());
            solve(triangles)
        })
    }

    #[bench]
    fn chaching(b: &mut Bencher) {
        let input = include_str!("../input");
        let triangles: Vec<_> = parse(input.par_lines()).collect();

        b.iter(|| solve(triangles.par_iter().copied()))
    }

    #[bench]
    fn chaching_scaled(b: &mut Bencher) {
        let input = include_str!("../input");
        let triangles: Vec<_> = parse(input.par_lines())
            .flat_map(|i| rayon::iter::repeat(i).take(1000))
            .collect();

        b.iter(|| solve(triangles.par_iter().copied()))
    }
}
