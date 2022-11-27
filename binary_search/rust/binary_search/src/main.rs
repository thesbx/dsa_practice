use std::time::Instant;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();


    match args.len() {
        2 => {
            let size = args[1].parse::<usize>().unwrap();
            let target = size - 1;
            let array = create_array(size);
            println!("Array size: {}", &array.len());
            binary_search(&array, &target);
            println!("---------------------");
            linear_search(&array, &target);
        },
        3 => {
            let size = args[1].parse::<usize>().unwrap();
            let target = args[2].parse::<usize>().unwrap();
            let array = create_array(size);
            println!("Array size: {}", &array.len());
            binary_search(&array, &target);
            println!("---------------------");
            linear_search(&array, &target);
        },
        _ => println!("Need at least 1 arguments"),
    }
}

fn create_array(length: usize) -> Vec<usize> {
    let mut arr = vec![1; length];
    for i in 0..length {
        arr[i] = i;
    }
    arr
}

fn binary_search(data: &Vec<usize>, target: &usize) -> Option<usize> {
    let start = Instant::now();
    let mut low: usize = 0;
    let mut high: usize = data.len() as usize - 1;
    
    while low <= high {
        
        let mid = ((high - low) / 2) + low;
        let mid_index = mid as usize;
        let val = &data[mid_index];

        if val == target {
            let end = start.elapsed();
            println!("Result: {}", mid_index);
            println!("Binary Search Exec: {:?}", end);
            break;
        }

        // Search values that are greater than val - to right of current mid_index
        if val < target {
            low = mid + 1;
        }

        // Search values that are less than val - to the left of current mid_index
        if val > target {
            high = mid - 1;
        }
    }
    None
}

fn linear_search(data: &Vec<usize>, target: &usize) -> Option<usize> {
    let start = Instant::now();
    for (index, val) in data.iter().enumerate() {
        if val == target {
            let end = start.elapsed();
            println!("Result: {}", index);
            println!("Linear Search Exec: {:?}", end);
            break;
        }
    }
    None
}
