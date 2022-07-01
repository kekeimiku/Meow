use crate::error::Result;

use std::{fs::File, io::Write, os::unix::prelude::FileExt, path::Path};

// 文件分块的大小 默认8mb
const CHUNK_SIZE: usize = 8192;

pub struct Mem<'a, T: FileExt> {
    pub handle: &'a T,
}

impl<'a, T> Mem<'a, T>
where
    T: FileExt,
{
    pub fn new(handle: &'a T) -> Mem<T> {
        Self { handle }
    }

    pub fn read(&self, addr: usize, size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; size];
        self.handle.read_at(&mut buf, addr as u64)?;
        Ok(buf)
    }

    pub fn write(&self, addr: usize, payload: &[u8]) -> Result<usize> {
        self.handle.write_at(payload, addr as u64)?;
        Ok(payload.len())
    }

    pub fn dump(&self, addr: usize, size: usize, path: &str) -> Result<usize> {
        let mut file = File::create(Path::new(path))?;
        let buf = self.read(addr, size)?;
        file.write_all(&buf)?;
        Ok(buf.len())
    }

    pub fn find_region_addr(&self, start: usize, end: usize, value: &[u8]) -> Result<Vec<usize>> {
        find_region_addr(self.handle, start, end, CHUNK_SIZE, value)
    }
}

pub fn find_region_addr<T: FileExt>(
    handle: &T,
    start: usize,
    end: usize,
    size: usize,
    value: &[u8],
) -> Result<Vec<usize>> {
    let mut num = 0;
    Chunks::new(handle, start, end, size)
        .into_iter()
        .try_fold(Vec::default(), |mut init, next| {
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
pub struct Chunks<'a, T: FileExt> {
    handle: &'a T,
    start: usize,
    size: usize,
    num: usize,
    last: usize,
}

impl<'a, T> Chunks<'a, T>
where
    T: FileExt,
{
    pub fn new(handle: &'a T, start: usize, end: usize, size: usize) -> Self {
        Self {
            handle,
            start,
            size,
            num: (end - start) / size,
            last: (end - start) % size,
        }
    }
}

impl<T> Iterator for Chunks<'_, T>
where
    T: FileExt,
{
    type Item = std::io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num != 0 {
            let mut chunk = vec![0; self.size];
            match self.handle.read_at(&mut chunk, self.start as u64) {
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
            match self.handle.read_at(&mut chunk, self.start as u64) {
                Ok(_) => {
                    self.last = 0;
                    return Some(Ok(chunk));
                }
                Err(e) => return Some(Err(e)),
            };
        }

        None
    }
}
