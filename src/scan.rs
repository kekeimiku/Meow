use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

use memchr::memmem::find_iter;

use crate::{
    error::Result,
    maps::{MapRange, MapsExt},
    mem::MemExt,
    region::{CandidateLocations, Region},
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
    pub region: Vec<Region>,
    pub maps: Vec<MapRange>,
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
        self.cache.maps = self.region_lv1()?;

        for m in self.cache.maps.iter() {
            let mut location = CandidateLocations::Discrete {
                // TODO 减少搜索过程中的内存占用
                locations: find_iter(&self.read(m.start(), m.end() - m.start()).unwrap_or_default(), value).collect(),
            };

            location.try_compact(value.len());

            self.cache.region.push(Region {
                info: m.clone(),
                locations: location,
                value: crate::region::Value::Exact(value.to_vec()),
            })
        }

        Ok(())
    }

    pub fn rescan(&mut self, _value: &[u8]) -> Result<()> {
        let mut reg = self.cache.region.iter();
        for _ in 0..self.cache.region.len() {
            let rn = reg.next().unwrap();
            let mut offset = rn.locations.iter();
            let map = &rn.info;
            let _mem = self.read(map.start(), map.end() - map.start()).unwrap_or_default();
            let _i = offset.next().unwrap_or_default();

            // if &mem[i..i + value.len()] == value {
            //     println!("0x{:x}", i + map.start());
            // }
        }

        Ok(())
    }
}
