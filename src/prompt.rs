use std::{
    env,
    fmt::format,
    fs::{self, File},
};

use utils::info;

use crate::{
    error::Result,
    platform::{parse_proc_maps, Mem, Region},
    region::RegionExt,
    scan::Scan, mem::MemExt,
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
    let pid = env::args().nth(1).unwrap();
    let handle = Mem::new(File::open(format!("/proc/{}/mem", pid)).unwrap());
    let v = parse_proc_maps(&fs::read_to_string(format!("/proc/{}/maps", pid)).unwrap()).unwrap();
    let region = v
        .iter()
        .filter(|x| x.pathname() == "[anon:libc_malloc]")
        .collect::<Vec<_>>()[0];

    let mut app = Scan::new(&handle, region).unwrap();

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
                    let v = app.list(region.start()).unwrap();
                    v.iter().for_each(|x| {
                        info!("0x{:x}", x);
                    });
                }

                "len" => {
                    info!("{}", app.len());
                }

                "w" => {
                    let arg1 = hexstr_to_usize(input[1]).unwrap();
                    let arg2 = input[1].parse::<i32>().unwrap().to_ne_bytes();
                    handle.write(arg1, &arg2).unwrap();
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