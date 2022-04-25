use std::{fs::File, io::Read, path::Path};

use memchr::memmem::find_iter;

pub mod error;
pub mod libc;
pub mod maps;
pub mod promt;

use crate::{
    error::{Error, Result},
    libc::{mpt as mprotect, pvr as process_vm_readv, pvw as process_vm_writev},
    maps::{parse_proc_maps, MapRange},
};

#[derive(Debug)]
pub struct MemScan {
    pub addr_cache: Vec<usize>, //读取到的值缓存在这里
    pub pid: i32,               //pid
    pub v: Vec<u8>,             //输入的值，用来读取那个
}

impl MemScan {
    pub fn new(pid: i32) -> Self {
        Self {
            addr_cache: vec![],
            pid,
            v: vec![],
        }
    }

    // 读取maps中所有可读的行
    fn readmaps_all(&self) -> Result<Vec<MapRange>> {
        dbg!(self.pid);
        let mut file = File::open(&Path::new(&format!("/proc/{}/maps", self.pid)))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(parse_proc_maps(&contents)?
            .into_iter()
            .filter(|m| m.is_read())
            .collect::<Vec<MapRange>>())
    }

    // 读取内存
    pub fn read_bytes(&self, addr: usize, size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; size];
        process_vm_readv(self.pid, addr, &mut buf)?;
        Ok(buf)
    }

    // 写入
    pub fn write_bytes(&self, addr: usize, payload: &[u8]) -> Result<usize> {
        process_vm_writev(self.pid, addr, payload)?;
        Ok(payload.len())
    }

    // 搜索全部带有可读权限的内存
    pub fn search_all(&mut self, v: &[u8]) -> Result<()> {
        for f in self.readmaps_all()?.iter() {
            let vl = find_iter(&self.read_bytes(f.start(), f.end() - f.start())?, v)
                .map(|m| m + f.start())
                .collect::<Vec<usize>>();
            if !vl.is_empty() {
                for i in vl {
                    self.addr_cache.push(i)
                }
            }
        }
        Ok(())
    }

    pub fn less_mem(&self, _v: &[u8]) -> Result<()> {
        Ok(())
    }

    pub fn more_mem(&self) {}

    // 发生变化
    pub fn change_mem(&mut self) -> Result<()> {
        Ok(())
    }

    // 打印地址列表
    pub fn list(&self) {
        dbg!(&self.addr_cache);
    }

    // TODO 设置权限
    pub fn reset_perm(&self, addr: usize, len: usize, prot: i32) -> Result<()> {
        mprotect(addr, len, prot)
    }
}

pub fn do_vecs_match(a: Vec<u8>, b: Vec<u8>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}
