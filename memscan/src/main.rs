use memscan::promt::start;
// use memscan::promt::start;

fn main() {
    // let mut app = MemScan::new(4093).unwrap();
    // loop {
    // app.write_bytes(0x7ffd8ccfccb0, &9995_i32.to_le_bytes());
    // std::thread::sleep(Duration::from_micros(10));
    // }

    // let a = vec![1, 2, 3];
    // let b = vec![2];
    // println!("{}", memcmp(&a, &b));

    // main1()
    match start() {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}

// fn main1() {
//     //7ffd8ccfccb0

//     let mut app = MemScan::new(5442).unwrap();
//     let input = &1_u8.to_le_bytes();
//     let start = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .unwrap()
//         .as_millis();
//     app.search_all(input).unwrap();
//     let end = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .unwrap()
//         .as_millis();
//     println!("len: {}  耗时: {}", app.addr_cache.len(), end - start);
//     app.input = input.to_vec();

//     app.addr_list(10);

//     let start = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .unwrap()
//         .as_millis();
//     app.change_input_mem().unwrap();
//     let end = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .unwrap()
//         .as_millis();
//     println!("len: {}  耗时: {}", app.addr_cache.len(), end - start);

//     app.addr_list(10);

//     // match start() {
//     //     Ok(_) => {}
//     //     Err(err) => println!("{:?}", err),
//     // }
// }
