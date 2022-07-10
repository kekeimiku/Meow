use std::{
    env,
    fs::{self, File},
};

use utils::{warn, info};

use crate::{
    error::Result,
    platform::{parse_proc_maps, Mem},
    region::RegionExt,
    scan::{find_addr_by_region, scan_region},
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

struct Data {
    vec: Vec<Vec<Vec<u16>>>,
}

struct Data1 {
    vec: Vec<Vec<usize>>,
}

pub fn start() -> Result<()> {
    let pid = env::args().nth(1).unwrap();
    let mut app = Data { vec: Vec::default() };
    loop {
        let prompt = prompt("> ")?;
        let input = prompt.iter().map(String::as_str).collect::<Vec<&str>>();
        if input.is_empty() {
            println!("参数为空");
        } else {
            match input[0] {
                "find" => {
                    let num = input[1].parse::<u8>()?.to_ne_bytes();
                    let handle = Mem::from(File::open(format!("/proc/{}/mem", pid)).unwrap());
                    let regions =
                        parse_proc_maps(&fs::read_to_string(format!("/proc/{}/maps", pid)).unwrap()).unwrap();
                    for reg in regions {
                        if reg.is_write() && reg.is_read() {
                            let vv = scan_region(&handle, reg.start(), reg.end(), &num);
                            if let Ok(vec) = vv {
                                app.vec.push(vec);
                            } else {
                                warn!("搜索出错 0x{:x}-0x{:x} {}", reg.start(), reg.end(), reg.pathname());
                            }
                        }
                    }
                }

                "len" => {
                    let sum: usize = app
                        .vec
                        .iter()
                        .map(|v| v.iter().map(|v| v.len()).sum::<usize>())
                        .sum();
                    info!("{}", sum);
                }
                _ => {}
            }
        }
    }
    Ok(())
}

// pub fn start() -> Result<()> {
//     let pid = env::args().nth(1).unwrap();
//     let mut app = Data1 { vec: Vec::default() };
//     loop {
//         let prompt = prompt("> ")?;
//         let input = prompt.iter().map(String::as_str).collect::<Vec<&str>>();
//         if input.is_empty() {
//             println!("参数为空");
//         } else {
//             match input[0] {
//                 "find" => {
//                     let num = input[1].parse::<u8>()?.to_ne_bytes();
//                     let handle = Mem::from(File::open(format!("/proc/{}/mem", pid)).unwrap());
//                     let regions =
//                         parse_proc_maps(&fs::read_to_string(format!("/proc/{}/maps", pid)).unwrap()).unwrap();
//                     for reg in regions {
//                         if reg.is_read() && reg.is_write() {
//                             let vv = find_addr_by_region(&handle, reg.start(), reg.end(), &num);
//                             if let Ok(vec) = vv {
//                                 app.vec.push(vec);
//                             } else {
//                                 println!("搜索出错 0x{:x}-0x{:x} {}", reg.start(), reg.end(), reg.pathname());
//                             }
//                         }
//                     }
//                 }
//                 "len" => {
//                     let sum: usize = app.vec.iter().map(|v| v.len()).sum();
//                     println!("{}", sum);
//                 }
//                 _ => {}
//             }
//         }
//     }
//     Ok(())
// }
