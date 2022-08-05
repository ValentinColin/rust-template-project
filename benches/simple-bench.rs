//! This folder is only recognize on the nightly channel
//! Look for [Criterion](https://github.com/bheisler/criterion.rs) for benchmark on stable channel

#![feature(test)]

extern crate test;
use test::Bencher;

#[bench]
fn fibo_fast(b: &mut Bencher) {
    b.iter(|| {
        const N: u64 = 10000;
        let mut a = 0;
        let mut b = 1;

        if N != 0 {
            for _ in 0..N {
                let c = a + b;
                a = b;
                b = c;
            }
        }
        b
    });
}
