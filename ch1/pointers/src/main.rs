fn main() {
    let integral = Box::new(127u32);
    let pointer = Box::into_raw(integral);
    println!("Pointer: {pointer:?}");
    println!("Value: {}", unsafe { *pointer });
}
