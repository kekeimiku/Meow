use std::{io::Error, thread::sleep, time::Duration};

use lince::mem::{read_bytes, search_all_rw_mem};

fn main() {    
    loop {
        match test1() {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e.to_string());
                sleep(Duration::from_secs(1));
            }
        }
    }
}

fn test1() -> Result<(), Error> {
    let mut i = 0;
    let hello = "hello".as_bytes();
    loop {
        let t = search_all_rw_mem(hello)?;
        if t.len() < 11 {
            t.iter().try_for_each(|f| -> Result<(), Error> {
                let a = read_bytes(*f, hello.len())?;
                println!("0x{:x} -> {:?}", f, a);
                Ok(())
            })?;
        } else {
            t[0..10].iter().try_for_each(|f| -> Result<(), Error> {
                let a = read_bytes(*f, hello.len())?;
                println!("0x{:x} -> {:?}", f, a);
                Ok(())
            })?;
        }

        i += 1;
        println!("============={}=============================", i);
        sleep(Duration::from_secs(1));
    }
}
