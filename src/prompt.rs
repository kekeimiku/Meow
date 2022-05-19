use crate::{
    error::{Error, Result},
    ext::{InjectExt, MemExt, ScanExt},
    utils::hexstr_to_usize,
};

#[cfg(target_os = "linux")]
use crate::linux::Linux;

#[cfg(target_os = "windows")]
use crate::windows::Windows;

#[cfg(target_os = "macos")]
use crate::macos::Macos;

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
    let pid = std::env::args().nth(1).ok_or(Error::ArgsError)?.parse::<u32>()?;
    #[cfg(target_os = "linux")]
    let mut app = Linux::new(pid)?;

    #[cfg(target_os = "windows")]
    let mut app = Windows::new(pid)?;

    #[cfg(target_os = "macos")]
    let mut app = Macos::new();

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
                    let addr = hexstr_to_usize(input[1])?;
                    let val = &input[2].parse::<i32>()?.to_le_bytes();
                    merr!(app.write(addr, val), "写入成功,字节数: ", "写入失败: Error: ");
                }
                "read" | "r" => {
                    let addr = hexstr_to_usize(input[1])?;
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
                    let addr = hexstr_to_usize(input[1])?;
                    let payload = &input[2].parse::<i32>()?.to_le_bytes();
                    app.freeze(addr, payload.to_vec())?;
                }
                "dump" => {
                    let addr = hexstr_to_usize(input[1])?;
                    let size = input[2].parse::<usize>()?;
                    let path = input[3];
                    app.dump(addr, size, path)?;
                }
                _ => {}
            }
        }
    }
}
