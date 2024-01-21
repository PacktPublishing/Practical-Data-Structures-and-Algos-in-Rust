fn main() {
    //let arr = Box::new([1, 2, 3, 4, 5]);
    let arr = Box::new([0; 8_000_000]);
    println!("{arr:?}")
}
