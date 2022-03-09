#![feature(test)]

extern crate test;

#[test]
fn bench_get_pid_by_name() {
    use lince::comm::*;
    assert_eq!(get_pid_by_name("kthreadd").unwrap()[0], 2);
}
