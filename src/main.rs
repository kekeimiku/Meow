use std::{fs::File, os::unix::prelude::FileExt};

use memchr::memmem::find_iter;
use meow::mem::Mem;
use utils::info;

fn main() {
    let file = File::open("test.txt").unwrap();
    let value = "22".as_bytes();

    let v = Mem::new(&file).find_region_addr(0, 100, &value).unwrap();

    info!("{:?}", v);

    let mut buf = vec![0; 100];
    file.read_at(&mut buf, 0).unwrap();
    info!("helloworld{:?}", find_iter(&buf, &value).collect::<Vec<usize>>());
}
