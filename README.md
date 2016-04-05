# Experiments with Rust memcmp implmentations.

Break even point on my laptop appears to be around 16 bytes.

```
running 80 tests
test bench_cmp_0001_memcmp    ... bench:           8 ns/iter (+/- 1) = 250 MB/s
test bench_cmp_0001_my_memcmp ... bench:           3 ns/iter (+/- 0) = 666 MB/s
test bench_cmp_0001_rust_cmp  ... bench:           8 ns/iter (+/- 1) = 250 MB/s
test bench_cmp_0002_memcmp    ... bench:          10 ns/iter (+/- 1) = 300 MB/s
test bench_cmp_0002_my_memcmp ... bench:           4 ns/iter (+/- 1) = 750 MB/s
test bench_cmp_0002_rust_cmp  ... bench:           8 ns/iter (+/- 2) = 375 MB/s
test bench_cmp_0003_memcmp    ... bench:           9 ns/iter (+/- 1) = 444 MB/s
test bench_cmp_0003_my_memcmp ... bench:           6 ns/iter (+/- 1) = 666 MB/s
test bench_cmp_0003_rust_cmp  ... bench:           9 ns/iter (+/- 3) = 444 MB/s
test bench_cmp_0004_memcmp    ... bench:          12 ns/iter (+/- 2) = 416 MB/s
test bench_cmp_0004_my_memcmp ... bench:           7 ns/iter (+/- 2) = 714 MB/s
test bench_cmp_0004_rust_cmp  ... bench:          12 ns/iter (+/- 2) = 416 MB/s
test bench_cmp_0005_memcmp    ... bench:          12 ns/iter (+/- 2) = 500 MB/s
test bench_cmp_0005_my_memcmp ... bench:           9 ns/iter (+/- 1) = 666 MB/s
test bench_cmp_0005_rust_cmp  ... bench:          12 ns/iter (+/- 4) = 500 MB/s
test bench_cmp_0006_memcmp    ... bench:          14 ns/iter (+/- 5) = 500 MB/s
test bench_cmp_0006_my_memcmp ... bench:          10 ns/iter (+/- 2) = 700 MB/s
test bench_cmp_0006_rust_cmp  ... bench:          13 ns/iter (+/- 3) = 538 MB/s
test bench_cmp_0007_memcmp    ... bench:          14 ns/iter (+/- 3) = 571 MB/s
test bench_cmp_0007_my_memcmp ... bench:          10 ns/iter (+/- 2) = 800 MB/s
test bench_cmp_0007_rust_cmp  ... bench:          14 ns/iter (+/- 2) = 571 MB/s
test bench_cmp_0008_memcmp    ... bench:          14 ns/iter (+/- 3) = 642 MB/s
test bench_cmp_0008_my_memcmp ... bench:          12 ns/iter (+/- 3) = 750 MB/s
test bench_cmp_0008_rust_cmp  ... bench:          15 ns/iter (+/- 3) = 600 MB/s
test bench_cmp_0015_memcmp    ... bench:          20 ns/iter (+/- 6) = 800 MB/s
test bench_cmp_0015_my_memcmp ... bench:          19 ns/iter (+/- 2) = 842 MB/s
test bench_cmp_0015_rust_cmp  ... bench:          20 ns/iter (+/- 4) = 800 MB/s
test bench_cmp_0016_memcmp    ... bench:          20 ns/iter (+/- 5) = 850 MB/s
test bench_cmp_0016_my_memcmp ... bench:          20 ns/iter (+/- 10) = 850 MB/s
test bench_cmp_0016_rust_cmp  ... bench:          21 ns/iter (+/- 5) = 809 MB/s
test bench_cmp_0032_memcmp    ... bench:          12 ns/iter (+/- 2) = 2750 MB/s
test bench_cmp_0032_my_memcmp ... bench:          40 ns/iter (+/- 5) = 825 MB/s
test bench_cmp_0032_rust_cmp  ... bench:          12 ns/iter (+/- 2) = 2750 MB/s
test bench_cmp_0064_memcmp    ... bench:          15 ns/iter (+/- 1) = 4333 MB/s
test bench_cmp_0064_my_memcmp ... bench:          84 ns/iter (+/- 8) = 773 MB/s
test bench_cmp_0064_rust_cmp  ... bench:          15 ns/iter (+/- 1) = 4333 MB/s
test bench_cmp_0128_memcmp    ... bench:          20 ns/iter (+/- 2) = 6450 MB/s
test bench_cmp_0128_my_memcmp ... bench:         149 ns/iter (+/- 24) = 865 MB/s
test bench_cmp_0128_rust_cmp  ... bench:          20 ns/iter (+/- 2) = 6450 MB/s
test bench_cmp_0256_memcmp    ... bench:          29 ns/iter (+/- 2) = 8862 MB/s
test bench_cmp_0256_my_memcmp ... bench:         271 ns/iter (+/- 43) = 948 MB/s
test bench_cmp_0256_rust_cmp  ... bench:          30 ns/iter (+/- 11) = 8566 MB/s
test bench_cmp_1024_memcmp    ... bench:         114 ns/iter (+/- 12) = 8991 MB/s
test bench_cmp_1024_my_memcmp ... bench:       1,019 ns/iter (+/- 143) = 1005 MB/s
test bench_cmp_1024_rust_cmp  ... bench:         114 ns/iter (+/- 14) = 8991 MB/s
test bench_cmp_4096_memcmp    ... bench:         309 ns/iter (+/- 49) = 13258 MB/s
test bench_cmp_4096_my_memcmp ... bench:       4,036 ns/iter (+/- 750) = 1015 MB/s
test bench_cmp_4096_rust_cmp  ... bench:         314 ns/iter (+/- 113) = 13047 MB/s
test bench_eq_0001_my_memeq   ... bench:           4 ns/iter (+/- 1) = 500 MB/s
test bench_eq_0001_rust_eq    ... bench:           3 ns/iter (+/- 1) = 666 MB/s
test bench_eq_0002_my_memeq   ... bench:           5 ns/iter (+/- 0) = 600 MB/s
test bench_eq_0002_rust_eq    ... bench:           5 ns/iter (+/- 1) = 600 MB/s
test bench_eq_0003_my_memeq   ... bench:           8 ns/iter (+/- 1) = 500 MB/s
test bench_eq_0003_rust_eq    ... bench:           6 ns/iter (+/- 1) = 666 MB/s
test bench_eq_0004_my_memeq   ... bench:           8 ns/iter (+/- 1) = 625 MB/s
test bench_eq_0004_rust_eq    ... bench:           7 ns/iter (+/- 0) = 714 MB/s
test bench_eq_0005_my_memeq   ... bench:          10 ns/iter (+/- 1) = 600 MB/s
test bench_eq_0005_rust_eq    ... bench:           9 ns/iter (+/- 3) = 666 MB/s
test bench_eq_0006_my_memeq   ... bench:          10 ns/iter (+/- 1) = 700 MB/s
test bench_eq_0006_rust_eq    ... bench:          10 ns/iter (+/- 2) = 700 MB/s
test bench_eq_0007_my_memeq   ... bench:          13 ns/iter (+/- 1) = 615 MB/s
test bench_eq_0007_rust_eq    ... bench:          11 ns/iter (+/- 1) = 727 MB/s
test bench_eq_0008_my_memeq   ... bench:          13 ns/iter (+/- 1) = 692 MB/s
test bench_eq_0008_rust_eq    ... bench:          12 ns/iter (+/- 0) = 750 MB/s
test bench_eq_0015_my_memeq   ... bench:          26 ns/iter (+/- 2) = 615 MB/s
test bench_eq_0015_rust_eq    ... bench:          21 ns/iter (+/- 1) = 761 MB/s
test bench_eq_0016_my_memeq   ... bench:          24 ns/iter (+/- 7) = 708 MB/s
test bench_eq_0016_rust_eq    ... bench:          22 ns/iter (+/- 1) = 772 MB/s
test bench_eq_0032_my_memeq   ... bench:          52 ns/iter (+/- 2) = 634 MB/s
test bench_eq_0032_rust_eq    ... bench:          42 ns/iter (+/- 9) = 785 MB/s
test bench_eq_0064_my_memeq   ... bench:         100 ns/iter (+/- 5) = 650 MB/s
test bench_eq_0064_rust_eq    ... bench:          89 ns/iter (+/- 10) = 730 MB/s
test bench_eq_0128_my_memeq   ... bench:         193 ns/iter (+/- 13) = 668 MB/s
test bench_eq_0128_rust_eq    ... bench:         148 ns/iter (+/- 17) = 871 MB/s
test bench_eq_0256_my_memeq   ... bench:         348 ns/iter (+/- 49) = 738 MB/s
test bench_eq_0256_rust_eq    ... bench:         278 ns/iter (+/- 78) = 924 MB/s
test bench_eq_1024_my_memeq   ... bench:       1,324 ns/iter (+/- 183) = 774 MB/s
test bench_eq_1024_rust_eq    ... bench:       1,008 ns/iter (+/- 68) = 1016 MB/s
test bench_eq_4096_my_memeq   ... bench:       5,206 ns/iter (+/- 363) = 786 MB/s
test bench_eq_4096_rust_eq    ... bench:       3,947 ns/iter (+/- 1,189) = 1038 MB/s

test result: ok. 0 passed; 0 failed; 0 ignored; 80 measured

     Running target/release/memcmp-8e6da4379eccaab6

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```
