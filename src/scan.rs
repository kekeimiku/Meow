use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

use log::debug;
#[cfg(feature = "memmem")]
use memchr::memmem::find_iter;

use crate::{
    data::VecMinValue,
    error::Result,
    maps::{MapRange, MapsExt},
    mem::MemExt,
};

pub struct Process {
    pub pid: u32,
    pub mem: File,
    pub maps: PathBuf,
    pub syscall: PathBuf,
}

pub struct Scan {
    pub proc: Process,
    pub cache: Cache,
}

#[derive(Default, Debug)]
pub struct Cache {
    pub input: (Vec<u8>, usize),
    pub region: Vec<RegionData>,
    pub flag1: u8,
    pub flag2: u8,
}

#[derive(Debug, Default)]
pub struct RegionData {
    pub addr: VecMinValue,
    pub value: Value,
    pub maps: MapRange,
}

#[derive(Debug, Default)]
pub struct Value {
    pub exact: Vec<Vec<u8>>,
    pub unknown: Vec<u8>,
}

impl Scan {
    pub fn new(pid: u32) -> Result<Self> {
        let mem = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&format!("/proc/{}/mem", pid))?;
        let maps = PathBuf::from(&format!("/proc/{}/maps", pid));
        let syscall = PathBuf::from(&format!("/proc/{}/syscall", pid));
        Ok(Self {
            proc: Process {
                pid,
                mem,
                maps,
                syscall,
            },
            cache: Cache::default(),
        })
    }

    pub fn run(&mut self, value: &[u8]) {
        self.cache.input = (value.to_vec(), value.len());
        if self.cache.flag1 == 0 {
            for maps in self.region_lv1().unwrap() {
                self.first_scan(maps).unwrap();
            }
            self.cache.flag1 = 1
        } else if self.cache.flag1 == 1 {
            self.filter_scan().unwrap();
        }
    }

    pub fn run1(&mut self) {
        if self.cache.input.0.is_empty() {
            println!("1111");
            return;
        }
        if self.cache.flag2 == 0 {
            self.less_scan1().unwrap();
            self.cache.flag2 = 1
        } else if self.cache.flag2 == 1 {
            self.less_scan2().unwrap();
        }
    }

    // 精准过滤搜索
    pub fn filter_scan(&mut self) -> Result<()> {
        (0..self.cache.region.len()).for_each(|k1| {
            let mem = self
                .read(
                    self.cache.region[k1].maps.start(),
                    self.cache.region[k1].maps.end() - self.cache.region[k1].maps.start(),
                )
                .unwrap();
            self.cache.region[k1]
                .addr
                .retain(|&a| self.cache.input.0 == mem[a..a + self.cache.input.1]);
            self.cache.region[k1].addr.shrink_to_fit()
        });
        Ok(())
    }

    // 第一次变小搜索
    pub fn less_scan1(&mut self) -> Result<()> {
        (0..self.cache.region.len()).for_each(|k1| {
            let mem = self
                .read(
                    self.cache.region[k1].maps.start(),
                    self.cache.region[k1].maps.end() - self.cache.region[k1].maps.start(),
                )
                .unwrap();

            // (0..self.cache.region[k1].addr.len()).rev().for_each(|k2| {
            //     let cache_value = &self.cache.input.0[0..4];

            //     debug!(
            //         "addr: 0x{:x}",
            //         self.cache.region[k1].addr.val(k2) + self.cache.region[k1].maps.start()
            //     );
            //     debug!("cache_value: {:?}", cache_value);

            //     let next_value = &mem[self.cache.region[k1].addr.val(k2)
            //         ..self.cache.region[k1].addr.val(k2) + cache_value.len()];

            //     debug!("next_value: {:?}", next_value);

            //     if i32::from_ne_bytes(next_value.try_into().unwrap())
            //         > i32::from_ne_bytes(cache_value.try_into().unwrap())
            //     {
            //         self.cache.region[k1].addr.swap_remove(k2);
            //         debug!("next_value >= cache_value, remove value and addr")
            //     } else {
            //         self.cache.region[k1].value.exact.push(next_value.to_vec());
            //     }
            // });

            let mut tmp = Vec::new();
            self.cache.region[k1].addr.retain(|&a| {
                let v = &mem[a..a + self.cache.input.1];
                let c = &self.cache.input.0[0..self.cache.input.1];

                if i32::from_ne_bytes(v.try_into().unwrap()) < i32::from_ne_bytes(c.try_into().unwrap()) {
                    tmp.push(v.to_vec());
                    true
                } else {
                    false
                }
            });

            self.cache.region[k1].value.exact = tmp;
            self.cache.region[k1].addr.shrink_to_fit()
        });
        Ok(())
    }

    // 第二次以及后续变小搜索都用这个
    pub fn less_scan2(&mut self) -> Result<()> {
        (0..self.cache.region.len()).for_each(|k1| {
            let mem = self
                .read(
                    self.cache.region[k1].maps.start(),
                    self.cache.region[k1].maps.end() - self.cache.region[k1].maps.start(),
                )
                .unwrap();

            (0..self.cache.region[k1].addr.len()).rev().for_each(|k2| {
                let cache_value = &self.cache.region[k1].value.exact[k2][0..4];

                debug!(
                    "addr: 0x{:x}",
                    self.cache.region[k1].addr.val(k2) + self.cache.region[k1].maps.start()
                );
                debug!("cache_value: {:?}", cache_value);

                let next_value = &mem[self.cache.region[k1].addr.val(k2)
                    ..self.cache.region[k1].addr.val(k2) + cache_value.len()];

                debug!("next_value: {:?}", next_value);

                if i32::from_ne_bytes(next_value.try_into().unwrap())
                    >= i32::from_ne_bytes(cache_value.try_into().unwrap())
                {
                    self.cache.region[k1].addr.swap_remove(k2);
                    self.cache.region[k1].value.exact.swap_remove(k2);
                    debug!("next_value >= cache_value, remove value and addr")
                } else {
                    self.cache.region[k1].value.exact[k2] = next_value.to_vec();
                    debug!("next_value < cache_value. no remove, update value cache")
                }
            });

            self.cache.region[k1].addr.shrink_to_fit()
        });
        Ok(())
    }

    // 第一次搜索
    pub fn first_scan(&mut self, maps: MapRange) -> Result<()> {
        let n = self.cache.input.1;
        let mut vec = VecMinValue::Orig {
            // TODO 减少搜索过程中的内存占用
            #[cfg(feature = "memmem")]
            vec: find_iter(&self.read(m.start(), m.end() - m.start()).unwrap_or_default(), value)
                .collect::<Vec<_>>(),

            #[cfg(not(feature = "memmem"))]
            vec: self
                .read(maps.start(), maps.end() - maps.start())
                .unwrap()
                .windows(n)
                .enumerate()
                .step_by(n)
                .filter_map(|(k, v)| if v == self.cache.input.0 { Some(k) } else { None })
                .collect::<Vec<_>>(),
        };
        vec.compact();

        self.cache.region.push(RegionData {
            addr: vec,
            value: Value::default(),
            maps,
        });
        Ok(())
    }

    pub fn print(&self) {
        let num = self.cache.region.iter().map(|r| r.addr.len()).sum::<usize>();

        if num < 11 {
            (0..self.cache.region.len()).for_each(|k| {
                if !self.cache.region[k].addr.is_empty() {
                    println!(
                        "{:x?}",
                        self.cache.region[k]
                            .addr
                            .iter()
                            .map(|x| x + self.cache.region[k].maps.start())
                            .collect::<Vec<_>>()
                    );
                    println!("{:?}", self.cache.region[k].value.exact);
                }
            });
        }

        println!("num {}", num);
    }

    // 第一次未知搜索，保存整个内存区域
    pub fn unknown(&mut self, maps: MapRange) -> Result<()> {
        let mem = self.read(maps.start(), maps.end() - maps.start()).unwrap();
        // println!("len {}", mem.len());
        self.cache.region.push(RegionData {
            addr: VecMinValue::default(),
            value: Value {
                exact: Vec::default(),
                unknown: mem,
            },
            maps,
        });
        Ok(())
    }

    // 第一次未知变小
    pub fn unknown_less1(&mut self) {
        (0..self.cache.region.len()).for_each(|k| {
            let maps = &self.cache.region[k].maps;
            let ormem = &self.cache.region[k].value.unknown;
            let mem = self.read(maps.start(), maps.end() - maps.start()).unwrap();
            let addr = ormem
                .windows(4)
                .enumerate()
                .step_by(4)
                .zip(mem.windows(4).step_by(4))
                .filter_map(|(m1, m2)| if m1.1 > m2 { Some(m1.0) } else { None })
                .collect::<Vec<_>>();

            let mut vec = VecMinValue::Orig { vec: addr };
            vec.compact();

            let r = RegionData {
                addr: vec,
                value: Value {
                    exact: Vec::default(),
                    unknown: mem,
                },
                maps: maps.clone(),
            };

            self.cache.region.push(r);
        });
    }
}
