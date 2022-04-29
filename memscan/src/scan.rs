use std::fs::File;
use std::path::Path;

use memchr::memmem::find_iter;

use crate::error::Result;
use crate::libc::memcmp;
use crate::maps::MapRange;

#[derive(Debug)]
pub struct MemScan {
    pub pid: i32,                  //pid
    pub maps_cache: Vec<MapRange>, //maps缓存
    pub addr_cache: Vec<usize>,    //读取到的值缓存在这里
    pub input: Vec<u8>,            //输入的值，用来读取那个
    pub lock_cache: Vec<u8>,       //冻结的地址列表
    pub save_cache: Vec<u8>,       //主动保存的地址列表
    pub mem_file: File,            //内存文件
}

impl MemScan {
    pub fn new(pid: i32) -> Result<Self> {
        Ok(Self {
            pid,
            maps_cache: Vec::default(),
            addr_cache: Vec::default(),
            input: Vec::default(),
            lock_cache: Vec::default(),
            save_cache: Vec::default(),
            mem_file: File::open(&Path::new(&format!("/proc/{}/mem", pid)))?,
        })
    }

    pub fn search_all(&mut self, v: &[u8]) -> Result<()> {
        self.readmaps_lv1()?;
        self.addr_cache = self
            .maps_cache
            .iter()
            .flat_map(|m| -> Vec<_> {
                find_iter(&self.read_bytes(m.start(), m.end() - m.start()), v)
                    .map(|u| u + m.start())
                    .collect()
            })
            .collect();
        Ok(())
    }

    // 查找发生变化的值
    pub fn change_mem(&mut self) -> Result<()> {
        let tmp = self.addr_cache.clone();
        self.addr_cache.clear();
        // self.addr_cache = tmp
        //     .into_iter()
        //     .filter(|addr| &self.read_bytes(*addr, self.input.len()) != &self.input)
        //     .collect();

        for addr in tmp {
            if !memcmp(&self.read_bytes(addr, self.input.len()), &self.input) {
                self.addr_cache.push(addr)
            }
        }

        Ok(())
    }

    pub fn lock_meme(&self) {}

    // 直接搜索全部内存，不论数值
    pub fn unsafe_all(&self) {}

    // 打印冻结列表
    pub fn lock_list(&self) {}

    // 获取指针
    pub fn get_ptr(&self) {}

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
}
