use crate::error::Result;

#[cfg(target_os = "windows")]
use crate::windows::MapRange;

#[cfg(target_os = "linux")]
use crate::linux::MapRange;

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

pub trait Region {
    fn size(&self) -> usize;
    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn pathname(&self) -> &String;
    fn is_read(&self) -> bool;
    fn is_write(&self) -> bool;
    fn is_exec(&self) -> bool;
}

#[derive(Default)]
pub struct Cache {
    pub input: Vec<u8>,
    pub maps: Vec<MapRange>,
    pub addr: Vec<Vec<usize>>,
}

#[macro_export]
macro_rules! schedule {
    ($a:expr,$b:expr,$c:expr,$d:expr) => {
        $a += 1;
        println!("[{}/{}] scan 0x{:x}-0x{:x} ...", $a, $b, $c, $d);
    };
}