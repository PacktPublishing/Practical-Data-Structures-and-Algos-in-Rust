fn main() {
    let arr = [1, 2, 3, 4, 5];
    println!("Size of arr: {}", std::mem::size_of_val(&arr));
    println!("Size of [i64; 5]: {}", std::mem::size_of::<[i64; 5]>());
}
