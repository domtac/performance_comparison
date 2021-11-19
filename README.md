# Performance measurement

Unsorted collection of algorithm for the prupose of doing basic benchmarks between Rust, go and python. 

My key take away is to never forget to `--release` my rust code ever again.

The algorithms used here are differnet apporaches to sorting random numbers. 
So far I have done:
- Bubblesort
- Insertions sort
- Quick sort
- Parallel quicksort

> Disclaimer:
> As the intention is to quickly get an impression of performance. The code is not optimized, not well document and violates a bunch of desing guidelines, especially [DRY](https://en.wikipedia.org/wiki/Don%27t_repeat_yourself). Please do not take this code as an example unless as a bad one

## Results
> Examples taken from:
>
> The sorting algorithms have been taken from the following blog post:
> https://levelup.gitconnected.com/evaluating-performance-on-classic-sorting-algorithms-in-python-and-rust-76f981dfc0c
>
> The GO implementations came from [here](https://www.golangprograms.com/)


Results in ms:


Results on MacBookPro M1 (2020)
| Algo | number of values | python | rust | rust (release) | go |
|----|-----|----|-----|----|----|
| Bubble | 10000 | 5133 | 7442 | 90 | 105 |
| Bubble | 100000 | - | - | 18231 | 11000 |
| Insertion | 10000 | 1400 | 1700 | 37 | 54 |
| Quicksort | 10000 | 14.5 | 12.9 |  0.48 | 0.6 |
| Quicksort | 100000 | 180 | 140 |  6 | 9 |

### Bonus: Parallel quick sort

As the manufacturers of my  laptop where so kind to provide me with several computation cores, I was willing to try a parallel quicksort to speed up the sorting even more. Another reason is that I wanted to try out the [rayon](https://crates.io/crates/rayon/1.2.1) crate.

The interesting effect here is, that rayon really allowed parallelism w/o significant overhead, whereas the parallel inplementation in python had little effect.

| Algo | number of values | python | rust | rust (release) | go (GOMAXPROCS=8) |
|----|-----|----|-----|----|----|
| Parallel quick sort | 100000 | 150 | 25 | 1.7 | 2 |

As a note, goroutines in theory do take double the cores into account compared to rayon, as rayon only runs on physical cores and goroutines can leverage hyperthreading.
If I reduce 'runtime.GOMAXPROCS' to the number of my physical cores, the time taken goes up to roughly 3ms.


## Running the tests

No elaborated runner in place here. I assume that you have python3, rust and go installed on your machine.

To run the tests:
__Python__
Navigate into the python folder and:
```shell
python3 bubblesort.py
python3 insertionsort.py
python3 quicksort.py
python3 par_quicksort.py
```

__Golang__
Navigate into the go folder and:
```shell
go run bubblesort.go
go run insertionsort.go
go run quicksort.go
go run par_quicksort.go
```

__Rust__
Stay in the root folder and:
```shell
cargo run --bin bubble_sort
cargo run --bin insertion_sort
cargo run --bin quick_sort
cargo run --bin par_quick_sort
```
To get the release run add a `--release` to the above mentioned commands


# License

Some of the code parts have been copied from example sites. The sources will be stated in the files.

The rest is licensed under [WTFPL](http://www.wtfpl.net)