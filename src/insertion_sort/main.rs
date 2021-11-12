use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Instant;
#[path = "../constants.rs"]
mod constants;



fn insertion_sort(vec: &mut Vec<i32>) {
    for index in 1..vec.len() {
        let mut j = index;
        while j > 0 && vec[j - 1] > vec[j] {
            vec.swap(j - 1, j);
            j -= 1;
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

        insertion_sort(&mut int_vec);

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