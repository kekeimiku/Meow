use memscan::promt::start;

fn main() {
    match start() {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }
}
