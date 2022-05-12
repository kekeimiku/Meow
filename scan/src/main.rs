use memscan::prompt::start;

fn main() {
    match start() {
        Ok(_) => {}
        Err(err) => println!("Error: {}", err),
    }

    // let mut v1 = vec![1, 2, 0, 4, 5, 6];
    // let v2 = vec![0, 2, 3, 4, 5, 6];

    // for (k, i) in (0..v1.len()).rev().zip(v2.iter().rev()) {
    //     println!("{} {}",v1[k],i);
    //     if v1[k] == *i {            
    //         v1.swap_remove(k);
    //     }
    // }
    // println!("{:?}", v1);
}
