use memscan::prompt::start;

fn main() {
    match start() {
        Ok(_) => {}
        Err(err) => println!("Error: {}", err),
    }
}
