#![feature(is_some_with)]

use std::io::{Seek, SeekFrom};
use std::{fs::File, io::Read, path::Path};

use indicatif::ProgressBar;
use memchr::memmem::find_iter;

pub mod error;
pub mod libc;
pub mod maps;
pub mod promt;

use crate::{
    error::{Error, Result},
    libc::pvw as process_vm_writev,
    maps::{parse_proc_maps, MapRange},
};

#[derive(Debug)]
pub struct MemScan {
    pub pid: i32,                  //pid
    pub maps_cache: Vec<MapRange>, //maps缓存
    pub addr_cache: Vec<usize>,    //读取到的值缓存在这里
    pub input: Vec<u8>,            //输入的值，用来读取那个
    pub lock_cache: Vec<u8>,       //冻结的地址列表
    pub save_cache: Vec<u8>,       //主动保存的地址列表
}

impl MemScan {
    pub fn new(pid: i32) -> Self {
        Self {
            pid,
            maps_cache: vec![],
            addr_cache: vec![],
            input: vec![],
            lock_cache: vec![],
            save_cache: vec![],
        }
    }

    // 读取maps中列出的所有可读的行地址 除了 [vvar]
    fn readmaps_all(&mut self) -> Result<()> {
        let mut file = File::open(&Path::new(&format!("/proc/{}/maps", self.pid)))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        self.maps_cache = parse_proc_maps(&contents)?
            .into_iter()
            .filter(|m| m.is_read() && m.pathname() != "[vvar]")
            .collect::<Vec<MapRange>>();

        Ok(())
    }

    // 读取内存
    pub fn read_bytes(&self, addr: usize, size: usize) -> Result<Vec<u8>> {
        // TODO 如果失败了用其它方式读取
        let mut buf = vec![0; size];
        let mut file = File::open(&Path::new(&format!("/proc/{}/mem", self.pid)))?;
        file.seek(SeekFrom::Start(addr as u64))?;
        file.read_exact(&mut buf)?;
        // process_vm_readv(self.pid, addr, &mut buf)?;
        Ok(buf)
    }

    // 写入
    pub fn write_bytes(&self, addr: usize, payload: &[u8]) -> Result<usize> {
        // TODO 如果失败了用其它方式写入
        process_vm_writev(self.pid, addr, payload)?;
        Ok(payload.len())
    }

    // 搜索全部带有可读权限的内存
    // TODO 提供可选的类型，有时已经知道内存类型，不需要搜索全部。
    pub fn search_all(&mut self, v: &[u8]) -> Result<()> {
        self.readmaps_all()?;
        let pb = ProgressBar::new(self.maps_cache.len() as u64);
        self.addr_cache = self
            .maps_cache
            .iter()
            .map(|m| -> Result<Vec<usize>> {
                pb.inc(1);
                Ok(
                    find_iter(&self.read_bytes(m.start(), m.end() - m.start())?, v)
                        .map(|u| u + m.start())
                        .collect::<Vec<usize>>(),
                )
            })
            // TODO 过滤掉空的vec应该可以在上面做，然后性能应该会好一点？
            .filter(|v| v.is_ok_and(|x| !x.is_empty()))
            .collect::<Result<Vec<Vec<_>>>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<usize>>();
        Ok(())
    }

    // 直接搜索全部内存，不论数值
    pub fn unsafe_all(&self) {}

    // dump
    pub fn dump(&self) {}

    // 冻结一段内存
    pub fn lock(&self) {}

    // 解冻
    pub fn unlock(&self) {}

    // 打印冻结列表
    pub fn lock_list(&self) {}

    // 发生变化（包括变大或者变小）
    // TODO 第一次非常慢
    pub fn change_mem(&mut self) -> Result<()> {
        let pb = ProgressBar::new(self.addr_cache.len() as u64);
        let tmp = self
            .addr_cache
            .iter()
            .filter(|addr| {
                pb.inc(1);
                self.read_bytes(**addr, self.input.len()).unwrap() != self.input
            })
            .copied()
            .collect::<Vec<usize>>();

        self.addr_cache.clear();
        self.addr_cache = tmp;
        Ok(())
    }

    pub fn less_mem(&self, _v: &[u8]) -> Result<()> {
        Ok(())
    }

    pub fn more_mem(&self) {}

    // 清空缓存
    pub fn update(&mut self) {
        self.maps_cache.clear();
        self.addr_cache.clear();
    }

    // 清空缓存 刷新结果
    pub fn update_at(&mut self) {}

    // 打印地址列表 太多了 先打印个长度
    // TODO 分页展示每个地址的值，用于直接观察变化，每页显示10个，loop读取20个值，翻到第二页的时候开始读取第20-30个，以此类推
    pub fn addr_list(&mut self, num: usize) {
            self.addr_cache.iter().for_each(|a| {
                println!("0x{:x}", a);
            });
        // if self.addr_cache.len() > num {
        //     self.addr_cache[0..num].iter().for_each(|a| {
        //         println!("0x{:x}", a);
        //     });
        //     println!(".......剩余 {} 条未显示", self.addr_cache.len() - num);
        // }
        //
        // if self.addr_cache.len() < num {
        //     self.addr_cache.iter().for_each(|a| {
        //         println!("0x{:x}", a);
        //     });
        // }
    }

    // 打印maps列表 规则同上
    pub fn map_list(&self) {}

    // TODO 设置权限
    pub fn reset_perm(&self, _addr: usize, _len: usize, _prot: i32) -> Result<()> {
        Ok(())
    }
}
