pub fn knapsack(items: &[(usize, u64)], backpack: usize) -> Vec<usize> {
    let mut cache = vec![(None, vec![]); backpack + 1];
    knapsack_impl(items, backpack, &mut cache).1
}

fn knapsack_impl(items: &[(usize, u64)], backpack: usize, cache: &mut [(Option<u64>, Vec<usize>)]) -> (u64, Vec<usize>) {
    if let (Some(value), result) = &cache[backpack] {
        (*value, result.clone())
    } else {
        let best = (0..items.len())
            .filter(|idx| items[*idx].0 <= backpack && items[*idx].0 > 0)
            .map(|idx| {
                let (weight, value) = items[idx];
                let (sub_value, mut result) = knapsack_impl(items, backpack - weight, cache);
                result.push(idx);
                (sub_value + value, result)
            })
            .max_by_key(|(value, _)| *value)
            .unwrap_or_default();

        cache[backpack] = (Some(best.0), best.1.clone());
        best
    }
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
