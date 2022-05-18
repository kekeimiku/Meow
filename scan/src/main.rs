use scan::prompt::start;

fn main() {
    if let Err(err) = start() {
        println!("Error: {}", err)
    }
}