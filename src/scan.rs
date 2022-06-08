use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

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
    pub input: Vec<u8>,
    pub region: Vec<RegionData>,
    pub flag: u8,
}

#[derive(Default, Debug)]
pub struct RegionData {
    pub addr: VecMinValue,
    pub value: Vec<Vec<u8>>,
    pub maps: MapRange,
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
        self.cache.input = value.to_vec();
        if self.cache.flag == 0 {
            for maps in self.region_lv1().unwrap() {
                self.first_scan(maps).unwrap();
            }
            self.cache.flag = 1
        } else if self.cache.flag == 1 {
            self.filter_scan().unwrap();
        }
    }

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
                .retain(|&a| self.cache.input == &mem[a..a + self.cache.input.len()]);
            self.cache.region[k1].addr.shrink_to_fit()
        });
        Ok(())
    }

    pub fn less_scan1(&mut self) -> Result<()> {
        (0..self.cache.region.len()).for_each(|k1| {
            let mem = self
                .read(
                    self.cache.region[k1].maps.start(),
                    self.cache.region[k1].maps.end() - self.cache.region[k1].maps.start(),
                )
                .unwrap();
            let mut tmp = Vec::new();
            self.cache.region[k1].addr.retain(|&a| {
                let v = &mem[a..a + self.cache.input.len()];
                if &self.cache.input[0..self.cache.input.len()] > v {
                    tmp.push(v.to_vec());
                    true
                } else {
                    false
                }
            });
            self.cache.region[k1].value = tmp;
            self.cache.region[k1].addr.shrink_to_fit()
        });
        Ok(())
    }

    pub fn less_scan2(&mut self) -> Result<()> {
        (0..self.cache.region.len()).for_each(|k1| {
            let mem = self
                .read(
                    self.cache.region[k1].maps.start(),
                    self.cache.region[k1].maps.end() - self.cache.region[k1].maps.start(),
                )
                .unwrap();

            (0..self.cache.region[k1].addr.len()).rev().for_each(|k2| {
                // println!("{:?}", self.cache.region[k1].addr);
                // println!("{:?}", self.cache.region[k1].value);

                let value = &self.cache.region[k1].value[k2];

                // println!("储存的值: {:?}", value);

                let vv = &mem
                    [self.cache.region[k1].addr.val(k2)..self.cache.region[k1].addr.val(k2) + value.len()];

                // println!("读取的值: {:?}", vv);

                if vv >= value {
                    self.cache.region[k1].addr.remove(k2);
                    // println!("{:?}", self.cache.region[k1].addr);
                    self.cache.region[k1].value.remove(k2);
                    // println!("{:?}", self.cache.region[k1].value);
                } else {
                    self.cache.region[k1].value[k2] = vv.to_vec();
                }
            });

            self.cache.region[k1].addr.shrink_to_fit()
        });
        Ok(())
    }

    pub fn first_scan(&mut self, maps: MapRange) -> Result<()> {
        let n = self.cache.input.len();
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
                .filter_map(|(k, v)| if v == self.cache.input { Some(k) } else { None })
                .collect::<Vec<_>>(),
        };
        vec.compact();

        self.cache.region.push(RegionData {
            addr: vec,
            value: Vec::default(),
            maps,
        });

        Ok(())
    }

    pub fn print(&self) {
        let mut num = 0;
        (0..self.cache.region.len()).for_each(|k| {
            num += self.cache.region[k].addr.len();
        });

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
                }
            });
        }

        println!("num {}", num);
    }
}
