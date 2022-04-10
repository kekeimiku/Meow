use std::{
    fs::{self},
    io::Error,
    path::Path,
    thread::sleep,
    time::Duration,
};

use args::Args;
use elf::elf64;
use lince::mem::{read_bytes, search_all_rw_mem};

fn gg() {
    let bytes = fs::read(Path::new("elf/tests/bin/armelf64")).unwrap();
    let elf = elf64::Parse::new(&bytes);
    dbg!(elf.ehdr());
    elf.phdr_iter().for_each(|f| {
        dbg!(f);
    });

    elf.shdr_iter().for_each(|f| {
        dbg!(f);
    })
}

fn main() {
    // gg();
    // loop {
    //     match test1() {
    //         Ok(_) => {}
    //         Err(e) => {
    //             println!("{}", e.to_string());
    //             sleep(Duration::from_secs(1));
    //         }
    //     }
    // }
}

struct TARG {
    name: Option<String>,
}
pub fn test1() -> Result<(), Error> {
    let mut args = Args::new().unwrap();
    let args = TARG {        
        name: args.init("--name").unwrap(),
    };

    let mut i = 0;
    let hello = args.name.unwrap();
    loop {
        let t = search_all_rw_mem(&hello)?;
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
