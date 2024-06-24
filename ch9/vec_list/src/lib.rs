#![feature(test)]

#[cfg(test)]
extern crate test;

#[cfg(test)]
mod bench {
    use std::collections::{LinkedList, VecDeque};

    use test::{black_box, Bencher};

    const SIZE: usize = 1000;

    #[bench]
    fn push_back_vec(b: &mut Bencher) {
        b.iter(|| {
            let mut data = black_box(Vec::new());
            for i in 0..SIZE {
                data.push(i);
            }
        });
    }

    #[bench]
    fn push_back_vec_reserve(b: &mut Bencher) {
        b.iter(|| {
            let mut data = black_box(Vec::with_capacity(SIZE));
            for i in 0..SIZE {
                data.push(i);
            }
        });
    }

    #[bench]
    fn push_back_list(b: &mut Bencher) {
        b.iter(|| {
            let mut data = black_box(LinkedList::new());
            for i in 0..SIZE {
                data.push_back(i);
            }
        });
    }

    #[bench]
    fn push_back_deque(b: &mut Bencher) {
        b.iter(|| {
            let mut data = black_box(VecDeque::new());
            for i in 0..SIZE {
                data.push_back(i);
            }
        });
    }

    #[bench]
    fn push_back_deque_reserve(b: &mut Bencher) {
        b.iter(|| {
            let mut data = black_box(VecDeque::with_capacity(SIZE));
            for i in 0..SIZE {
                data.push_back(i);
            }
        });
    }

    #[bench]
    fn push_front_vec(b: &mut Bencher) {
        b.iter(|| {
            let mut data = black_box(Vec::new());
            for i in 0..SIZE {
                data.insert(0, i);
            }
        });
    }

    #[bench]
    fn push_front_vec_reserve(b: &mut Bencher) {
        b.iter(|| {
            let mut data = black_box(Vec::with_capacity(SIZE));
            for i in 0..SIZE {
                data.insert(0, i);
            }
        });
    }

    #[bench]
    fn push_front_list(b: &mut Bencher) {
        b.iter(|| {
            let mut data = black_box(LinkedList::new());
            for i in 0..SIZE {
                data.push_front(i);
            }
        });
    }

    #[bench]
    fn push_front_deque(b: &mut Bencher) {
        b.iter(|| {
            let mut data = black_box(VecDeque::new());
            for i in 0..SIZE {
                data.push_front(i);
            }
        });
    }

    #[bench]
    fn push_front_deque_reserve(b: &mut Bencher) {
        b.iter(|| {
            let mut data = black_box(VecDeque::with_capacity(SIZE));
            for i in 0..SIZE {
                data.push_front(i);
            }
        });
    }

    #[bench]
    fn push_front_big_vec(b: &mut Bencher) {
        b.iter(|| {
            let mut data = black_box(Vec::new());
            for i in 0..SIZE {
                data.insert(0, [i; 1000]);
            }
        });
    }

    #[bench]
    fn push_front_big_vec_boxed(b: &mut Bencher) {
        b.iter(|| {
            let mut data = black_box(Vec::new());
            for i in 0..SIZE {
                data.insert(0, Box::new([i; 1000]));
            }
        });
    }

    #[bench]
    fn push_front_big_vec_reserve(b: &mut Bencher) {
        b.iter(|| {
            let mut data = black_box(Vec::with_capacity(SIZE));
            for i in 0..SIZE {
                data.insert(0, [i; 1000]);
            }
        });
    }

    #[bench]
    fn push_front_big_vec_boxed_reserve(b: &mut Bencher) {
        b.iter(|| {
            let mut data = black_box(Vec::with_capacity(SIZE));
            for i in 0..SIZE {
                data.insert(0, Box::new([i; 1000]));
            }
        });
    }

    #[bench]
    fn push_front_big_list(b: &mut Bencher) {
        b.iter(|| {
            let mut data = black_box(LinkedList::new());
            for i in 0..SIZE {
                data.push_front([i; 1000]);
            }
        });
    }

    #[bench]
    fn push_front_big_deque(b: &mut Bencher) {
        b.iter(|| {
            let mut data = black_box(VecDeque::new());
            for i in 0..SIZE {
                data.push_front([i; 1000]);
            }
        });
    }

    #[bench]
    fn push_front_big_deque_reserve(b: &mut Bencher) {
        b.iter(|| {
            let mut data = black_box(VecDeque::with_capacity(SIZE));
            for i in 0..SIZE {
                data.push_front([i; 1000]);
            }
        });
    }

    #[bench]
    fn insert_vec(b: &mut Bencher) {
        b.iter(|| {
            let mut data = black_box(Vec::new());
            for i in 0..SIZE {
                data.insert(i / 2, i);
            }
        });
    }

    #[bench]
    fn insert_vec_reserve(b: &mut Bencher) {
        b.iter(|| {
            let mut data = black_box(Vec::with_capacity(SIZE));
            for i in 0..SIZE {
                data.insert(i / 2, i);
            }
        });
    }

