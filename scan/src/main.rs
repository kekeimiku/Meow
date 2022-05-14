use scan::{prompt::start, mem::ttt};

fn main() {
    match start() {
        Ok(_) => {}
        Err(err) => println!("Error: {}", err),
    }
    // ttt()
}
