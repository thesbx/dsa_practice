use std::time::Instant;
fn swap(vec: &mut [u8], i: usize, j: usize) {
    let temp = vec[i];
    vec[i] = vec[j];
    vec[j] = temp;
}

fn partition(vec: &mut [u8], lo: usize, hi: usize) -> u8 {
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
    return index as u8;
}

fn quicksort(vec: &mut [u8], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }
    let pivot = partition(vec, lo, hi);
    quicksort(vec, lo, (pivot - 1) as usize);
    quicksort(vec, (pivot + 1) as usize, hi);
}

fn random_vec(length: usize) -> Vec<u8> {
    let mut slice = Vec::new(); 
    for _ in 0..length {
        slice.push(rand::random::<u8>());
    }
    slice
}

fn main() {
    let mut vec = random_vec(100);
    let last_index = vec.len() - 1;
    let st = Instant::now();
    quicksort(&mut vec, 0, last_index);
    println!("{:?}", st.elapsed());
    println!("{:?}", vec);
}
