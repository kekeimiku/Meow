仅为满足个人需求的命令行解析库

特点：轻量，非常小的二进制开销（x64 8kb）。

它可能有非常多的小问题，但可能是故意不去解决这些，对我来说已经够用那就无所谓，也不会去专门处理它，但是欢迎issue以及pr。

```rust
use args::{Args, Error};

#[derive(Debug)]
struct TestArgs {
    help: bool,
    age: Option<u32>,
    name: Option<String>,
}

fn main() {
    dbg!(init_args());
}

fn init_args() -> Result<TestArgs, Error> {
    let mut args = Args::init()?;
    let args = TestArgs {
        help: args.contains(["-h", "--help"]),
        age: args.new("--age")?,
        name: args.new("--name")?,
    };

    if args.help {
        println!(
            r#"help
        -h --help
        --age aaaaaaaa
        --name bbbbbbbbb
        "#
        );
    }

    Ok(args)
}
```