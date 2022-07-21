use std::env;

use utils::{debug, info};

use crate::{error::Result, mem::MemExt, platform::get_memory_handle, scan::Scan};

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
    let pid = env::args().nth(1).unwrap().parse::<u32>().unwrap();

    #[cfg(any(target_os = "linux", target_os = "android"))]
    let region =
        crate::platform::get_region_range(&std::fs::read_to_string(format!("/proc/{}/maps", pid)).unwrap())
            .unwrap()
            .into_iter()
            .filter(|m| m.is_write() && m.is_read())
            .collect::<Vec<_>>();
    #[cfg(any(target_os = "linux", target_os = "android"))]
    let handle = get_memory_handle(pid).unwrap();

    #[cfg(target_os = "windows")]
    let handle = get_memory_handle(pid).unwrap();
    #[cfg(target_os = "windows")]
    let region = crate::platform::get_region_range(handle.handle).unwrap();

    let mut app = Scan::new(&handle, &region).unwrap();

    loop {
        let prompt = prompt("> ")?;
        let input = prompt.iter().map(String::as_str).collect::<Vec<&str>>();
        if input.is_empty() {
            println!("参数为空");
        } else {
            match input[0] {
                "find" => {
                    let arg1 = input[1].parse::<i32>().unwrap().to_ne_bytes();
                    app.find(&arg1)?;
                    info!("{}", app.len());
                }

                "re" => {
                    let arg1 = input[1].parse::<i32>().unwrap().to_ne_bytes();
                    app.refind(&arg1)?;
                    info!("{}", app.len());
                }
                "p" => {
                    app.list().unwrap();
                }

                "len" => {
                    info!("{}", app.len());
                }

                "w" => {
                    let arg1 = hexstr_to_usize(input[1]).unwrap();
                    let arg2 = input[2].parse::<i32>().unwrap().to_ne_bytes();
                    handle.write(arg1, &arg2).unwrap();
                }
                "r" => {
                    let arg1 = hexstr_to_usize(input[1]).unwrap();
                    debug!("{}", arg1);
                    let a = handle.read(arg1, 4).unwrap();
                    debug!("{}", i32::from_ne_bytes(a.try_into().unwrap()));
                }
                "q" => {
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

pub fn hexstr_to_usize(s: &str) -> Result<usize> {
    Ok(usize::from_str_radix(&s.replace("0x", ""), 16)?)
}
