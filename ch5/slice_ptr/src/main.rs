fn main() {
    let arr = [1, 2, 3, 4, 5];
    let arr_ref = &arr;
    let slice_ref: &[_] = &arr;

    println!("Arr ref ptr: {:x?}", arr_ref as *const _ as usize);
    println!("Slice ref ptr: {:x?}", unsafe {
        std::mem::transmute::<_, [usize; 2]>(slice_ref)
    });
}
