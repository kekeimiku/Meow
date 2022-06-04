use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

use memchr::memmem::find_iter;

use crate::{
    error::Result,
    maps::{MapRange, MapsExt},
    mem::MemExt,
    data::VecMinValue,
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
    pub region: Vec<VecMinValue>,
    pub maps: Vec<MapRange>,
    pub flag: u8,
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

    pub fn scan(&mut self, value: &[u8]) -> Result<()> {
        if self.cache.flag == 0 {
            self.cache.maps = self.region_lv1()?;
            for m in self.cache.maps.iter() {
                let mut vec = VecMinValue::Orig {
                    // TODO 减少搜索过程中的内存占用
                    vec: find_iter(&self.read(m.start(), m.end() - m.start()).unwrap_or_default(), value)
                        .collect::<Vec<_>>(),
                };
                vec.compact();
                self.cache.region.push(vec)
            }

            self.cache.flag = 1;
        } else {
            (0..self.cache.region.len()).for_each(|k1| {
                let mem = self
                    .read(
                        self.cache.maps[k1].start(),
                        self.cache.maps[k1].end() - self.cache.maps[k1].start(),
                    )
                    .unwrap_or_default();
                self.cache.region[k1].retain(|&a| &mem[a..a + value.len()] == value)
            });
        }

        Ok(())
    }

    pub fn print(&self) {
        let mut num = 0;
        (0..self.cache.region.len()).for_each(|k| {
            println!(
                "{:x?}",
                self.cache.region[k]
                    .iter()
                    .map(|x| x + self.cache.maps[k].start())
                    .collect::<Vec<_>>()
            );
            num += self.cache.region[k].len();
        });

        println!("num {}", num);
    }
}
