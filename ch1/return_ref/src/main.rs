struct A(u32);

fn print_a(a: &A) {
    println!("A: {}", a.0);
}

fn return_ref<'a>(a: &'a A) -> &'a A {
    let a = A(15);
    &a
}

fn main() {
    let a = A(15);
    let ref_a = return_ref(&a);
    print_a(ref_a);
}
