fn fac(n: u8) -> u8 {
    match n {
        0 | 1 => 1,
        n => {
            let a = fac(n - 1);
            a * n
        }
    }
}

fn main() {
    println!("fac(4): {}", fac(4));
}
