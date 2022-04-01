Pure Rust, Zero allocation, no_std

仅仅为了满足个人需求而编写的二进制解析库。

特点：轻量快速，no_std，零分配，零依赖，纯rust，非常小的二进制开销（x64 4kb-）

elf32/64 ehdr,phdr,shdr解析。

** ！重要！它没有附带任何错误处理，也不会验证elf合法性。

```rust
use std::{fs, path::Path};
use elf::elf64;

fn main() {
    let file = fs::read(Path::new("elf/tests/bin/armelf64")).unwrap();
    let elf = elf64::Parse::new(&file);
    dbg!(elf.ehdr());
    elf.phdr_iter().for_each(|f| {
        dbg!(f);
    });

    elf.shdr_iter().for_each(|f| {
        dbg!(f);
    })
}
```