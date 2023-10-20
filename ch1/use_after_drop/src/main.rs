struct A(u32);

impl Drop for A {
    fn drop(&mut self) {
        println!("{} dropped", self.0);
    }
}

fn main() {
    let a = A(4);
    std::mem::drop(a);
    println!("{}", a.0);
}
