pub fn knapsack(items: &[(u64, u64)], backpack: u64) -> Vec<usize> {
    (0..items.len())
        .filter(|idx| items[*idx].0 <= backpack)
        .map(|idx| {
            let weight = items[idx].0;
            let mut result = knapsack(items, backpack - weight);
            result.push(idx);
            result
        })
        .max_by_key(|solution| solution.iter().map(|idx| items[*idx].1).sum::<u64>())
        .unwrap_or_default()
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
