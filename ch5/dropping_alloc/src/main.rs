#[derive(Debug)]
struct U64WithDrop(u64);

impl Drop for U64WithDrop {
    fn drop(&mut self) {
        println!("Dropping: {}", self.0);
    }
}

fn main() {
    println!("Pre-allocation");
    let layout = std::alloc::Layout::new::<U64WithDrop>();
    // Safety: this is safe, because created layout is guaranteed to have size of a single u64,
    // that is non-zero.
    let ptr = unsafe { std::alloc::alloc(layout) };
    if ptr.is_null() {
        panic!("Allocation failed");
    }
    println!("{ptr:p} allocated");
    let u64_ptr = ptr as *mut U64WithDrop;
    // Safety: this is safe, because the pointer is allocated with proper layout
    unsafe { u64_ptr.write(U64WithDrop(5)) };
    // Safety: this is safe, because the pointer is allocated with proper layout, and is already
    // initialized.
    let u64_ref = unsafe { &*u64_ptr };
    println!("{ptr:p} initialized with {:?}", *u64_ref);
    // Safety: this is safe, because the pointer is allocated and initialized, and not dropped
    // before
    unsafe { ptr.drop_in_place() };
    // Safety: this is safe, because the pointer is allocated with the exact same layout as passed
    unsafe { std::alloc::dealloc(ptr, layout) };
    println!("{ptr:p} released");
}
