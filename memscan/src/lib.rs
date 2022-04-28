use indicatif::ProgressBar;
use std::os::unix::fs::FileExt;
use std::{fs::File, io::Read, path::Path};
use std::time::{SystemTime, UNIX_EPOCH};

use memchr::memmem::find_iter;

pub mod error;
pub mod libc;
pub mod maps;
// pub mod promt;
// pub mod pbar;
pub mod cmd;

use crate::libc::pvr;
use crate::{
    error::{Error, Result},
    libc::pvw as process_vm_writev,
    maps::{parse_proc_maps, MapRange},
};
// use crate::pbar::ProgressBar;

#[derive(Debug)]
pub struct MemScan {
    pub pid: i32,                  //pid
    pub maps_cache: Vec<MapRange>, //maps缓存
    pub addr_cache: Vec<usize>,    //读取到的值缓存在这里
    pub input: Vec<u8>,            //输入的值，用来读取那个
    pub lock_cache: Vec<u8>,       //冻结的地址列表
    pub save_cache: Vec<u8>,       //主动保存的地址列表
    pub mem_file: File,
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
            mem_file: File::open(&Path::new(&format!("/proc/{}/mem", pid))).unwrap(),
        }
    }

    // 读取maps中列出的所有可读的行地址 除了 [vvar]
    fn readmaps_all(&mut self) -> Result<()> {
        let mut file = File::open(&Path::new(&format!("/proc/{}/maps", self.pid)))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        self.maps_cache = parse_proc_maps(&contents)?
            .into_iter()
            .filter(|m| {
                m.end() - m.start() > 0 && (m.pathname() == "[heap]" || m.pathname() == "[stack]")
                    || (m.pathname().is_empty() && m.is_read() && m.is_write())
                    || (m.pathname().is_empty() && m.is_read() && m.is_write() && m.is_exec())
            })
            .collect::<Vec<MapRange>>();

        // dbg!(&self.maps_cache.len());
        Ok(())
    }

    // 写入
    #[inline(always)]
    pub fn write_bytes(&self, addr: usize, payload: &[u8]) -> Result<usize> {
        // TODO 如果失败了用其它方式写入
        process_vm_writev(self.pid, addr, payload)?;
        Ok(payload.len())
    }

    // 读取内存
    #[inline(always)]
    pub fn read_bytes(&self, addr: usize, size: usize) -> Vec<u8> {
        // TODO 如果失败了用其它方式读取
        let mut buf = vec![0; size];
        // self.mem_file.read_at(&mut buf, addr as u64)?;
        match pvr(self.pid, addr, &mut buf) {
            Ok(_) => {}
            Err(_) => {
                match self.mem_file.read_at(&mut buf, addr as u64) {
                    Ok(_) => {}
                    Err(_) => {
                        println!("err read_at")
                    }
                };
            }
        };

        buf
    }

    // 搜索内存
    // TODO 提供可选的类型，有时已经知道内存类型，不需要搜索全部。
    pub fn search_all(&mut self, v: &[u8]) -> Result<()> {
        self.readmaps_all()?;
        let pb = ProgressBar::new(self.maps_cache.len() as u64);
        self.addr_cache = self
            .maps_cache
            .iter()
            .flat_map(|m| -> Vec<usize> {
                pb.inc(1);
                find_iter(&self.read_bytes(m.start(), m.end() - m.start()), v)
                    .map(|u| u + m.start()).collect()
            })
            .collect();
        Ok(())
    }

    // 查找发生变化的值
    pub fn change_mem(&mut self) -> Result<()> {
        println!(
            "长度长度长度长度长度长度长度长度长度长度{}",
            self.addr_cache.len()
        );
        println!("开始查找变化");

        let mut tmp = self.addr_cache.clone();
        self.addr_cache.clear();
        self.addr_cache = tmp
            .into_iter()
            .filter(|addr| self.read_bytes(*addr, self.input.len()) < self.input)
            .collect();
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

    // 获取指针
    pub fn get_ptr(&self) {}

    // 批量写入
    pub fn write_all(&self) {}

    pub fn less_mem(&self, _v: &[u8]) -> Result<()> {
        Ok(())
    }

    pub fn more_mem(&self) {}

    // 清空所有缓存，重新开始
    pub fn reset(&mut self) {
        self.maps_cache.clear();
        self.addr_cache.clear();
        self.lock_cache.clear();
        self.save_cache.clear();
        self.maps_cache.shrink_to_fit();
        self.addr_cache.shrink_to_fit();
        self.lock_cache.shrink_to_fit();
        self.save_cache.shrink_to_fit();
    }

    // 清空缓存 刷新结果
    pub fn update_at(&mut self) {}

    // 打印地址列表 太多了 先打印个长度
    // TODO 分页展示每个地址的值，用于直接观察变化，每页显示10个，loop读取20个值，翻到第二页的时候开始读取第20-30个，以此类推
    pub fn addr_list(&mut self, num: usize) {
        println!();

        if self.addr_cache.len() > num {
            self.addr_cache[0..num].iter().for_each(|a| {
                println!("0x{:x}", a);
            });
            println!(".......剩余 {} 条未显示", self.addr_cache.len() - num);
        }

        if self.addr_cache.len() < num {
            self.addr_cache.iter().for_each(|a| {
                println!("0x{:x}", a);
            });
        }
    }

    // 打印maps列表 规则同上
    pub fn map_list(&self) {}

    // TODO 设置权限
    pub fn reset_perm(&self, _addr: usize, _len: usize, _prot: i32) -> Result<()> {
        Ok(())
    }
}
