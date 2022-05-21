use crate::{
    error::{Error, Result},
    ext::{InjectExt, MemExt, ScanExt, Type},
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

// 吐了 TODO 等个好心人重构
//
macro_rules! input_scan {
    ($app:ident,$t:expr,$input:expr) => {
        if let Err(e) = $app.input($t, $input) {
            println!("{}", e);
        } else {
            merr!($app.value_scan(), "搜索成功, 搜索到值为*的地址数: ", "搜索失败: Error: ");
        }
    };
}

macro_rules! match_input_type {
    ($n1:expr,$n2:expr,$input:expr,$app:ident) => {
        match $input[$n1] {
            "u8" => input_scan!($app, Type::U8, $input[$n2]),
            "u16" => input_scan!($app, Type::U16, $input[$n2]),
            "u32" => input_scan!($app, Type::U32, $input[$n2]),
            "u64" => input_scan!($app, Type::U64, $input[$n2]),
            "i8" => input_scan!($app, Type::I8, $input[$n2]),
            "i16" => input_scan!($app, Type::I16, $input[$n2]),
            "i32" => input_scan!($app, Type::I32, $input[$n2]),
            "i64" => input_scan!($app, Type::I64, $input[$n2]),
            "str" => input_scan!($app, Type::STR, $input[$n2]),
            _ => {
                input_scan!($app, Type::UNKNOWN, $input[$n1])
            }
        }
    };
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
                    if input.len() > 2 {
                        match input[1] {
                            "u8" => input_scan!(app, Type::U8, input[2]),
                            "u16" => input_scan!(app, Type::U16, input[2]),
                            "u32" => input_scan!(app, Type::U32, input[2]),
                            "u64" => input_scan!(app, Type::U64, input[2]),
                            "i8" => input_scan!(app, Type::I8, input[2]),
                            "i16" => input_scan!(app, Type::I16, input[2]),
                            "i32" => input_scan!(app, Type::I32, input[2]),
                            "i64" => input_scan!(app, Type::I64, input[2]),
                            "str" => input_scan!(app, Type::STR, input[2]),
                            "<" => {
                                match_input_type!(3, 4, input, app);
                                merr!(app.value_less(), "搜索成功, 搜索到值为*的地址数: ", "搜索失败: Error: ");
                            }
                            ">" => {
                                match_input_type!(3, 4, input, app);
                                merr!(app.value_more(), "搜索成功, 搜索到值为*的地址数: ", "搜索失败: Error: ");
                            }
                            _ => {}
                        }
                    } else {
                        if let Err(e) = app.input(Type::UNKNOWN, input[1]) {
                            println!("{}", e);
                        } else {
                            merr!(app.value_scan(), "搜索成功, 搜索到值为*的地址数: ", "搜索失败: Error: ");
                        };
                    }
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
                "clear" | "clean" => app.clear(),
                _ => {}
            }
        }
    }
}
