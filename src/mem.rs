use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    path::Path,
};

use crate::def::Cheat;

impl Cheat {
    pub fn read_bytes(&self, address: usize, size: usize) -> Result<Vec<u8>, io::Error> {
        let mut file = File::open(&Path::new(&format!("/proc/{}/mem", self.pid)))?;
        file.seek(SeekFrom::Start(address as u64))?;
        let mut buffer = vec![0; size];
        file.read(&mut buffer)?;
        Ok(buffer)
    }

    pub fn write_bytes(&self, address: usize, payload: &[u8]) -> Result<usize, io::Error> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&Path::new(&format!("/proc/{}/mem", self.pid)))?;
        file.seek(SeekFrom::Start(address as u64))?;
        file.write_all(payload)?;
        Ok(payload.len())
    }

    pub fn search_index(buf: &[u8], target: &[u8]) -> Vec<usize> {
        memchr::memmem::find_iter(buf, target).collect::<Vec<usize>>()
    }
}


