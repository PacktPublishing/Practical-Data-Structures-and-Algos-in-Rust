fn take_iter(_: impl Iterator) {}

fn main() {
    let v = vec![1, 2, 3, 4];
    take_iter(v);
}
