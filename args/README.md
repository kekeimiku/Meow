仅为满足个人需求的命令行解析库

特点：轻量，非常小的二进制开销（x64 12kb）。

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