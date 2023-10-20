fn set<'a>(arr: &'a mut [u32], idx: usize, val: u32) -> &'a u32 {
    arr[idx] = val;
    &arr[idx]
}

fn main() {
    let mut arr = [1, 2, 3];
    let one = set(&mut arr, 1, 5);
    let zero = &arr[0];
    drop(one);
}
