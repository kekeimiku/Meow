use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    path::Path,
    thread::spawn,
};

use crate::{
    comm::PID,
    maps::{readmaps_all_r, readmaps_all_rw, readmaps_c_alloc},
    sdiff::sorted_difference,
};

pub fn read_bytes(address: usize, size: usize) -> Result<Vec<u8>, io::Error> {
    let mut file = File::open(&Path::new(&format!("/proc/{}/mem", *PID)))?;
    file.seek(SeekFrom::Start(address as u64))?;
    let mut buffer = vec![0; size];
    file.read_exact(&mut buffer)?;
    Ok(buffer)
}

pub fn write_bytes(address: usize, payload: &[u8]) -> Result<usize, io::Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&Path::new(&format!("/proc/{}/mem", *PID)))?;
    file.seek(SeekFrom::Start(address as u64))?;
    file.write_all(payload)?;
    Ok(payload.len())
}

pub fn search_index(haystack: &[u8], needle: &[u8]) -> Vec<usize> {
    memchr::memmem::find_iter(haystack, needle).collect::<Vec<usize>>()
}

pub fn search_all_rw_mem(target: &[u8]) -> Vec<usize> {
    let mut s: Vec<usize> = Default::default();
    readmaps_all_rw().iter().for_each(|f| {
        let buf = read_bytes(f.start(), f.end() - f.start());
        let target = search_index(&buf.unwrap(), target)
            .iter()
            .map(|m| m + f.start())
            .collect::<Vec<usize>>();
        if !target.is_empty() {
            target.iter().for_each(|f| s.push(*f))
        }
    });
    s
}

pub fn search_all_r_mem(target: &[u8]) -> Vec<usize> {
    let mut s: Vec<usize> = Default::default();
    readmaps_all_r().iter().for_each(|f| {
        let buf = read_bytes(f.start(), f.end() - f.start());

        match buf {
            Ok(ok) => {
                let target = search_index(&ok, target)
                    .iter()
                    .map(|m| m + f.start())
                    .collect::<Vec<usize>>();
                if !target.is_empty() {
                    target.iter().for_each(|f| s.push(*f))
                }
            }
            Err(err) => println!(
                "搜索失败的区域 {:x}-{:x} {}  error: {}",
                f.start(),
                f.end(),
                f.pathname(),
                err
            ),
        }
    });
    s
}

pub fn search_c_alloc(target: &[u8]) -> Vec<usize> {
    let mut s: Vec<usize> = Default::default();
    readmaps_c_alloc().iter().for_each(|f| {
        let buf = read_bytes(f.start(), f.end() - f.start());
        let target = search_index(&buf.unwrap(), target)
            .iter()
            .map(|m| m + f.start())
            .collect::<Vec<usize>>();
        if !target.is_empty() {
            target.iter().for_each(|f| s.push(*f))
        }
    });
    s
}

pub fn freeze(list: Vec<usize>, payload: &'static [u8], flag: bool) {
    spawn(move || loop {
        if !flag {
            break;
        }
        list.iter().for_each(|f| {
            write_bytes(*f, payload).unwrap();
        });
    });
}

pub fn x1(i1: Vec<usize>, i2: Vec<usize>) -> Vec<usize> {
    let mut it1 = i1;
    let mut it2 = i2;
    it1.sort_unstable();
    it2.sort_unstable();
    sorted_difference(it2.iter(), it1.iter())
        .copied()
        .collect::<Vec<usize>>()
}
