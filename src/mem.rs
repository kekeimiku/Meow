use std::{
    convert::AsRef,
    fs::{File, OpenOptions},
    io::{Error, Read, Seek, SeekFrom, Write},
    path::Path,
    thread::spawn,
};

use memchr::memmem::find_iter;

use crate::{
    comm::PID,
    maps::{readmaps_all_r, readmaps_all_rw, readmaps_c_alloc},
    sdiff::sorted_difference,
};

pub fn read_bytes(address: usize, size: usize) -> Result<Vec<u8>, Error> {
    let mut file = File::open(&Path::new(&format!("/proc/{}/mem", *PID)))?;
    file.seek(SeekFrom::Start(address as u64))?;
    let mut buffer = vec![0; size];
    file.read_exact(&mut buffer)?;
    Ok(buffer)
}

pub fn write_bytes(address: usize, payload: &[u8]) -> Result<usize, Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&Path::new(&format!("/proc/{}/mem", *PID)))?;
    file.seek(SeekFrom::Start(address as u64))?;
    file.write_all(payload)?;
    Ok(payload.len())
}

pub fn search_all_rw_mem<V>(v: &V) -> Result<Vec<usize>, Error>
where
    V: ?Sized + AsRef<[u8]>,
{
    for f in readmaps_all_rw()?.iter() {
        let vl = find_iter(&read_bytes(f.start(), f.end() - f.start())?, v)
            .map(|m| m + f.start())
            .collect::<Vec<usize>>();
        if !vl.is_empty() {
            return Ok(vl.into_iter().collect::<Vec<usize>>());
        }
    }
    Ok(Default::default())
}

pub fn search_all_r_mem<V>(v: &V) -> Result<Vec<usize>, Error>
where
    V: ?Sized + AsRef<[u8]>,
{
    for f in readmaps_all_r()?.iter() {
        let vl = find_iter(&read_bytes(f.start(), f.end() - f.start())?, v)
            .map(|m| m + f.start())
            .collect::<Vec<usize>>();
        if !vl.is_empty() {
            return Ok(vl.into_iter().collect::<Vec<usize>>());
        }
    }
    Ok(Default::default())
}

pub fn search_c_alloc<V>(v: &V) -> Result<Vec<usize>, Error>
where
    V: ?Sized + AsRef<[u8]>,
{
    for f in readmaps_c_alloc()?.iter() {
        let vl = find_iter(&read_bytes(f.start(), f.end() - f.start())?, &v)
            .map(|m| m + f.start())
            .collect::<Vec<usize>>();
        if !vl.is_empty() {
            return Ok(vl.into_iter().collect::<Vec<usize>>());
        }
    }
    Ok(Default::default())
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
