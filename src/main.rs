use scan::prompt::start;

fn main() {
    // let a = "12";

    // println!("{:?}",a.as_bytes());

    if let Err(err) = start() {
        println!("Error: {}", err)
    }
}
