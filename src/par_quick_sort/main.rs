use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Instant;
#[path = "../constants.rs"]
mod constants;



fn par_quick_sort<T: PartialOrd + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let mid = par_partition(v);
    let (lo, hi) = v.split_at_mut(mid);
    rayon::join(|| par_quick_sort::<T>(lo), || par_quick_sort::<T>(hi));
}

fn par_partition<T: PartialOrd + Send>(v: &mut [T]) -> usize {
    let pivot = v.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if v[j] <= v[pivot] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);
    i
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

        par_quick_sort(&mut int_vec);

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