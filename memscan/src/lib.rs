use std::{
    collections::{HashMap, HashSet},
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

use memchr::memmem::find_iter;

pub mod error;
pub mod maps;

use crate::{
    error::{Error, Result},
    maps::{parse_proc_maps, MapRange},
};

pub struct MemScan {
    mem_file: File,
    maps_file: File,
}

impl MemScan {
    pub fn new(pid: i32) -> Self {
        let mem_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&Path::new(&format!("/proc/{}/mem", pid)))
            .unwrap();
        let maps_file = File::open(format!("/proc/{}/maps", pid)).unwrap();

        Self {
            mem_file,
            maps_file,
        }
    }

    pub fn read_bytes(&mut self, address: usize, size: usize) -> Result<Vec<u8>> {
        self.mem_file.seek(SeekFrom::Start(address as u64))?;
        let mut buffer = vec![0; size];
        self.mem_file.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    pub fn write_bytes(&mut self, address: usize, payload: &[u8]) -> Result<usize> {
        self.mem_file.seek(SeekFrom::Start(address as u64))?;
        self.mem_file.write_all(payload)?;
        Ok(payload.len())
    }

    fn readmaps_all(&mut self) -> Result<Vec<MapRange>> {
        let mut contents = String::new();
        self.maps_file.read_to_string(&mut contents)?;
        Ok(parse_proc_maps(&contents)?
            .into_iter()
            .filter(|m| m.is_read() && m.is_write())
            .collect::<Vec<MapRange>>())
    }

    pub fn search_all_mem(&mut self, v: &[u8]) -> Result<HashSet<(usize, Vec<u8>)>> {
        //        let mut hs: HashMap<usize, Vec<u8>> = std::collections::HashMap::new();

        let mut hs: HashSet<(usize, Vec<u8>)> = HashSet::new();

        self.readmaps_all()?.iter().try_for_each(|f| {
            let vl = find_iter(&self.read_bytes(f.start(), f.end() - f.start()).ok()?, v)
                .map(|m| m + f.start())
                .collect::<Vec<usize>>();
            if !vl.is_empty() {
                vl.iter().for_each(|a| {
                    hs.insert((
                        *a,
                        self.read_bytes(*vl.iter().next().unwrap(), v.len())
                            .unwrap(),
                    ));
                });

                return None;
            }
            Some(())
        });

        Ok(hs)
    }
}

pub fn get_pid_by_name(name: &str) -> Result<i32> {
    let mut pid: i32 = -1;
    for process in fs::read_dir("/proc")? {
        let comm = format!("{}/comm", process?.path().display());
        let file = File::open(Path::new(&comm));
        if let Ok(mut f) = file {
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            if s.trim() == name {
                pid = comm.split('/').collect::<Vec<&str>>()[2].parse::<i32>()?;
            }
        }
    }

    if pid == -1 {
        return Err(Error::PidNotFound);
    }

    Ok(pid)
}

extern "C" {
    pub fn mprotect(addr: *mut core::ffi::c_void, len: usize, prot: i32) -> i32;
}
