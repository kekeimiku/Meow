use crate::{error::Result, maps::MapRange};

pub trait MemExt {
    fn write(&self, addr: usize, payload: &[u8]) -> Result<usize>;
    fn read(&self, addr: usize, size: usize) -> Result<Vec<u8>>;
    fn dump(&self, addr: usize, size: usize, path: &str) -> Result<usize>;
    fn freeze(&self, va: usize, payload: Vec<u8>) -> Result<()>;
    fn unfreeze(&self, va: usize, payload: Vec<u8>) -> Result<()>;
}

pub trait MapsExt {
    fn region_lv0(&mut self) -> Result<Vec<MapRange>>;
    fn region_lv1(&mut self) -> Result<Vec<MapRange>>;
}

pub trait SyscallExt {
    fn get_ip(&mut self) -> Result<u64>;
}

pub trait InjectExt {
    fn inject(&mut self, lib_path: &str) -> Result<()>;
}

pub trait ScanExt {
    fn scan(&mut self) -> Result<()>;
    fn print(&mut self) -> Result<()>;
}

#[derive(Default)]
pub struct Cache {
    pub input: Vec<u8>,
    pub maps: Vec<MapRange>,
    pub addr: Vec<Vec<usize>>,
}
