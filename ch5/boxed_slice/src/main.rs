fn main() {
    let array = [1, 2, 3];
    let boxed_slice: Box<[_]> = array.into();

    println!(
        "Boxed slice ptr size: {}",
        std::mem::size_of_val(&boxed_slice)
    );
}
