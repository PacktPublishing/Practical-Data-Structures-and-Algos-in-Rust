fn a_or_b<'a, 'b>(a: &'a u32, b: &'b u32) -> &'a u32 {
    b
}

fn proxy<'a>(a: &'a u32) -> &'a u32 {
    let b = 25;
    a_or_b(a, &b)
}

fn main() {
    let a = 20;
    let ret = proxy(&a);
    println!("{ret}");
}
