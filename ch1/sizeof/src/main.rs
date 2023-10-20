struct Foo {
    _a: u8,
    _b: u32,
    _c: u16,
}

fn main() {
    println!("{}", std::mem::size_of::<Foo>());
}
