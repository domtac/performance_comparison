# Performance measurement

Unsorted collection of algorithm for the prupose of doing basic benchmarks between Rust, go and python. 

My key take away is to never forget to '--release' my rust code ever again.

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
| Quicksort | 10000 | 14.5 | 12.9 |  0.48 | 0.7 |
| Quicksort | 100000 | 180 | 140 |  6 | 16 |

### Bonus: Parallel quick sort

As the manufacturers of my  laptop where so kind to provide me with several computation cores, I was willing to try a parallel quicksort to speed up the sorting even more. Another reason is that I wanted to try out the [rayon](https://crates.io/crates/rayon/1.2.1) crate.

The interesting effect here is, that rayon really allowed parallelism w/o significant overhead, whereas the parallel inplementation in go or python had little effect. For Go the parallelism even made it slower.

| Algo | number of values | python | rust | rust (release) | go |
|----|-----|----|-----|----|----|
| Parallel quick sort | 100000 | 150 | 25 | 1.7 | 61 |


# License

Some of the code parts have been copied from example sites. The sources will be stated in the files.

The rest is licensed under [WTFPL](http://www.wtfpl.net)