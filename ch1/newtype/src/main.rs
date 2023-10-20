mod inner {
    pub struct Newtype(u8);

    impl Newtype {
        pub fn new() -> Self {
            Self(41)
        }
    }
}

fn main() {
    let _v = inner::Newtype::new();
}
