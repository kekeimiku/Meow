use crate::{
    error::{Error, Result},
    scan::MemScan,
};
use std::{
    io::Write,
    os::unix::prelude::FileExt,
    thread::{self, sleep},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

fn prompt(name: &str) -> Result<Vec<String>> {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut line)?;
    Ok(line
        .replace('\n', "")
        .split_whitespace()
        .map(String::from)
        .collect())
}

pub fn start() -> Result<()> {
    let pid = std::env::args()
        .nth(1)
        .ok_or(Error::SplitNextError)?
        .parse::<i32>()?;
    let mut app = MemScan::new(pid).unwrap();

    loop {
        let prompt = prompt("> ")?;
        let input = prompt.iter().map(String::as_str).collect::<Vec<&str>>();
        if input.is_empty() {
            println!("参数为空");
        } else {
            match input[0] {
                "find" => {
                    if input.len() < 2 {
                        println!("需要两个参数")
                    } else {
                        if input[1] == "int" {
                            let i = &input[2].parse::<u8>().unwrap().to_le_bytes();
                            app.search_all(i)?;
                            app.input = i.to_vec();
                            app.addr_list(10)
                        }
                        if input[1] == "str" {
                            let i = &input[2].as_bytes();
                            app.search_all(i)?;
                            app.input = i.to_vec();
                            app.addr_list(10)
                        }
                    }
                }

                "get" => {
                    if input.len() < 3 {
                        println!("需要两个参数 get address len")
                    } else {
                        let i = usize::from_str_radix(&input[1].replace("0x", ""), 16).unwrap();
                        let v = app.read_bytes(i, input[2].parse::<usize>()?);
                        println!("读取的值 {:?}", v);
                    }
                }

                "set" => {
                    if input.len() < 3 {
                        println!("需要两个参数")
                    } else {
                        let _ = app.write_bytes(
                            usize::from_str_radix(&input[1].replace("0x", ""), 16).unwrap(),
                            &input[2].parse::<i32>().unwrap().to_le_bytes(),
                        );
                    }
                }

                "<" => {
                    let start = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis();
                    app.change_mem()?;
                    let end = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis();

                    // println!("len: {}  耗时: {}", app.addr_cache.len(), end - start);
                    // println!("有变化");
                    // app.addr_list(10)
                }
                ">" => {
                    // println!("{:?}",&app.addr_cache);
                    println!("变大的值 && 发生了变化");
                }
                "=" => {
                    println!("没有发生变化的值");
                }
                "p" => {
                    println!("设置权限");
                }
                "reset" => {
                    println!("重新搜索");
                    app.reset()
                }
                "lock" => {
                    let i = usize::from_str_radix(&input[1].replace("0x", ""), 16).unwrap();
                    let b = app.input.clone();
                    let f = app.mem_file.try_clone()?;
                    thread::spawn(move || loop {
                        f.write_at(&b, i as u64).unwrap();
                        sleep(Duration::from_millis(200));
                    });
                }
                "list" => app.addr_list(10),
                "exit" | "quit" => std::process::exit(0),
                "help" => {
                    println!("help");
                }
                _ => {}
            };
        }
        sleep(Duration::from_millis(50));
    }
}
