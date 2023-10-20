fn copy(from: &[u32], to: &mut [u32]) {
    let end = from.len().min(to.len());

    for idx in 0..end {
        to[idx] = from[idx];
    }
}

fn main() {
    let source = [1, 2, 3, 4];
    let mut dest = [0; 4];
    copy(&source, &mut dest);
    println!("Copied: {dest:?}")
}
