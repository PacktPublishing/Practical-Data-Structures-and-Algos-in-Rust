fn main() {
    let layout = std::alloc::Layout::new::<u64>();
    // Safety: this is safe, because created layout is guaranteed to have size of a single u64,
    // that is non-zero.
    let ptr = unsafe { std::alloc::alloc(layout) };
    if ptr.is_null() {
        panic!("Allocation failed");
    }
    let u64_ptr = ptr as *mut u64;
    // Safety: this is safe, because the pointer is allocated with u64 layout
    unsafe { u64_ptr.write(5) };
    // Safety: this is safe, because the pointer is allocated with u64 layout, and is already
    // initialized.
    let u64_ref = unsafe { &*u64_ptr };
    println!("u64_ref: {}", *u64_ref);
    println!("data addr: {ptr:p}");
    println!("u64_ref addr: {u64_ref:p}");
    // Safety: this is safe, because the pointer is allocated with the exact same layout as passed
    unsafe { std::alloc::dealloc(ptr, layout) };
}
