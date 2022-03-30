#![feature(once_cell)]

fn main() {
    // '0x7fff5ea93408'
    // loop {
    //     let mut v = read_bytes(0x7fff5ea93408, 8).unwrap();
    //     v.reverse();
    //     let mut v1 = [0u8; 8];
    //     v1.copy_from_slice(&v);
    //     println!("{:?} == {}", v, i64::from_be_bytes(v1));
    //     sleep(Duration::from_secs(1));
    // }

    // let mut xx = HashMap::new();
    // xx.insert(1, 2);

    let mut i = 0;

    let hello = "hello".as_bytes();

    loop {
        let l = lince::mem::search_all_rw_mem(hello).unwrap();

        if l.len() < 11 {
            l.iter().for_each(|f| {
                let a = lince::mem::read_bytes(*f, hello.len());
                println!("{:x} -> {:?}", f, a.unwrap());
            });
        } else {
            l[0..10].iter().for_each(|f| {
                let a = lince::mem::read_bytes(*f, hello.len());
                println!("{:x} -> {:?}", f, a.unwrap());
            });
        }

        i += 1;
        println!("============={}===========================", i);
        std::thread::sleep(std::time::Duration::from_secs(1))
    }
}

pub fn sum(v: &[i32], n: usize) -> i32 {
    v.iter().take(n).sum()
}
