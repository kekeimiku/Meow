use memscan::MemScan;
// use memscan::promt::start;

fn main() {

    //let pid = std::env::args().nth(1).unwrap().parse::<i32>().unwrap();
    let mut app = MemScan::new(1025);
    let input = &1_i32.to_le_bytes();
    app.search_all(input).unwrap();
    app.input = input.to_vec();
    app.addr_list(10);

    app.change_mem().unwrap();
    app.addr_list(10);

    // match start() {
    //     Ok(_) => {}
    //     Err(err) => println!("{:?}", err),
    // }
}
