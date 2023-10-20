struct BorrowingInt<'a> {
    r: &'a u32,
}

fn create_borrowing_int<'a>(a: &'a u32) -> BorrowingInt<'a> {
    BorrowingInt { r: a }
}

fn main() {
    let a = 22;
    let _bi = create_borrowing_int(&a);
}
