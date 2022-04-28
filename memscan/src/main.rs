use std::time::{SystemTime, UNIX_EPOCH};
use memscan::MemScan;
// use memscan::promt::start;

fn main() {
    //let pid = std::env::args().nth(1).unwrap().parse::<i32>().unwrap();
    let mut app = MemScan::new(1025);
    let input = &1_u8.to_le_bytes();
    let start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    app.search_all(input).unwrap();
    let end = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    println!("len: {}  耗时: {}", app.addr_cache.len(), end - start);
    app.input = input.to_vec();

    app.addr_list(10);

    let start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    app.change_mem().unwrap();
    let end = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    println!("len: {}  耗时: {}", app.addr_cache.len(), end - start);

    app.addr_list(10);

    // match start() {
    //     Ok(_) => {}
    //     Err(err) => println!("{:?}", err),
    // }
}
