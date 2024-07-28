fn solve_race(time: u64, distance: u64) -> usize {
    (0..time)
        .map(|t| t * (time - t))
        .filter(|d| *d > distance)
        .count()
}

fn bisect(xs: (u64, u64), f: impl Fn(u64) -> std::cmp::Ordering) -> u64 {
    use std::cmp::Ordering::*;

    std::iter::successors(Some(xs), |&(low, high)| {
        let x = (low + high) / 2;
        match f(x) {
            Less | Equal => Some((x, high)),
            Greater => Some((low, x)),
        }
    })
    .find(|&(low, high)| low + 1 == high || high + 1 == low)
    .unwrap()
    .1
}

fn solve_race_bisect(time: u64, distance: u64) -> u64 {
    let dist = |t| t * (time - t);

    let low = bisect((0, time / 2), |t| dist(t).cmp(&distance));
    let high = bisect((time, time / 2), |t| dist(t).cmp(&distance));

    high - low + 1
}

#[allow(unused)]
fn solve(times: impl IntoIterator<Item = u64>, dists: impl IntoIterator<Item = u64>) -> usize {
    times
        .into_iter()
        .zip(dists)
        .map(|(t, d)| solve_race(t, d))
        .product()
}

fn solve_bisect(times: impl IntoIterator<Item = u64>, dists: impl IntoIterator<Item = u64>) -> u64 {
    times
        .into_iter()
        .zip(dists)
        .map(|(t, d)| solve_race_bisect(t, d))
        .product()
}

fn main() {
    let times = [57726992];
    let dists = [291117211762026];

    let result = solve_bisect(times, dists);

    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve() {
        let times = [7, 15, 30];
        let dists = [9, 40, 200];

        let result = super::solve(times, dists);
        assert_eq!(288, result);

        let times = [57, 72, 69, 92];
        let dists = [291, 1172, 1176, 2026];

        let result = super::solve(times, dists);
        assert_eq!(160816, result);

        let times = [57726992];
        let dists = [291117211762026];

        let result = super::solve(times, dists);
        assert_eq!(46561107, result);
    }

    #[test]
    fn solve_bisect() {
        let times = [7, 15, 30];
        let dists = [9, 40, 200];

        let result = super::solve_bisect(times, dists);
        assert_eq!(288, result);

        let times = [57, 72, 69, 92];
        let dists = [291, 1172, 1176, 2026];

        let result = super::solve_bisect(times, dists);
        assert_eq!(160816, result);

        let times = [57726992];
        let dists = [291117211762026];

        let result = super::solve_bisect(times, dists);
        assert_eq!(46561107, result);
    }
}
