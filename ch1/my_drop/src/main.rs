struct A(u32);

impl Drop for A {
    fn drop(&mut self) {
        println!("{} dropped", self.0);
    }
}

fn my_drop<T>(_: T) {}

fn main() {
    println!("Entering main");
    let a = A(3);
    println!("Dropping {}", a.0);
    my_drop(a);
    println!("After drop");
}
