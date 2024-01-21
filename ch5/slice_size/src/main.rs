fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let slice1: &[_] = &arr[..];
    let slice2: &[_] = &arr[2..5];

    println!("Size of slice1: {}", std::mem::size_of_val(slice1));
    println!("Size of slice2: {}", std::mem::size_of_val(slice2));
}
