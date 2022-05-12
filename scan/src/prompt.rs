use std::{process::exit, thread::sleep, time::Duration};

use crate::{
    error::{Error, Result},
    scan::MemScan,
};

pub fn prompt(name: &str) -> Result<Vec<String>> {
    let mut line = String::new();
    print!("{}", name);
    std::io::Write::flush(&mut std::io::stdout())?;
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
        .ok_or(Error::ArgsError)?
        .parse::<i32>()?;
    let mut app = MemScan::new(pid)?;
    loop {
        sleep(Duration::from_millis(50));
        let prompt = prompt("> ")?;
        let input = prompt.iter().map(String::as_str).collect::<Vec<&str>>();
        if input.is_empty() {
            println!("参数为空");
        } else {
            match input[0] {
                "find" => {
                    find(&mut app, &input)?;
                    app.list_abs_addr()
                }
                "<" => less(&mut app, &input),
                "read" => read(&mut app, &input)?,
                "laddr" => laddr(&mut app),
                "lmap" => lamp(&mut app)?,
                "clear" => app.clear(),
                "help" => help(),
                "exit" => exit(0),

                _ => {}
            }
        }
    }
}

fn find(app: &mut MemScan, input: &[&str]) -> Result<()> {
    if input.len() < 2 {
        println!("需要两个参数")
    } else if input[1] == "int" {
        let input_value = &input[2].parse::<i32>()?.to_le_bytes();
        app.input = input_value.to_vec();
        app.first_scan()?;
    }
    Ok(())
}

fn less(app: &mut MemScan, input: &[&str]) {
    match input.get(1) {
        Some(v) => {
            if v == &"self" {
                app.self_less();
            }
            if v == &"input" {
                app.input_less();
            }
        }
        None => println!("help"),
    }
}

fn read(app: &mut MemScan, input: &[&str]) -> Result<()> {
    let addr = usize::from_str_radix(&input[1].replace("0x", ""), 16)?;
    let v = app.read_bytes(addr, 4);
    println!("{:?}", v);
    Ok(())
}

fn laddr(app: &mut MemScan) {
    app.list_abs_addr();
}

fn lamp(app: &mut MemScan) -> Result<()> {
    app.list_maps()?;
    Ok(())
}

fn help() {
    println!("help")
}
