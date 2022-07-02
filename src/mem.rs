use crate::error::Result;

use std::os::unix::prelude::FileExt;

pub trait MemExt {
    fn read(&self, addr: usize, size: usize) -> Result<Vec<u8>>;
    fn write(&self, addr: usize, payload: &[u8]) -> Result<usize>;
}

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
}

impl<'a, T> MemExt for Mem<'a, T>
where
    T: FileExt,
{
    fn read(&self, addr: usize, size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; size];
        self.handle.read_at(&mut buf, addr as u64)?;
        Ok(buf)
    }

    fn write(&self, addr: usize, payload: &[u8]) -> Result<usize> {
        self.handle.write_at(payload, addr as u64)?;
        Ok(payload.len())
    }
}

#[derive(Debug)]
pub struct Chunks<'a, T: MemExt> {
    mem: &'a T,
    start: usize,
    size: usize,
    num: usize,
    last: usize,
}

impl<'a, T> Chunks<'a, T>
where
    T: MemExt,
{
    pub fn new(mem: &'a T, start: usize, end: usize, mut size: usize) -> Self {
        let mut last = 0;
        let mut num = 1;
        if size < end - start {
            last = (end - start) % size;
            num = (end - start) / size;
        } else {
            size = end - start;
        }
        Self {
            mem,
            start,
            size,
            num,
            last,
        }
    }
}

impl<T> Iterator for Chunks<'_, T>
where
    T: MemExt,
{
    type Item = Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num != 0 {
            match self.mem.read(self.start, self.size) {
                Ok(chunk) => {
                    self.start += self.size;
                    self.num -= 1;
                    return Some(Ok(chunk));
                }
                Err(err) => return Some(Err(err)),
            };
        }

        if self.last != 0 {
            match self.mem.read(self.start, self.last) {
                Ok(chunk) => {
                    self.last = 0;
                    return Some(Ok(chunk));
                }
                Err(err) => return Some(Err(err)),
            };
        }

        None
    }
}
