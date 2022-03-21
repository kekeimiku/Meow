#![feature(once_cell)]

use std::{thread::sleep, time::Duration};

use lince::mem::{read_bytes, search_all_rw_mem};

pub mod comm;
pub mod def;

pub mod maps;
pub mod mem;
pub mod sdiff;

fn main() {
    // '0x7fff5ea93408'
    // loop {
    //     let mut v = read_bytes(0x7fff5ea93408, 8).unwrap();
    //     v.reverse();
    //     let mut v1 = [0u8; 8];
    //     v1.copy_from_slice(&v);
    //     println!("{:?} == {}", v, i64::from_be_bytes(v1));
    //     sleep(Duration::from_secs(1));
    // }

    let mut i = 0;

    let hello = "hello".as_bytes();

    loop {
        let l = search_all_rw_mem(&hello);

        if l.len() < 11 {
            l.iter().for_each(|f| {
                let a = read_bytes(*f, hello.len());
                println!("{:x} -> {:?}", f, a.unwrap());
            });
        } else {
            l[0..10].iter().for_each(|f| {
                let a = read_bytes(*f, hello.len());
                println!("{:x} -> {:?}", f, a.unwrap());
            });
        }

        i += 1;
        println!("============={}===========================", i);
        sleep(Duration::from_secs(1))
    }
}

pub fn sum(v: &[i32], n: usize) -> i32 {
    v.iter().take(n).sum()
}
