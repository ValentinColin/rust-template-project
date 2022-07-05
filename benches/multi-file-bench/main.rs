//! It seems these bench doesn't really work

#![feature(test)]

extern crate test;
use test::{Bencher, black_box};

mod bench_module;
use bench_module::{fibonacci_slow, fibonacci_fast};

#[bench]
fn fibo_slow_10(b: &mut Bencher) {
    b.iter(|| {
        black_box(fibonacci_slow(10));
    });
}

#[bench]
fn fibo_fast_10(b: &mut Bencher) {
    b.iter(|| {
        black_box(fibonacci_fast(10));
    });
}


#[bench]
fn fibo_fast_100(b: &mut Bencher) {
    b.iter(|| {
        black_box(fibonacci_fast(100));
    });
}

#[bench]
fn fibo_fast_1000(b: &mut Bencher) {
    b.iter(|| {
        black_box(fibonacci_fast(1000));
    });
}
