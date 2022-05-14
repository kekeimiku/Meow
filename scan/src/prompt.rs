use std::{thread::sleep, time::Duration};

use crate::{
    error::{Error, Result},
    mem::{InjectExt, Process, ScanExt},
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
                    app.scan(input_val.to_vec(), 1)?;
                    // find(&mut app, &input)?;
                    // app.list_abs_addr()
                }
                "status" => {
                    let input_val = &input[1].parse::<i32>()?.to_le_bytes();
                    app.scan(input_val.to_vec(), 3)?;
                }
                "p" => {
                    let input_val = &input[1].parse::<i32>()?.to_le_bytes();
                    app.scan(input_val.to_vec(), 5)?;
                }
                "inject" => {
                    let libpath = &input[1];
                    app.inject(libpath)?;
                }
                // "<" => less(&mut app, &input),
                // "read" => read(&mut app, &input)?,
                // "laddr" => laddr(&mut app),
                // "lmap" => lamp(&mut app)?,
                // "clear" => app.clear(),
                // "help" => help(),
                // "exit" => exit(0),
                _ => {}
            }
        }
    }
}
