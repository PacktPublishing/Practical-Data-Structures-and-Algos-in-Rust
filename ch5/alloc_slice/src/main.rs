use std::alloc;

#[derive(Debug)]
struct U64WithDrop(u64);

impl Drop for U64WithDrop {
    fn drop(&mut self) {
        println!("Dropping: {}", self.0);
    }
}

fn main() {
    let layout = alloc::Layout::array::<U64WithDrop>(5).unwrap();
    // Safety: layout is space for five u64 values, non-zero
    let ptr = unsafe { alloc::alloc(layout) };
    if ptr.is_null() {
        panic!("Allocation failed");
    }
    let u64_ptr = ptr as *mut U64WithDrop;
    for i in 0..5 {
        // Safety: u64 is allocated for five u64 items, and `i` is never more than 4
        unsafe { u64_ptr.add(i).write(U64WithDrop(i as _)) };
    }
    // Safety: memory is allocated for five u64 elements, all of them being initialized
    let u64_slice = unsafe { std::slice::from_raw_parts_mut(u64_ptr, 5) };
    println!("Slice: {:?}", u64_slice);
    unsafe { std::ptr::drop_in_place(u64_slice) };
    unsafe { alloc::dealloc(ptr, layout) };
}
