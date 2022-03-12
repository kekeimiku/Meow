use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    os::unix::prelude::OsStrExt,
    path::Path,
};

use crate::{
    comm::PID,
    maps::{readmaps_all_r, readmaps_all_rw, readmaps_c_alloc},
};

pub fn read_bytes(address: usize, size: usize) -> Result<Vec<u8>, io::Error> {
    let mut file = File::open(&Path::new(&format!("/proc/{}/mem", unsafe { PID })))?;
    file.seek(SeekFrom::Start(address as u64))?;
    let mut buffer = vec![0; size];
    file.read(&mut buffer)?;
    Ok(buffer)
}

pub fn write_bytes(address: usize, payload: &[u8]) -> Result<usize, io::Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&Path::new(&format!("/proc/{}/mem", unsafe { PID })))?;
    file.seek(SeekFrom::Start(address as u64))?;
    file.write_all(payload)?;
    Ok(payload.len())
}

pub fn search_index(buf: &[u8], target: &[u8]) -> Vec<usize> {
    memchr::memmem::find_iter(buf, target).collect::<Vec<usize>>()
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
