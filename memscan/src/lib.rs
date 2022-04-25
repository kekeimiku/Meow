#![feature(is_some_with)]

use std::io::{Seek, SeekFrom};
use std::time::{SystemTime, UNIX_EPOCH};
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
            .filter(|m| m.is_read() && m.pathname() != "[vvar]")
            .collect::<Vec<MapRange>>())
    }

    // 读取内存
    pub fn read_bytes(&self, addr: usize, size: usize) -> Result<Vec<u8>> {
        // let mut buf = vec![0; size];
        // process_vm_readv(self.pid, addr, &mut buf)?;

        let mut file = File::open(&Path::new(&format!("/proc/{}/mem", self.pid)))?;
        file.seek(SeekFrom::Start(addr as u64))?;
        let mut buf = vec![0; size];
        file.read_exact(&mut buf)?;

        Ok(buf)
    }

    // 写入
    pub fn write_bytes(&self, addr: usize, payload: &[u8]) -> Result<usize> {
        process_vm_writev(self.pid, addr, payload)?;
        Ok(payload.len())
    }

    // 搜索全部带有可读权限的内存
    pub fn search_all(&mut self, v: &[u8]) -> Result<()> {
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        self.addr_cache = self
            .readmaps_all()?
            .iter()
            .map(|m| -> Result<Vec<usize>> {
                Ok(
                    find_iter(&self.read_bytes(m.start(), m.end() - m.start())?, v)
                        .map(|u| u + m.start())
                        .collect::<Vec<usize>>(),
                )
            })
            .filter(|v| v.is_ok_and(|x| !x.is_empty()))
            .collect::<Result<Vec<Vec<_>>>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<usize>>();

        let end = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        println!("len: {}  耗时: {}", self.addr_cache.len(), end - start);
        Ok(())
    }

    pub fn less_mem(&self, _v: &[u8]) -> Result<()> {
        Ok(())
    }

    pub fn more_mem(&self) {}

    // 发生变化
    pub fn change_mem(&mut self) -> Result<()> {
        self.addr_cache = self.addr_cache.iter().filter(|addr|
           self.read_bytes(**addr,self.v.len()).unwrap() != self.v
        ).copied().collect::<Vec<usize>>();
        Ok(())
    }

    // 打印地址列表 太多了 先打印个长度
    pub fn list(&self) {
        dbg!(&self.addr_cache.len());
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
