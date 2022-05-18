use crate::{
    error::{Error, Result},
    mem::{InjectExt, Linux, MemExt, ScanExt},
};

macro_rules! merr {
    ($m:expr,$ok:expr,$err:expr) => {
        match $m {
            Ok(v) => println!("{} {:?}", $ok, v),
            Err(e) => println!("{} {}", $err, e),
        }
    };
}

pub fn prompt(name: &str) -> Result<Vec<String>> {
    let mut line = String::new();
    print!("{}", name);
    std::io::Write::flush(&mut std::io::stdout())?;
    std::io::stdin().read_line(&mut line)?;
    Ok(line.replace('\n', "").split_whitespace().map(String::from).collect())
}

pub fn start() -> Result<()> {
    let pid = std::env::args().nth(1).ok_or(Error::ArgsError)?.parse::<i32>()?;
    let mut app = Linux::new(pid).unwrap();
    loop {
        let prompt = prompt("> ")?;
        let input = prompt.iter().map(String::as_str).collect::<Vec<&str>>();
        if input.is_empty() {
            println!("参数为空");
        } else {
            match input[0] {
                "find" | "f" => {
                    let val = &input[1].parse::<i32>()?.to_le_bytes();
                    app.input(val);
                    merr!(app.scan(), "搜索成功,总条数: ", "搜索失败: Error: ");
                }
                "write" | "w" => {
                    let addr = usize::from_str_radix(&input[1].replace("0x", ""), 16)?;
                    let val = &input[2].parse::<i32>()?.to_le_bytes();
                    merr!(app.write(addr, val), "写入成功,字节数: ", "写入失败: Error: ");
                }
                "read" | "r" => {
                    let addr = usize::from_str_radix(&input[1].replace("0x", ""), 16)?;
                    let size = &input[2].parse::<usize>()?;
                    merr!(app.read(addr, *size), "", "Error: ");
                }
                "print" | "p" => {
                    app.print()?;
                }
                "inject" | "inj" => {
                    let libpath = &input[1];
                    merr!(app.inject(libpath), "注入成功", "注入失败,Error: ");
                }
                "lock" => {
                    let addr = usize::from_str_radix(&input[1].replace("0x", ""), 16)?;
                    let payload = &input[2].parse::<i32>()?.to_le_bytes();
                    app.freeze(addr, payload.to_vec())?;
                }
                _ => {}
            }
        }
    }
}
