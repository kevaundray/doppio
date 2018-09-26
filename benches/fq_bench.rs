#![feature(test)]

extern crate jubjub;
extern crate test;

use jubjub::Fq;
use std::ops::MulAssign;
use test::Bencher;

#[bench]
fn bench_mul_assign(bencher: &mut Bencher) {
    let mut n = Fq::one();
    bencher.iter(move || {
        let tmp = n;
        n.mul_assign(&tmp);
    });
}

#[bench]
fn bench_square_assign(bencher: &mut Bencher) {
    let mut n = Fq::one();
    bencher.iter(move || {
        n.square_assign();
    });
}

#[bench]
fn bench_pow_q_minus_2(bencher: &mut Bencher) {
    let n = Fq::one();
    bencher.iter(move || {
        n.pow_q_minus_2()
    });
}