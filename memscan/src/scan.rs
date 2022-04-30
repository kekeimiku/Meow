use std::fs::{File, OpenOptions};
use std::path::Path;

use memchr::memmem::find_iter;

use crate::error::Result;
use crate::maps::MapRange;

// TODO 等待重构.......
// TODO 模糊搜索：
// 查找内存是否发生变化应该有两种：1.相对于自身原有值的变化。2.相对于输入值变化。
// 具体变化应该分为 变大，变小，没有变化，大于x，小于x
// 回退上一次搜索
// 直接扫描图像内存

#[derive(Debug)]
pub struct MemScan {
    pub pid: i32,                    //pid
    pub maps_cache: Vec<MapRange>,   //maps缓存
    pub addr_cache: Vec<Vec<usize>>, //查找到的地址缓存
    pub input: Vec<u8>,              //输入的值
    pub lock_cache: Vec<u8>,         //冻结的地址列表
    pub save_cache: Vec<u8>,         //主动保存的地址列表
    pub mem_file: File,              //文件
    pub mem_cache: Vec<Vec<u8>>,     //内存缓存
    pub gen_cache: Vec<usize>,       //结果汇总缓存
    pub value_cache: Vec<Vec<u8>>,   //读取到的值缓存
                                     //pub his 历史
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
            mem_file: OpenOptions::new()
                .read(true)
                .write(true)
                .open(&Path::new(&format!("/proc/{}/mem", pid)))?,
            mem_cache: Vec::default(),
            gen_cache: Vec::default(),
            value_cache: Vec::default(),
        })
    }

    pub fn search_all(&mut self, value: &[u8]) -> Result<()> {
        self.readmaps_lv1()?;

        let mut num = 0;
        for line in &self.maps_cache {
            let v = self.read_bytes(line.start(), line.end() - line.start());
            self.mem_cache.push(v);
            let addr = find_iter(&self.mem_cache[num], value).collect::<Vec<_>>();
            self.addr_cache.push(addr);
            num = num + 1;
            println!(
                "[{}/{}] 0x{:x}-0x{:x} ...",
                num,
                self.maps_cache.len(),
                line.start(),
                line.end()
            );
        }

        num = 0;
        for i in &self.addr_cache {
            num = num + i.len()
        }
        println!("共找到: {} 条", num);

        // println!("{:?}", &self.addr_cache);

        Ok(())
    }

    // 过滤出比输入的值小的地址
    pub fn change_input_mem(&mut self) -> Result<()> {
        // TODO 过滤之后可以删掉addr_cache中空的value，其它缓存删除对应的key，可以有效减少内存使用

        self.mem_cache.clear();
        self.mem_cache.shrink_to_fit();
        self.mem_cache = self
            .maps_cache
            .iter()
            .map(|line| self.read_bytes(line.start(), line.end() - line.start()))
            .collect();

        let tmp = self.addr_cache.clone();
        self.addr_cache.clear();
        self.addr_cache.shrink_to_fit();
        self.addr_cache = tmp
            .into_iter()
            .enumerate()
            .map(|(k, v)| -> Vec<_> {
                v.into_iter()
                    .filter(|addr| {
                        let mc = &self.mem_cache[k][*addr..*addr + self.input.len()];
                        // if mc < &self.input {
                        //     println!("相对输入发生变化的值{:?}", mc);
                        // }
                        mc < &self.input
                    })
                    .collect()
            })
            .collect();

        self.gen_cache.clear();
        self.gen_cache.shrink_to_fit();

        self.gen_cache = self
            .addr_cache
            .iter()
            .enumerate()
            .flat_map(|(k, v)| {
                v.into_iter()
                    .map(|addr| self.maps_cache[k].start() + addr)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(())
    }

    pub fn change_self_mem(&mut self) {}

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

    // 清空所有缓存，重新开始，pid不变
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
        if self.gen_cache.len() > num {
            self.gen_cache[0..num].iter().for_each(|a| {
                println!("addr 0x{:x}", a);
            });
            println!(".......剩余 {} 条未显示", self.addr_cache.len() - num);
        }

        if self.gen_cache.len() < num {
            self.gen_cache.iter().for_each(|a| {
                println!("0x{:x}", a);
            });
        }
    }

    // 打印maps列表 规则同上
    pub fn map_list(&self) {}
}
