fn main() {
    let a = Box::new(15);
    let a_ref = a.as_ref();
    drop(a);
    println!("{a_ref}");
}
