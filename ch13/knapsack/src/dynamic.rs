pub fn knapsack(items: &[(usize, u64)], backpack: usize) -> Vec<usize> {
    let backpack = backpack as usize;
    let mut cache: Vec<(u64, Option<usize>)> = vec![];

    for w in 0..=backpack {
        let best = (0..items.len())
            .filter(|idx| items[*idx].0 <= w && items[*idx].0 > 0)
            .max_by_key(|idx| {
                let (weight, value) = items[*idx];
                cache[w - weight].0 + value
            });
        let value = best.map(|idx| items[idx].1).unwrap_or(0);
        cache.push((value, best))
    }

    std::iter::successors(Some(backpack), |w| {
        let added = cache[*w].1?;
        Some(*w - items[added].0)
    })
    .filter_map(|w| cache[w].1)
    .collect()
}

#[cfg(test)]
mod test {
    #[test]
    fn knapsack() {
        let items = vec![(5, 4), (4, 3), (3, 2), (2, 1)];
        let results = super::knapsack(&items, 18);
        let total = results.into_iter().fold((0, 0), |(weight, value), idx| {
            (weight + items[idx].0, value + items[idx].1)
        });

        assert!(total.0 <= 18);
        assert_eq!(total.1, 14);
    }
}
