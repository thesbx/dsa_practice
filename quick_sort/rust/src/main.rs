use std::{time::Instant, isize};
use sbx_common::generate_random_vec;

fn swap(vec: &mut [isize], i: usize, j: usize) {
    let temp = vec[i];
    vec[i] = vec[j];
    vec[j] = temp;
}

fn partition(vec: &mut [isize], lo: usize, hi: usize) -> isize {
    let pivot = vec[hi];
    let mut index = lo;
    let mut i = lo;

    while i < hi {
        if vec[i] < pivot {
            swap(vec, i, index);
            index += 1;
        }
        i += 1;
    }
    swap(vec, index, hi);
    return index as isize;
}

fn quicksort(vec: &mut [isize], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }
    let pivot = partition(vec, lo, hi);
    quicksort(vec, lo, (pivot - 1) as usize);
    quicksort(vec, (pivot + 1) as usize, hi);
}

fn main() {
    let mut vec = generate_random_vec(10);
    let last_index = vec.len() - 1;
    let st = Instant::now();
    quicksort(&mut vec, 0, last_index);
    println!("{:?}", st.elapsed());
    println!("{:?}", vec);
}
