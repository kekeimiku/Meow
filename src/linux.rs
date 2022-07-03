use std::os::unix::prelude::FileExt;

use crate::{error::Result, mem::{MemExt, Chunks}};

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
