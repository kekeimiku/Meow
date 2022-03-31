use std::{fs::read, path::Path};

use elf::elf64::Elf64;

#[test]
fn test_elf64() {
    let bytes = read(Path::new("/usr/lib/libc.so.6")).unwrap();

    let elf = Elf64::new(&bytes);

    println!("{:?}", elf.ehdr());

    for i in elf.shdr_iter() {
        println!("{:?}", i);
    }

    for i in elf.phdr_iter() {
        println!("{:?}", i);
    }
}
