# Examples taken from:

https://levelup.gitconnected.com/evaluating-performance-on-classic-sorting-algorithms-in-python-and-rust-76f981dfc0c

Results in ms:

| Algo | number of values | python | rust | rust (release) |
|----|-----|----|-----|----|
| Bubble | 10000 | 7250 | 10169 | 159 |
| Bubble | 100000 | 719522 | 1030906 | 18131|
| Insertion | 10000 | 2256 | 2406 | 34 |
| Quicksort | 10000 | 20 | 20 | 0.675 |
| Quicksort | 100000 | 198 | 194 | 8 |


Results on MacBookPro M1 (2020)
| Algo | number of values | python | rust | rust (release) | go |
|----|-----|----|-----|----|----|
| Bubble | 10000 | 5133 | 7442 | 90 | 105 |
| Bubble | 100000 | - | - | 18231 | 11000 |
| Insertion | 10000 | 1400 | 1700 | 37 | 54 |
| Quicksort | 10000 | 14.5 | 12.9 |  0.48 | 0.7 |
| Quicksort | 100000 | - | - |  6 | 16 |
