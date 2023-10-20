struct A(u32);

impl Drop for A {
    fn drop(&mut self) {
        println!("{} dropped", self.0);
    }
}

fn foo(a: A) {
    println!("Entering foo: {}", a.0);
}

fn main() {
    println!("Entering main");
    let a = A(1);
    println!("Calling foo");
    foo(a);
    println!("After foo");
}
