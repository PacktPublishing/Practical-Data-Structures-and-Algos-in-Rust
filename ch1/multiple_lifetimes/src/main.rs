fn a_or_b<'a, 'b>(a: &'a u32, b: &'b u32) -> &'a u32 {
    a
}

fn main() {
    println!("{}", a_or_b(&10, &15));
}
