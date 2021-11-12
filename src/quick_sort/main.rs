use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Instant;
#[path = "../constants.rs"]
mod constants;

fn swap(vec: &mut Vec<i32>, i: usize, j: usize) {
    let temp = vec[i];
    vec[i] = vec[j];
    vec[j] = temp;
}
fn partition(vec: &mut Vec<i32>, start: usize, end: usize) -> i32 {
    let mut pivot = vec[end];
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

fn quick_sort(vec: &mut Vec<i32>, start: usize, end: usize) {
    if start >= end {
        return;
    }

    let mut pivot = partition(vec, start, end);
    let min_pivot = if pivot == 0 { 0 } else { pivot - 1 };

    quick_sort(vec, start, min_pivot as usize);
    quick_sort(vec, (pivot + 1) as usize, end);
}


fn main() {
    let mut rng = thread_rng();
    let mut int_vec = Vec::new();
    let max_num = constants::SIZE_OF_SORT_ARRAY;

    for num in 1..max_num {
        int_vec.push(num as i32);
    }

    let mut total_elapsed = 0;
    println!("Running {} times", constants::NUMBER_OF_RUNS);
    let vec_size = int_vec.len() - 1;
    for i in 0..constants::NUMBER_OF_RUNS {
        int_vec.shuffle(&mut rng);
        let start = Instant::now();
        println!("{} run", i);

        quick_sort(&mut int_vec, 0, vec_size);

        let elapsed = start.elapsed().as_micros();
        total_elapsed += elapsed;
    }
    let elapsed = total_elapsed / constants::NUMBER_OF_RUNS as u128;
    println!(
        "It took: {}.{} milliseconds in average to sort",
        elapsed / 1000,
        elapsed % 1000
    );
}