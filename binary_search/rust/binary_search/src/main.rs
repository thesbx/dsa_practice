fn main() {
    binary_search(Box::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 5)
}

fn binary_search(data: Box<[usize]>, item: usize) {
    let mut low = 0;
    let mut high = data.len();

    while low <= high {
        let mid = (low + high) / 2;
        let guess = data[mid];

        if guess == item {
            println!("{}", mid);
            return;
        } else if guess > mid {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
}
