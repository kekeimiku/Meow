use std::{
    fs::{self, File},
    io::{Error, Read, Seek, SeekFrom},
    path::Path,
    thread::sleep,
    time::Duration,
};

use lince::mem::{read_bytes, search_all_rw_mem};

fn gg() {
    let mut i = 0;

    let bytes = fs::read(Path::new("elf/tests/bin/armelf64")).unwrap();
    let elf = elf::elf64::Elf64::new(&bytes);
    // println!("ehdr=> {:?}", elf.ehdr());
    println!("节区表数量：{}", elf.ehdr().e_shnum);
    elf.phdr_iter().for_each(|f| {
        // println!("phdr=> {:?}", f);
    });

    elf.shdr_iter().for_each(|f| {
        println!("[{}] shdr=> {:?}", i, f);
        i += 1;
    })
}

fn main() {
    gg();
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

pub fn test1() -> Result<(), Error> {
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
