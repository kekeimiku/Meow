use std::{fs::File, os::unix::prelude::FileExt};

use meow::mem::MemScan;
use utils::info;

fn main() {
    let file = File::open("test.txt").unwrap();
    let value = "22".as_bytes();

    let v = MemScan::new(&file).find_region_addr(0, 10, value);

    info!("{:?}", v);

    let mut buf = vec![0; 10];
    file.read_at(&mut buf, 0).unwrap();

    info!("{:?}", memchr::memmem::find_iter(&buf, &value).collect::<Vec<usize>>());
}
