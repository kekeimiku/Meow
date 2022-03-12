use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

pub static mut PID: i32 = -1;

pub fn get_pid_by_name(name: &str) -> Option<Vec<i32>> {
    let mut pid: Vec<i32> = Vec::new();
    for process in fs::read_dir("/proc").unwrap() {
        let comm = format!("{}/comm", process.unwrap().path().display());
        let file = File::open(Path::new(&comm));
        if let Ok(mut f) = file {
            let mut s = String::new();
            f.read_to_string(&mut s).unwrap();
            if s.trim() == name {
                let split = comm.split("/").collect::<Vec<&str>>();
                pid.push(split[2].parse::<i32>().unwrap());
            }
        }
    }

    if pid.is_empty() {
        None
    } else {
        Some(pid)
    }
}
