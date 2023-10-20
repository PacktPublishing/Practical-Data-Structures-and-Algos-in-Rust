struct A(u32);

fn print_a(a: A) {
    println!("A: {}", a.0);
}

fn main() {
    let a = A(10);
    print_a(a);
}
