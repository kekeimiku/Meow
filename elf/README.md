Pure Rust, Zero allocation, no_std

非常轻量,no_std,零分配,纯rust elf32/64 ehdr,phdr,shdr解析。

！！！它没有任何错误处理，也不会验证elf合法性。


```
    let bytes = fs::read(Path::new("tests/bin/armelf64")).unwrap();
    let elf = Elf64::parse(&bytes);
    dbg!(elf.ehdr());
    elf.phdr_iter().for_each(|f| {
        dbg!(f);
    });

    elf.shdr_iter().for_each(|f| {
        dbg!(f);
    })
```