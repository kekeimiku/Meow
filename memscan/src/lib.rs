use std::{
    collections::{HashMap, HashSet},
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    os::unix::fs::FileExt,
    path::Path,
};

use memchr::memmem::find_iter;

pub mod error;
mod libc;
pub mod maps;

use crate::libc::pvr as process_vm_readv;
use crate::{
    error::{Error, Result},
    maps::{parse_proc_maps, MapRange},
};

pub struct MemScan {
    mem_file: File,
    maps_file: File,
    pid: i32,
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
            pid,
        }
    }

    pub fn read_bytes(&self, addr: usize, size: usize) -> Result<Vec<u8>> {
        // if size > 1024*1024*30{}
        let mut buf = vec![0; size];
        process_vm_readv(self.pid, addr, &mut buf)?;
        Ok(buf)
    }

    pub fn write_bytes(&self, addr: usize, payload: &[u8]) -> Result<usize> {
        self.mem_file.write_at(payload, addr as u64)?;
        Ok(payload.len())
    }

    fn readmaps_all(&mut self) -> Result<Vec<MapRange>> {
        let mut contents = String::new();
        // TODO read_at_to_string
        self.maps_file.read_to_string(&mut contents)?;
        self.maps_file.rewind();
        Ok(parse_proc_maps(&contents)?
            .into_iter()
            .filter(|m| m.is_read() )
            .collect::<Vec<MapRange>>())
    }

    // TODO 简化代码 分多种类型 ca cheap
    pub fn search_all_mem(&mut self, v: &[u8]) -> Result<Vec<usize>> {
        let mut vv = Vec::new();
        for f in self.readmaps_all()?.iter() {
            //dbg!(f.end()-f.start());
               let a =  find_iter(&self.read_bytes(f.start(), f.end() - f.start())?, v)
                    .map(|m| m + f.start())
                    .collect::<Vec<usize>>();

            for q in a {
                vv.push(q);
            }
        }
        Ok(vv)
    }
}

// TODO 改善性能
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
