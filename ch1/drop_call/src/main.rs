struct A(u32);

impl Drop for A {
    fn drop(&mut self) {
        println!("{} dropped", self.0);
    }
}

fn main() {
    let mut a = A(4);
    Drop::drop(&mut a);
}
