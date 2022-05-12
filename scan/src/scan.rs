use std::{
    fs::{File, OpenOptions},
    path::Path,
};

use crate::{error::Result, maps::MapRange};
use memchr::memmem::find_iter;

#[derive(Debug)]
pub struct MemScan {
    pub pid: i32,                    //pid
    pub input: Vec<u8>,              //输入缓存
    pub maps_cache: Vec<MapRange>,   //maps缓存
    pub mem_cache: Vec<Vec<u8>>,     //mem缓存
    pub mem_file: File,              //文件
    pub addr_cache: Vec<Vec<usize>>, //地址缓存
}

impl MemScan {
    pub fn new(pid: i32) -> Result<Self> {
        Ok(Self {
            pid,
            input: Vec::default(),
            maps_cache: Vec::default(),
            mem_cache: Vec::default(),
            addr_cache: Vec::default(),
            mem_file: OpenOptions::new()
                .read(true)
                .write(true)
                .open(&Path::new(&format!("/proc/{}/mem", pid)))?,
        })
    }

    pub fn first_scan(&mut self) -> Result<()> {
        self.readmaps_lv1()?;

        if self.addr_cache.is_empty() {
            self.addr_cache = self
                .maps_cache
                .iter()
                .map(|m| {
                    find_iter(
                        &self.read_bytes(m.start(), m.end() - m.start()),
                        &self.input,
                    )
                    .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
        } else {
            let v: [u8; 4] = self.input[0..4].try_into().unwrap();

            for (m, k1) in self.maps_cache.iter().zip(0..self.addr_cache.len()) {
                let mem = self.read_bytes(m.start(), m.end() - m.start());
                for k2 in (0..self.addr_cache[k1].len()).rev() {
                    if mem[self.addr_cache[k1][k2]..self.addr_cache[k1][k2] + v.len()] != v {
                        self.addr_cache[k1].swap_remove(k2);
                        self.addr_cache[k1].shrink_to_fit();
                    }
                }
            }
        }

        assert_eq!(self.addr_cache.len(), self.maps_cache.len());
        Ok(())
    }

    // 刷新缓存
    fn refresh_mem_cache(&mut self) {
        self.mem_cache = self
            .maps_cache
            .iter()
            .map(|m| self.read_bytes(m.start(), m.end() - m.start()))
            .collect();
    }

    // 相比输入变小的
    pub fn input_less(&mut self) {
        // let tmp = self.addr_cache.clone();
        // self.addr_cache.clear();
        // for (m, v) in self.maps_cache.iter().zip(tmp.iter()) {
        //     let mem = self.read_bytes(m.start(), m.end() - m.start());
        //     self.addr_cache.push(
        //         v.iter()
        //             .filter(|a| mem[**a..**a + self.input.len()].to_vec() < self.input)
        //             .copied()
        //             .collect::<Vec<_>>(),
        //     );
        // }
        // self.addr_cache.shrink_to_fit();

        assert_eq!(self.addr_cache.len(), self.maps_cache.len());
    }

    // 相比自身变小的
    pub fn self_less(&mut self) {
        for (m, k1) in self.maps_cache.iter().zip(0..self.addr_cache.len()) {
            let mem = self.read_bytes(m.start(), m.end() - m.start());
            for k2 in (0..self.addr_cache[k1].len()).rev() {
                if mem[self.addr_cache[k1][k2]..self.addr_cache[k1][k2] + self.input.len()].to_vec()
                    > self.input
                {
                    self.addr_cache[k1].swap_remove(k2);
                    self.addr_cache[k1].shrink_to_fit();
                }
            }
        }
    }

    pub fn list_maps(&self) -> Result<()> {
        self.maps_cache.iter().for_each(|m| {
            println!(
                "0x{:x}-0x{:x} {}{}{} {}",
                m.start(),
                m.end(),
                m.read(),
                m.write(),
                m.exec(),
                m.pathname()
            );
        });
        Ok(())
    }

    pub fn list_abs_addr(&self) {
        // let val = self
        //     .addr_cache
        //     .iter()
        //     .enumerate()
        //     .map(|(k, v)| {
        //         v.iter()
        //             .map(|a| a + self.maps_cache[k].start())
        //             .collect::<Vec<_>>()
        //     })
        //     .collect::<Vec<_>>();

        // let mut n = 0;
        // val.iter().for_each(|v| {
        //     v.iter().for_each(|f| println!("0x{:x}", f));
        //     n += v.len();
        // });

        let mut n = 0;
        self.addr_cache.iter().for_each(|f| n += f.len());

        println!("总数：{}", n);
    }

    pub fn clear(&mut self) {
        self.input.clear();
        self.maps_cache.clear();
        self.maps_cache.shrink_to_fit();
        // self.mem_cache.clear();
        // self.mem_cache.shrink_to_fit();
        self.addr_cache.clear();
        self.addr_cache.shrink_to_fit();
    }
}
