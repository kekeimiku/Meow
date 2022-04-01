use std::{
    fs::File,
    io::{Read, Result},
};

use crate::comm::PID;

#[derive(Debug)]
pub struct MapRange {
    pub range_start: usize,
    pub range_end: usize,
    pub offset: usize,
    pub dev: String,
    pub flags: String,
    pub inode: usize,
    pub pathname: String,
}

impl MapRange {
    pub fn size(&self) -> usize {
        self.range_end - self.range_start
    }
    pub fn start(&self) -> usize {
        self.range_start
    }
    pub fn end(&self) -> usize {
        self.range_end
    }
    pub fn pathname(&self) -> &String {
        &self.pathname
    }
    pub fn is_exec(&self) -> bool {
        &self.flags[2..3] == "x"
    }
    pub fn is_write(&self) -> bool {
        &self.flags[1..2] == "w"
    }
    pub fn is_read(&self) -> bool {
        &self.flags[0..1] == "r"
    }
}

pub fn get_process_maps() -> Result<Vec<MapRange>> {
    let maps_file = format!("/proc/{}/maps", *PID);
    let mut file = File::open(maps_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(parse_proc_maps(&contents))
}

pub fn parse_proc_maps(contents: &str) -> Vec<MapRange> {
    let mut vec: Vec<MapRange> = Vec::new();
    for line in contents.split('\n') {
        let mut split = line.split_whitespace();
        let range = split.next();
        if range.is_none() {
            break;
        }
        let mut range_split = range.unwrap().split('-');
        let range_start = range_split.next().unwrap();
        let range_end = range_split.next().unwrap();
        let flags = split.next().unwrap();
        let offset = split.next().unwrap();
        let dev = split.next().unwrap();
        let inode = split.next().unwrap();

        vec.push(MapRange {
            range_start: usize::from_str_radix(range_start, 16).unwrap(),
            range_end: usize::from_str_radix(range_end, 16).unwrap(),
            offset: usize::from_str_radix(offset, 16).unwrap(),
            dev: dev.to_string(),
            flags: flags.to_string(),
            inode: inode.parse::<usize>().unwrap(),
            pathname: split.collect::<Vec<&str>>().join(" "),
        });
    }
    vec
}

pub fn readmaps_c_alloc() -> Result<Vec<MapRange>> {
    Ok(get_process_maps()?
        .into_iter()
        .filter(|m| m.pathname() == "[anon:libc_malloc]" && m.is_read() && m.is_write())
        .collect::<Vec<MapRange>>())
}

pub fn readmaps_c_bss() -> Result<Vec<MapRange>> {
    Ok(get_process_maps()?
        .into_iter()
        .filter(|m| m.pathname() == "[anon:.bss]" && m.is_read() && m.is_write())
        .collect::<Vec<MapRange>>())
}

pub fn readmaps_c_data() -> Result<Vec<MapRange>> {
    Ok(get_process_maps()?
        .into_iter()
        .filter(|m| m.pathname() == "/data/app/" && m.is_read() && m.is_write())
        .collect::<Vec<MapRange>>())
}

pub fn readmaps_java_heap() -> Result<Vec<MapRange>> {
    Ok(get_process_maps()?
        .into_iter()
        .filter(|m| m.pathname() == "/dev/ashmem/" && m.is_read() && m.is_write())
        .collect::<Vec<MapRange>>())
}

// pathname is none
pub fn readmaps_a_anonmyous() -> Result<Vec<MapRange>> {
    Ok(get_process_maps()?
        .into_iter()
        .filter(|m| m.pathname().is_empty() && m.is_read() && m.is_write())
        .collect::<Vec<MapRange>>())
}

pub fn readmaps_code_system() -> Result<Vec<MapRange>> {
    Ok(get_process_maps()?
        .into_iter()
        .filter(|m| m.pathname() == "/system" && m.is_read() && m.is_write())
        .collect::<Vec<MapRange>>())
}

pub fn readmaps_ashmem() -> Result<Vec<MapRange>> {
    Ok(get_process_maps()?
        .into_iter()
        .filter(|m| m.pathname() == "/dev/ashmem/" && m.is_read() && m.is_write())
        .collect::<Vec<MapRange>>())
}

pub fn readmaps_code_app() -> Result<Vec<MapRange>> {
    Ok(get_process_maps()?
        .into_iter()
        .filter(|m| m.pathname() == "/data/app/" && m.is_read() && !m.is_write() && m.is_exec())
        .collect::<Vec<MapRange>>())
}

pub fn readmaps_stack() -> Result<Vec<MapRange>> {
    Ok(get_process_maps()?
        .into_iter()
        .filter(|m| m.pathname() == "[stack]" && m.is_read() && m.is_write())
        .collect::<Vec<MapRange>>())
}

pub fn readmaps_c_heap() -> Result<Vec<MapRange>> {
    Ok(get_process_maps()?
        .into_iter()
        .filter(|m| m.pathname() == "[heap]" && m.is_read() && m.is_write())
        .collect::<Vec<MapRange>>())
}

pub fn readmaps_other() -> Result<Vec<MapRange>> {
    Ok(get_process_maps()?
        .into_iter()
        .filter(|m| m.pathname() == "[anon:thread signal stack]" && m.is_read() && m.is_write())
        .collect::<Vec<MapRange>>())
}

pub fn readmaps_all_rw() -> Result<Vec<MapRange>> {
    Ok(get_process_maps()?
        .into_iter()
        .filter(|m| m.is_read() && m.is_write())
        .collect::<Vec<MapRange>>())
}

pub fn readmaps_all_r() -> Result<Vec<MapRange>> {
    Ok(get_process_maps()?
        .into_iter()
        .filter(|m| m.is_read() && m.pathname() != "[vvar]")
        .collect::<Vec<MapRange>>())
}