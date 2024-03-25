use std::mem::MaybeUninit;

fn basic() {
    let maybe_four = MaybeUninit::new(4);
    println!("{}", unsafe { maybe_four.assume_init() });

    let mut maybe_five = MaybeUninit::uninit();
    maybe_five.write(5);
    println!("{}", unsafe { maybe_five.assume_init() });
}

struct AnnounceDrop(u32);

impl Drop for AnnounceDrop {
    fn drop(&mut self) {
        println!("Dropping: {}", self.0);
    }
}

fn dropping() {
    let _uninit: MaybeUninit<AnnounceDrop> = MaybeUninit::uninit();
    let _maybe_one = MaybeUninit::new(AnnounceDrop(1));
    let mut maybe_two = MaybeUninit::uninit();
    maybe_two.write(AnnounceDrop(2));
    let maybe_three = MaybeUninit::new(AnnounceDrop(3));
    let _three = unsafe { maybe_three.assume_init() };
}

fn transmute_one() {
    let maybe_five = MaybeUninit::new(5u32);
    let five: u32 = unsafe { std::mem::transmute(maybe_five) };
    println!("{}", five)
}

fn transmute_array() {
    let maybe_arr: MaybeUninit<[u32; 100]> = MaybeUninit::uninit();
    let mut maybe_arr: [MaybeUninit<u32>; 100] = unsafe { std::mem::transmute(maybe_arr) };
    for (idx, item) in maybe_arr.iter_mut().enumerate() {
        item.write(idx as u32);
    }
    let arr: [u32; 100] = unsafe { std::mem::transmute(maybe_arr) };
    println!("{:?}", arr);
}

fn main() {
    basic();
    dropping();
    transmute_one();
    transmute_array();
}
