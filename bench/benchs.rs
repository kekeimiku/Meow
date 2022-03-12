#![feature(test)]

extern crate test;

use std::fs;

use test::Bencher;

#[bench]
fn bench_get_pid_by_name(b: &mut Bencher) {
    use lince::comm::*;
    b.iter(|| {
        for _ in 0..10 {
            assert_eq!(get_pid_by_name("kthreadd").unwrap()[0], 2)
        }
    });
}

#[bench]
fn bench_x1(b: &mut Bencher) {
    use lince::mem::*;
    b.iter(|| {
        let i1 = search_index(
            &fs::read("/home/keke/testmem1.txt").unwrap(),
            &1_u8.to_be_bytes(),
        );
        let i2 = search_index(
            &fs::read("/home/keke/testmem2.txt").unwrap(),
            &1_u8.to_be_bytes(),
        );
        assert_eq!(43367, x1(i1, i2).len());
    });
}
