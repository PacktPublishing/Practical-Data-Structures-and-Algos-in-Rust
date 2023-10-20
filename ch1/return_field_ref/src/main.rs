struct A(u32);

fn return_member<'a>(a: &'a A) -> &'a u32 {
    &a.0
}

fn main() {
    let a = A(20);
    let field = return_member(&a);
    println!("{field}");
}
