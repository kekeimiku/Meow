use std::{fs::File, os::unix::prelude::FileExt};

use memchr::memmem::find_iter;
use meow::mem::find_region_addr;
use utils::info;

fn main() {
    let file = File::open("test.txt").unwrap();
    let value = "22".as_bytes();
    let v = find_region_addr(&file, 0, 100, &value, false).unwrap();
    info!("{:?}", v);

    let mut buf = vec![0; 100];
    file.read_at(&mut buf, 0).unwrap();
    info!("{:?}", find_iter(&buf, &value).collect::<Vec<usize>>())
}
