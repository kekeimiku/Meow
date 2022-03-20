#![feature(test)]

extern crate test;

use std::fs;

use test::Bencher;

#[bench]
fn bench_get_pid_by_name(b: &mut Bencher) {
    use lince::comm::*;
    b.iter(|| {
        for _ in 0..10 {
            assert_eq!(get_pid_by_name("kthreadd").unwrap(), 2)
        }
    });
}

#[bench]
fn bench_x1(b: &mut Bencher) {
    use lince::mem::*;
    b.iter(|| {
        let i1 = memchr::memmem::find_iter(
            &fs::read("/home/keke/Templates/txt/testmem1.txt").unwrap(),
            &1_u8.to_be_bytes(),
        )
        .collect::<Vec<usize>>();
        let i2 = memchr::memmem::find_iter(
            &fs::read("/home/keke/Templates/txt/testmem2.txt").unwrap(),
            &1_u8.to_be_bytes(),
        )
        .collect::<Vec<usize>>();
        assert_eq!(43367, x1(i1, i2).len());
    });
}
