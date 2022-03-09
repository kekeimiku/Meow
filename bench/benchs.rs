#![feature(test)]

extern crate test;

use test::Bencher;

#[bench]
fn bench_get_pid_by_name(b: &mut Bencher) {
    use lince::comm::*;
    b.iter(|| {
        for _ in 0..10 {
            assert_eq!(get_pid_by_name("test").unwrap()[0], 143953) 
        }
    });

}

