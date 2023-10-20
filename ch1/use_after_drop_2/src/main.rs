struct A(u32);

fn print_a(a: &A) {
    println!("A: {}", a.0);
}

fn main() {
    let a = A(10);
    let a_ref = &a;
    drop(a);
    print_a(a_ref);
}
