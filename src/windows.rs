use windows_sys::Win32::{Foundation::HANDLE, System::Diagnostics::Debug::ReadProcessMemory};

use crate::{
    ext::{Cache, InjectExt, MapsExt, MemExt, ScanExt},
    maps::MapRange,
};

use crate::error::Result;

#[derive(Default)]
pub struct Windows {
    pub proc: WinProc,
    pub cache: Cache,
}

#[derive(Default)]
pub struct WinProc {
    pub pid: i32,
    pub hprocess: HANDLE,
}

impl MemExt for Windows {
    fn write(&self, _addr: usize, _payload: &[u8]) -> Result<usize> {
        todo!()
    }

    fn read(&self, addr: usize, size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; size];
        let ok = unsafe {
            ReadProcessMemory(
                self.proc.hprocess,
                addr as *const _,
                buf.as_mut_ptr() as *mut _,
                size,
                std::ptr::null_mut(),
            )
        };
        println!("ReadProcessMemory: {}", ok);
        Ok(buf)
    }

    fn dump(&self, _addr: usize, _size: usize, _path: &str) -> Result<usize> {
        todo!()
    }

    fn freeze(&self, _va: usize, _payload: Vec<u8>) -> Result<()> {
        todo!()
    }

    fn unfreeze(&self, _va: usize, _payload: Vec<u8>) -> Result<()> {
        todo!()
    }
}

impl MapsExt for Windows {
    fn region_lv0(&mut self) -> Result<Vec<MapRange>> {
        todo!()
    }

    fn region_lv1(&mut self) -> Result<Vec<MapRange>> {
        todo!()
    }
}

impl ScanExt for Windows {
    fn scan(&mut self) -> Result<()> {
        todo!()
    }

    fn print(&mut self) -> Result<()> {
        todo!()
    }
}

impl InjectExt for Windows {
    fn inject(&mut self, _lib_path: &str) -> Result<()> {
        todo!()
    }
}

impl Windows {
    pub fn new() -> Self {
        Self {
            proc: WinProc::default(),
            cache: Cache::default(),
        }
    }

    pub fn input(&mut self, v: &[u8]) {
        self.cache.input = v.to_vec()
    }
}
