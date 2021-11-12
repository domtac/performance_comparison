use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Instant;

#[path = "../constants.rs"]
mod constants;

fn bubble_sort(vec: &mut Vec<i32>) {
    let mut no_swap = true;
    while no_swap {
        no_swap = false;
        for index in 0..vec.len() - 1 {
            if vec[index] > vec[index + 1] {
                let temp = vec[index];
                vec[index] = vec[index + 1];
                vec[index + 1] = temp;
                no_swap = true;
            }
        }
    }
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
    for i in 0..constants::NUMBER_OF_RUNS {
        int_vec.shuffle(&mut rng);
        let start = Instant::now();
        println!("{} run", i);

        bubble_sort(&mut int_vec);

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
