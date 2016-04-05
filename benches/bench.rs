#![feature(test)]
#![allow(unused_unsafe)]

extern crate test;
extern crate memcmp;

fn make_vecs(len: usize) -> (Vec<u8>, Vec<u8>) {
    let mut a = Vec::new();
    let mut b = Vec::new();

    for _ in 0 .. len {
        a.push(0);
        b.push(0);
    }

    a.push(0);
    b.push(1);

    (a, b)
}

macro_rules! benchmark_cmp {
    ($($name:ident | $method:expr => $len:expr,)*) => {
        $(
        #[bench]
        fn $name(b: &mut test::Bencher) {
            let (lhs, rhs) = make_vecs($len);

            b.bytes = lhs.len() as u64;
            b.iter(|| {
                let cmp = unsafe { $method(&lhs, &rhs, lhs.len()) };
                assert!(cmp < 0);
                test::black_box(cmp);

                let cmp = unsafe { $method(&lhs, &lhs, lhs.len()) };
                assert!(cmp == 0);
                test::black_box(cmp);

                let cmp = unsafe { $method(&rhs, &lhs, lhs.len()) };
                assert!(cmp > 0);
                test::black_box(cmp);
            })
        }
        )*
    }
}

macro_rules! benchmark_eq {
    ($($name:ident | $method:expr => $len:expr,)*) => {
        $(
        #[bench]
        fn $name(b: &mut test::Bencher) {
            let (lhs, rhs) = make_vecs($len);

            b.bytes = lhs.len() as u64;
            b.iter(|| {
                let cmp = unsafe { $method(&lhs, &lhs, lhs.len()) };
                assert_eq!(cmp, true);
                test::black_box(cmp);

                let cmp = unsafe { $method(&lhs, &rhs, lhs.len()) };
                assert_eq!(cmp, false);
                test::black_box(cmp);

                let cmp = unsafe { $method(&rhs, &lhs, rhs.len()) };
                assert_eq!(cmp, false);
                test::black_box(cmp);

                let cmp = unsafe { $method(&rhs, &rhs, rhs.len()) };
                assert_eq!(cmp, true);
                test::black_box(cmp);
            })
        }
        )*
    }
}

benchmark_cmp! {
    bench_cmp_0001_memcmp | memcmp::real_memcmp => 1,
    bench_cmp_0002_memcmp | memcmp::real_memcmp => 2,
    bench_cmp_0003_memcmp | memcmp::real_memcmp => 3,
    bench_cmp_0004_memcmp | memcmp::real_memcmp => 4,
    bench_cmp_0005_memcmp | memcmp::real_memcmp => 5,
    bench_cmp_0006_memcmp | memcmp::real_memcmp => 6,
    bench_cmp_0007_memcmp | memcmp::real_memcmp => 7,
    bench_cmp_0008_memcmp | memcmp::real_memcmp => 8,
    bench_cmp_0015_memcmp | memcmp::real_memcmp => 15,
    bench_cmp_0016_memcmp | memcmp::real_memcmp => 16,
    bench_cmp_0032_memcmp | memcmp::real_memcmp => 32,
    bench_cmp_0064_memcmp | memcmp::real_memcmp => 64,
    bench_cmp_0128_memcmp | memcmp::real_memcmp => 128,
    bench_cmp_0256_memcmp | memcmp::real_memcmp => 256,
    bench_cmp_1024_memcmp | memcmp::real_memcmp => 1024,
    bench_cmp_4096_memcmp | memcmp::real_memcmp => 4096,

    bench_cmp_0001_my_memcmp | memcmp::my_memcmp => 1,
    bench_cmp_0002_my_memcmp | memcmp::my_memcmp => 2,
    bench_cmp_0003_my_memcmp | memcmp::my_memcmp => 3,
    bench_cmp_0004_my_memcmp | memcmp::my_memcmp => 4,
    bench_cmp_0005_my_memcmp | memcmp::my_memcmp => 5,
    bench_cmp_0006_my_memcmp | memcmp::my_memcmp => 6,
    bench_cmp_0007_my_memcmp | memcmp::my_memcmp => 7,
    bench_cmp_0008_my_memcmp | memcmp::my_memcmp => 8,
    bench_cmp_0015_my_memcmp | memcmp::my_memcmp => 15,
    bench_cmp_0016_my_memcmp | memcmp::my_memcmp => 16,
    bench_cmp_0032_my_memcmp | memcmp::my_memcmp => 32,
    bench_cmp_0064_my_memcmp | memcmp::my_memcmp => 64,
    bench_cmp_0128_my_memcmp | memcmp::my_memcmp => 128,
    bench_cmp_0256_my_memcmp | memcmp::my_memcmp => 256,
    bench_cmp_1024_my_memcmp | memcmp::my_memcmp => 1024,
    bench_cmp_4096_my_memcmp | memcmp::my_memcmp => 4096,

    bench_cmp_0001_rust_cmp | memcmp::rust_cmp => 1,
    bench_cmp_0002_rust_cmp | memcmp::rust_cmp => 2,
    bench_cmp_0003_rust_cmp | memcmp::rust_cmp => 3,
    bench_cmp_0004_rust_cmp | memcmp::rust_cmp => 4,
    bench_cmp_0005_rust_cmp | memcmp::rust_cmp => 5,
    bench_cmp_0006_rust_cmp | memcmp::rust_cmp => 6,
    bench_cmp_0007_rust_cmp | memcmp::rust_cmp => 7,
    bench_cmp_0008_rust_cmp | memcmp::rust_cmp => 8,
    bench_cmp_0015_rust_cmp | memcmp::rust_cmp => 15,
    bench_cmp_0016_rust_cmp | memcmp::rust_cmp => 16,
    bench_cmp_0032_rust_cmp | memcmp::rust_cmp => 32,
    bench_cmp_0064_rust_cmp | memcmp::rust_cmp => 64,
    bench_cmp_0128_rust_cmp | memcmp::rust_cmp => 128,
    bench_cmp_0256_rust_cmp | memcmp::rust_cmp => 256,
    bench_cmp_1024_rust_cmp | memcmp::rust_cmp => 1024,
    bench_cmp_4096_rust_cmp | memcmp::rust_cmp => 4096,
}

