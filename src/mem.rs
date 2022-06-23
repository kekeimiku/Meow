use crate::error::Result;

use std::{
    fs::File,
    io::{Read, Write},
    os::unix::prelude::FileExt,
    path::Path,
};

// 文件分块的大小 默认4mb，不要瞎鸡巴动它
const CHUNK_SIZE: usize = 4096;

pub struct MemScan<'a, T: Read + FileExt> {
    pub file: &'a T,
}

impl<'a, T> MemScan<'a, T>
where
    T: Read + FileExt,
{
    pub fn new(file: &'a T) -> MemScan<T> {
        Self { file }
    }

    pub fn read(&self, addr: usize, size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; size];
        self.file.read_at(&mut buf, addr as u64)?;
        Ok(buf)
    }

    pub fn write(&self, addr: usize, payload: &[u8]) -> Result<usize> {
        self.file.write_at(payload, addr as u64)?;
        Ok(payload.len())
    }

    pub fn dump(&self, addr: usize, size: usize, path: &str) -> Result<usize> {
        let mut file = File::create(Path::new(path))?;
        let buf = self.read(addr, size)?;
        file.write_all(&buf)?;
        Ok(buf.len())
    }

    pub fn find_region_addr(&self, start: usize, end: usize, value: &[u8]) -> Result<Vec<usize>> {
        find_region_addr(self.file, start, end, CHUNK_SIZE, value)
    }
}

pub fn find_region_addr<T: Read + FileExt>(
    file: &T,
    start: usize,
    end: usize,
    size: usize,
    value: &[u8],
) -> Result<Vec<usize>> {
    let mut num = 0;
    Chunks::new(file, start, end, size)
        .into_iter()
        .try_fold(vec![], |mut init, next| {
            init.extend(
                next?
                    .windows(value.len())
                    .enumerate()
                    .step_by(value.len())
                    .filter_map(|(k, v)| if v == value { Some(k + num) } else { None })
                    .collect::<Vec<_>>(),
            );
            num += size;
            Ok(init)
        })
}

#[derive(Debug)]
struct Chunks<'a, T: Read> {
    file: &'a T,
    start: usize,
    size: usize,
    num: usize,
    last: usize,
}

impl<'a, T> Chunks<'a, T>
where
    T: Read,
{
    fn new(file: &'a T, start: usize, end: usize, size: usize) -> Self {
        Self {
            file,
            start,
            size,
            num: (end - start) / size,
            last: (end - start) % size,
        }
    }
}

impl<T> Iterator for Chunks<'_, T>
where
    T: Read + FileExt,
{
    type Item = std::io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chunk = vec![0; self.num];
        if self.num != 0 {
            match self.file.read_at(&mut chunk, self.start as u64) {
                Ok(_) => {
                    self.start += self.size;
                    self.num -= 1;
                    return Some(Ok(chunk));
                }
                Err(e) => return Some(Err(e)),
            };
        }

        if self.last != 0 {
            let mut chunk = vec![0; self.last];
            match self.file.read_at(&mut chunk, self.start as u64) {
                Ok(_) => {
                    self.last = 0;
                    return Some(Ok(chunk));
                }
                Err(e) => return Some(Err(e)),
            };
        }

        return None;
    }
}
