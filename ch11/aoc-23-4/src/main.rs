#![cfg_attr(feature = "bench", feature(test))]

use std::collections::VecDeque;

#[cfg(all(feature = "bench", test))]
extern crate test;

#[derive(Clone, Debug)]
struct Card {
    winning: Vec<u32>,
    scratched: Vec<u32>,
}

impl Card {
    fn parse(card: &str) -> Result<Self, std::num::ParseIntError> {
        let card = card
            .trim_start_matches(|c| c != ':')
            .trim_start_matches(':');

        let (winning, scratched) = card.split_once('|').unwrap();

        let winning = winning
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse())
            .collect::<Result<_, _>>()?;

        let scratched = scratched
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse())
            .collect::<Result<_, _>>()?;

        Ok(Card { winning, scratched })
    }

    fn score(self) -> usize {
        self.scratched
            .iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }

    #[allow(unused)]
    fn score_binsearch(mut self) -> usize {
        self.winning.sort_unstable();

        self.scratched
            .iter()
            .filter(|n| self.winning.binary_search(n).is_ok())
            .count()
    }
}

fn parse(input: &str) -> Result<Vec<Card>, std::num::ParseIntError> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(Card::parse)
        .collect()
}

#[allow(unused)]
fn collect_cards(cards: Vec<Card>) -> usize {
    let mut collected: Vec<_> = dbg!(std::iter::repeat(1).take(cards.len()).collect());

    for (idx, card) in cards.into_iter().enumerate() {
        let copies = collected[idx];
        let score = card.score();

        for scored in collected[idx + 1..][..score].iter_mut() {
            *scored += copies;
        }
    }

    collected.into_iter().sum()
}

fn collect_cards_iter(cards: impl IntoIterator<Item = Card>) -> impl Iterator<Item = usize> {
    let cards = cards.into_iter();
    cards
        .into_iter()
        .scan(VecDeque::with_capacity(20), |collected, card| {
            let copies = collected.pop_front().unwrap_or(0) + 1;
            let score = card.score();
            collected.resize(score.max(collected.len()), 0);
            for scored in collected.range_mut(..score) {
                *scored += copies;
            }

            Some(copies)
        })
}

fn solve(cards: impl IntoIterator<Item = Card>) -> usize {
    collect_cards_iter(cards).sum()
}

fn main() {
    let input = include_str!("../input");
    let cards = parse(input).unwrap();

    let result = solve(cards);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test");
    const INPUT: &str = include_str!("../input");

    #[test]
    fn score() {
        let input = dbg!(parse(TEST_INPUT)).unwrap();
        let expected = vec![4, 2, 2, 1, 0, 0];

        let actual = input.into_iter().map(Card::score).collect::<Vec<_>>();

        assert_eq!(expected, actual);
    }

    #[test]
    fn score_binsearch() {
        let input = parse(TEST_INPUT).unwrap();
        let expected = vec![4, 2, 2, 1, 0, 0];

        let actual = input
            .into_iter()
            .map(Card::score_binsearch)
            .collect::<Vec<_>>();

        assert_eq!(expected, actual);
    }

    #[test]
    fn collect_cards() {
        let input = parse(TEST_INPUT).unwrap();
        let expected = vec![1, 2, 4, 8, 14, 1];

        let actual: Vec<_> = super::collect_cards_iter(input.clone()).collect();
        assert_eq!(expected, actual);

        let actual = super::solve(input.clone());
        assert_eq!(30, actual);

        let actual = super::collect_cards(input);
        assert_eq!(30, actual);

        let input = parse(INPUT).unwrap();

        assert_eq!(5923918, super::collect_cards(input.clone()));
        assert_eq!(5923918, super::solve(input));
    }
}

#[cfg(all(test, feature = "bench"))]
mod bench {
    use test::{black_box, Bencher};

    use super::*;

    const INPUT: &str = include_str!("../input");

    #[bench]
    fn score(b: &mut Bencher) {
        let input = parse(INPUT).unwrap();

        b.iter(|| {
            input.clone().into_iter().map(Card::score).for_each(|s| {
                black_box(s);
            })
        })
    }

    #[bench]
    fn score_binsearch(b: &mut Bencher) {
        let input = parse(INPUT).unwrap();

        b.iter(|| {
            input
                .clone()
                .into_iter()
                .map(Card::score_binsearch)
                .for_each(|s| {
                    black_box(s);
                })
        })
    }

    #[bench]
    fn solve(b: &mut Bencher) {
        let input = parse(INPUT).unwrap();

        b.iter(|| collect_cards(input.clone()))
    }

    #[bench]
    fn solve_iter(b: &mut Bencher) {
        let input = parse(INPUT).unwrap();

        b.iter(|| super::solve(input.clone()))
    }
}