    #[bench]
    fn insert_list(b: &mut Bencher) {
        b.iter(|| {
            let mut data = black_box(LinkedList::new());
            for i in 0..SIZE {
                let mut tail = data.split_off(i / 2);
                data.push_back(i);
                data.append(&mut tail);
            }
        });
    }

    #[bench]
    fn insert_deque(b: &mut Bencher) {
        b.iter(|| {
            let mut data = black_box(VecDeque::new());
            for i in 0..SIZE {
                data.insert(i / 2, i);
            }
        });
    }

    #[bench]
    fn insert_deque_reserve(b: &mut Bencher) {
        b.iter(|| {
            let mut data = black_box(VecDeque::with_capacity(SIZE));
            for i in 0..SIZE {
                data.insert(i / 2, i);
            }
        });
    }

    #[bench]
    fn pop_back_vec(b: &mut Bencher) {
        b.iter(|| {
            let mut data: Vec<_> = black_box(0..SIZE).collect();
            while !data.is_empty() {
                data.pop();
            }
        });
    }

    #[bench]
    fn pop_back_list(b: &mut Bencher) {
        b.iter(|| {
            let mut data: LinkedList<_> = black_box(0..SIZE).collect();
            while !data.is_empty() {
                data.pop_back();
            }
        });
    }

    #[bench]
    fn pop_back_deque(b: &mut Bencher) {
        b.iter(|| {
            let mut data: VecDeque<_> = black_box(0..SIZE).collect();
            while !data.is_empty() {
                data.pop_back();
            }
        });
    }

    #[bench]
    fn pop_front_vec(b: &mut Bencher) {
        b.iter(|| {
            let mut data: Vec<_> = black_box(0..SIZE).collect();
            while !data.is_empty() {
                black_box(data.remove(0));
            }
        });
    }

    #[bench]
    fn pop_front_vec_swap(b: &mut Bencher) {
        b.iter(|| {
            let mut data: Vec<_> = black_box(0..SIZE).collect();
            while !data.is_empty() {
                black_box(data.swap_remove(0));
            }
        });
    }

    #[bench]
    fn pop_front_list(b: &mut Bencher) {
        b.iter(|| {
            let mut data: LinkedList<_> = black_box(0..SIZE).collect();
            while !data.is_empty() {
                black_box(data.pop_front());
            }
        });
    }

    #[bench]
    fn pop_front_deque(b: &mut Bencher) {
        b.iter(|| {
            let mut data: VecDeque<_> = black_box(0..SIZE).collect();
            while !data.is_empty() {
                black_box(data.pop_front());
            }
        });
    }

    #[bench]
    fn iter_vec(b: &mut Bencher) {
        let data: Vec<_> = black_box(0..SIZE).collect();
        b.iter(|| {
            for i in &data {
                black_box(i);
            }
        });
    }

    #[bench]
    fn iter_list(b: &mut Bencher) {
        let data: LinkedList<_> = black_box(0..SIZE).collect();
        b.iter(|| {
            for i in &data {
                black_box(i);
            }
        });
    }

    #[bench]
    fn iter_deque(b: &mut Bencher) {
        let data: VecDeque<_> = black_box(0..SIZE).collect();
        b.iter(|| {
            for i in &data {
                black_box(i);
            }
        });
    }

    #[bench]
    fn access_vec(b: &mut Bencher) {
        let data: Vec<_> = black_box(0..SIZE).collect();
        b.iter(|| {
            for i in 0..SIZE {
                black_box(data[(i * 997) % SIZE]);
            }
        });
    }

    #[bench]
    fn access_list(b: &mut Bencher) {
        let data: LinkedList<_> = black_box(0..SIZE).collect();
        b.iter(|| {
            for i in 0..SIZE {
                black_box(data.iter().nth((i * 997) % SIZE).unwrap());
            }
        });
    }

    #[bench]
    fn access_deque(b: &mut Bencher) {
        let data: VecDeque<_> = black_box(0..SIZE).collect();
        b.iter(|| {
            for i in 0..SIZE {
                black_box(data[(i * 997) % SIZE]);
            }
        });
    }

    #[bench]
    fn iter_vec_big(b: &mut Bencher) {
        let data: Vec<_> = (0..SIZE).map(|i| [i; 1000]).collect();
        b.iter(|| {
            for i in &data {
                black_box(i[0]);
            }
        });
    }

    #[bench]
    fn iter_vec_boxed_big(b: &mut Bencher) {
        let data: Vec<_> = (0..SIZE).map(|i| Box::new([i; 1000])).collect();
        b.iter(|| {
            for i in &data {
                black_box(i[0]);
            }
        });
    }

    #[bench]
    fn iter_list_big(b: &mut Bencher) {
        let data: LinkedList<_> = (0..SIZE).map(|i| [i; 1000]).collect();
        b.iter(|| {
            for i in &data {
                black_box(i[0]);
            }
        });
    }

    #[bench]
    fn iter_deque_big(b: &mut Bencher) {
        let data: VecDeque<_> = (0..SIZE).map(|i| [i; 1000]).collect();
        b.iter(|| {
            for i in &data {
                black_box(i[0]);
            }
        });
    }
}
