use crate::error::Result;

use std::{fs::File, io::Write, os::unix::prelude::FileExt, path::Path};
use utils::debug;

pub struct Mem<'a> {
    pub file: &'a File,
}

impl<'a> Mem<'a> {
    pub fn new(file: &'a File) -> Mem {
        Mem { file }
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
        find_region_addr(self.file, start, end, value)
    }
}

// 文件分块的大小 默认4mb，不要瞎鸡巴动它
const CHUNK_SIZE: usize = 4096;

// 对齐搜索，用于搜索数字，足够应对大部分情况
macro_rules! find_num_addr {
    ($buf:expr,$value:expr,$len:expr,$num:expr,$tmp:expr,$eq:tt) => {
        let vec = $buf
            .windows($len)
            .enumerate()
            .step_by($len)
            .filter_map(|(k, v)| if v $eq $value { Some(k + $num) } else { None })
            .collect::<Vec<_>>();
        $tmp.push(vec);
    };
}

// 在一个分块查找一个内存区域中值为value的地址
// file:文件句柄, start:开始区域, end:结束区域, value:目标值,
fn find_region_addr(file: &File, mut start: usize, end: usize, value: &[u8]) -> Result<Vec<usize>> {
    let mut tmp = Vec::default();
    let len = value.len();
    let mut num = 0;
    let size = end - start;

    if CHUNK_SIZE >= size {
        let mut buf = vec![0; size];
        file.read_at(&mut buf, start as u64)?;
        debug!("CHUNK_SIZE >= size");
        find_num_addr![buf,value,len,num,tmp,==];
        return Ok(tmp.into_iter().flatten().collect::<Vec<_>>());
    }

    let mut buf = vec![0; CHUNK_SIZE];

    for _ in 0..(end - start) / CHUNK_SIZE {
        file.read_at(&mut buf, start as u64)?;
        find_num_addr![buf,value,len,num,tmp,==];
        start += CHUNK_SIZE;
        num += CHUNK_SIZE;
        debug!("0..(end - start) / CHUNK_SIZE");
    }

    let size = (end - start) % CHUNK_SIZE;
    if size != 0 {
        let mut buf = vec![0; size];
        file.read_at(&mut buf, start as u64)?;
        find_num_addr![buf,value,len,num,tmp,==];
        debug!("(end - start) % CHUNK_SIZE");
    }
    Ok(tmp.into_iter().flatten().collect::<Vec<_>>())
}
