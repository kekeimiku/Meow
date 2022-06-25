use std::fs::File;

use meow::mem::MemScan;
use utils::info;

fn main() {
    let file = File::open("test.txt").unwrap();
    let value = "22".as_bytes();

    let v = MemScan::new(&file).find_region_addr(0, 10, value);

    info!("{:?}", v);
}