benchmark_eq! {
    bench_eq_0001_my_memeq | memcmp::my_memeq => 1,
    bench_eq_0002_my_memeq | memcmp::my_memeq => 2,
    bench_eq_0003_my_memeq | memcmp::my_memeq => 3,
    bench_eq_0004_my_memeq | memcmp::my_memeq => 4,
    bench_eq_0005_my_memeq | memcmp::my_memeq => 5,
    bench_eq_0006_my_memeq | memcmp::my_memeq => 6,
    bench_eq_0007_my_memeq | memcmp::my_memeq => 7,
    bench_eq_0008_my_memeq | memcmp::my_memeq => 8,
    bench_eq_0015_my_memeq | memcmp::my_memeq => 15,
    bench_eq_0016_my_memeq | memcmp::my_memeq => 16,
    bench_eq_0032_my_memeq | memcmp::my_memeq => 32,
    bench_eq_0064_my_memeq | memcmp::my_memeq => 64,
    bench_eq_0128_my_memeq | memcmp::my_memeq => 128,
    bench_eq_0256_my_memeq | memcmp::my_memeq => 256,
    bench_eq_1024_my_memeq | memcmp::my_memeq => 1024,
    bench_eq_4096_my_memeq | memcmp::my_memeq => 4096,

    bench_eq_0001_rust_eq | memcmp::rust_eq => 1,
    bench_eq_0002_rust_eq | memcmp::rust_eq => 2,
    bench_eq_0003_rust_eq | memcmp::rust_eq => 3,
    bench_eq_0004_rust_eq | memcmp::rust_eq => 4,
    bench_eq_0005_rust_eq | memcmp::rust_eq => 5,
    bench_eq_0006_rust_eq | memcmp::rust_eq => 6,
    bench_eq_0007_rust_eq | memcmp::rust_eq => 7,
    bench_eq_0008_rust_eq | memcmp::rust_eq => 8,
    bench_eq_0015_rust_eq | memcmp::rust_eq => 15,
    bench_eq_0016_rust_eq | memcmp::rust_eq => 16,
    bench_eq_0032_rust_eq | memcmp::rust_eq => 32,
    bench_eq_0064_rust_eq | memcmp::rust_eq => 64,
    bench_eq_0128_rust_eq | memcmp::rust_eq => 128,
    bench_eq_0256_rust_eq | memcmp::rust_eq => 256,
    bench_eq_1024_rust_eq | memcmp::rust_eq => 1024,
    bench_eq_4096_rust_eq | memcmp::rust_eq => 4096,
}
