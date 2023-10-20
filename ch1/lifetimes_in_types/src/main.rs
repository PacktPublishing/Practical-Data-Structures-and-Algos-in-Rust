struct BorrowingInt<'a> {
    r: &'a u32,
}

fn main() {
    let a = 15;
    let _bi = BorrowingInt { r: &a };
}
