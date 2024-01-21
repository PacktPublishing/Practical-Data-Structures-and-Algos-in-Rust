fn main() {
    let arr = [1, 2, 3, 4, 5];
    let arr_ref = &arr;
    let slice_ref: &[_] = &arr;

    println!("Arr ref size: {}", std::mem::size_of_val(&arr_ref));
    println!("Slice ref size: {}", std::mem::size_of_val(&slice_ref));
}
