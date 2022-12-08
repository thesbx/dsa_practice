use std::time::Instant;
fn swap(vec: &mut [i32], i: usize, j: usize) {
    let temp = vec[i];
    vec[i] = vec[j];
    vec[j] = temp;
}
fn partition(vec: &mut [i32], start: usize, end: usize) -> i32 {
    let pivot = vec[end];
    let mut index = start;
    let mut i = start;

    while i < end {
        if vec[i] < pivot {
            swap(vec, i, index);
            index += 1;
        }
        i += 1;
    }
    swap(vec, index, end);
    return index as i32;
}

fn quicksort(vec: &mut [i32], start: usize, end: usize) {
    if start >= end {
        return;
    }
    let pivot = partition(vec, start, end);
    quicksort(vec, start, (pivot - 1) as usize);
    quicksort(vec, (pivot + 1) as usize, end);
}

fn random_vec(length: u32) -> Vec<i32> {
    let mut slice = Vec::new(); 
    for _ in 0..length {
        slice.push(rand::random::<i32>());
    }
    slice
}

fn main() {
    let mut vec = random_vec(100);
    let last_index = vec.len() - 1;
    let st = Instant::now();
    quicksort(&mut vec, 0, last_index);
    println!("{:?}", st.elapsed());
    println!("{:?}", vec.len());
}
