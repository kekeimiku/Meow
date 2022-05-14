use std::{thread::sleep, time::Duration};

use crate::{
    error::{Error, Result},
    mem::{InjectExt, Process, ScanExt, MemExt},
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
    let mut app = Process::new(pid).unwrap();
    loop {
        sleep(Duration::from_millis(50));
        let prompt = prompt("> ")?;
        let input = prompt.iter().map(String::as_str).collect::<Vec<&str>>();
        if input.is_empty() {
            println!("参数为空");
        } else {
            match input[0] {
                "find" => {
                    let input_val = &input[1].parse::<i32>()?.to_le_bytes();
                    app.cache.input = input_val.to_vec();
                    app.scan()?;
                }
                "status" => {
                    app.scan()?;
                }
                "p" => {
                    app.print()?;
                }
                "inject" => {
                    let libpath = &input[1];
                    app.inject(libpath)?;
                }
                "w"=>{
                    let addr = usize::from_str_radix(&input[1].replace("0x", ""), 16)?;
                    let input_val = &input[2].parse::<i32>()?.to_le_bytes();
                    app.write(addr, input_val);
                }
                _ => {}
            }
        }
    }
}
