use std::collections::VecDeque;

#[derive(Debug)]
struct Input {
    map: Vec<u8>,
    width: usize,
    start: usize,
    end: usize,
}

impl Input {
    fn parse(input: &str) -> Input {
        let map = input
            .lines()
            .flat_map(|line| line.bytes())
            .map(|c| match c {
                b'S' => b'a',
                b'E' => b'z',
                c => c,
            })
            .collect();
        let width = input.lines().next().unwrap().len();
        let start = input
            .lines()
            .flat_map(|line| line.bytes())
            .enumerate()
            .find_map(|(idx, c)| match c {
                b'S' => Some(idx),
                _ => None,
            })
            .unwrap();
        let end = input
            .lines()
            .flat_map(|line| line.bytes())
            .enumerate()
            .find_map(|(idx, c)| match c {
                b'E' => Some(idx),
                _ => None,
            })
            .unwrap();

        Input {
            map,
            width,
            start,
            end,
        }
    }
}

fn neighbours(input: &Input, idx: usize) -> [Option<usize>; 4] {
    let w = input.width;
    let stepable = |f, t| {
        let f: u8 = input.map[f];
        let t: u8 = input.map[t];
        f + 1 >= t
    };

    let up = if idx >= input.width && stepable(idx, idx - w) {
        Some(idx - w)
    } else {
        None
    };

    let right = if idx % w < w - 1 && stepable(idx, idx + 1) {
        Some(idx + 1)
    } else {
        None
    };

    let down = if idx < input.map.len() - input.width && stepable(idx, idx + w) {
        Some(idx + w)
    } else {
        None
    };

    let left = if idx % w > 0 && stepable(idx, idx - 1) {
        Some(idx - 1)
    } else {
        None
    };

    [up, right, down, left]
}

fn solve(input: Input) -> usize {
    let mut distances = vec![usize::MAX; input.map.len()];
    distances[input.start] = 0;

    let mut queue: VecDeque<usize> = std::iter::once(input.start).collect();

    while let Some(node) = queue.pop_front() {
        let d = distances[node];

        if node == input.end {
            return d;
        }

        for n in neighbours(&input, node).into_iter().flatten() {
            if d + 1 < distances[n] {
                distances[n] = d + 1;
                queue.push_back(n);
            }
        }
    }

    distances[input.end]
}

fn main() {
    let input = include_str!("../input");
    let input = Input::parse(input);
    let result = solve(input);

    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let input = include_str!("../test");
        let input = super::Input::parse(input);
        let result = super::solve(input);
        assert_eq!(result, 31);
    }

    #[test]
    fn input() {
        let input = include_str!("../input");
        let input = super::Input::parse(input);
        let result = super::solve(input);
        assert_eq!(result, 437);
    }
}
