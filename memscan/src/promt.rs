use crate::MemScan;
use crate::Result;
use std::io::Write;

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
    let pid = std::env::args().nth(1).unwrap().parse::<i32>()?;
    let mut app = MemScan::new(pid);

    loop {
        let prompt = prompt("> ")?;
        let input = prompt.iter().map(String::as_str).collect::<Vec<&str>>();
        if input.is_empty() {
            println!("参数为空");
        } else {
            match input[0] {
                "find" => {
                    if input.len() < 2 {
                        println!("需要一个参数")
                    } else {
                        let i = &input[1].parse::<i32>().unwrap().to_be_bytes();
                        app.search_all(i)?;
                        app.v = i.to_vec();
                        println!("搜索的值");
                        app.list()
                    }
                }

                "get" => {
                    if input.len() < 3 {
                        println!("需要两个参数 get address len")
                    } else {
                        let v =
                            app.read_bytes(input[1].parse::<usize>()?, input[2].parse::<usize>()?);
                        println!("读取的值 {:?}", v);
                    }
                }

                "set" => {
                    if input.len() < 3 {
                        println!("需要两个参数")
                    } else {
                        let o = app.write_bytes(input[1].parse::<usize>()?, input[2].as_bytes());
                        println!("写入返回值 {:?}", o);
                    }
                }
                "<" => {
                    // dbg!(&app.addr_cache);
                    dbg!(&app.v);
                    app.change_mem().unwrap();
                    println!("变小的值 && 发生了变化");
                    app.list()
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
                    let _ = app.addr_cache.clear();
                }
                "list" => {
                    dbg!(&app.addr_cache);
                }
                "exit" | "quit" => std::process::exit(0),

                _ => {}
            };
        }
    }
}
