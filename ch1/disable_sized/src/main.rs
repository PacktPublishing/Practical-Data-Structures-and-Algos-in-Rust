fn foo<T: ?Sized>(_x: &T) {}

fn main() {
    let arr = [1, 2, 3];
    let slice: &[_] = &arr;
    foo(slice);
}
