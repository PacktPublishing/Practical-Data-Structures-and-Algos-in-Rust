struct A(u32);

impl Drop for A {
    fn drop(&mut self) {
        println!("{} dropped", self.0);
    }
}

fn foo() -> A {
    println!("Entering foo");
    let a = A(2);
    println!("Returning from foo: {}", a.0);
    a
}

fn main() {
    println!("Entering main");
    let _a = foo();
    println!("After foo");
}
