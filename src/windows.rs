use std::{mem, ptr::null_mut};

use windows::Win32::{
    Foundation::{GetLastError, HANDLE},
    System::{
        Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory},
        Memory::{VirtualQueryEx, MEMORY_BASIC_INFORMATION},
        Threading::{OpenProcess, PROCESS_ALL_ACCESS},
    },
};

use crate::{
    error::Error,
    ext::{Cache, InjectExt, MapsExt, MemExt, Region, ScanExt},
};

use crate::error::Result;

#[derive(Default)]
pub struct Windows {
    pub proc: WinProc,
    pub cache: Cache,
}

#[derive(Default)]
pub struct WinProc {
    pub pid: u32,
    pub hprocess: HANDLE,
}

pub type MapRange = MEMORY_BASIC_INFORMATION;

impl Region for MapRange {
    fn start(&self) -> usize {
        self.BaseAddress as usize
    }

    fn end(&self) -> usize {
        self.BaseAddress as usize + self.RegionSize
    }

    fn size(&self) -> usize {
        todo!()
    }

    fn pathname(&self) -> &String {
        todo!()
    }

    fn is_read(&self) -> bool {
        todo!()
    }

    fn is_write(&self) -> bool {
        todo!()
    }

    fn is_exec(&self) -> bool {
        todo!()
    }
}

pub fn get_map_range(handle: HANDLE) -> Result<Vec<MapRange>> {
    let mut base: usize = 0;
    let mut regions = Vec::new();
    let mut info = mem::MaybeUninit::uninit();
    let len = mem::size_of::<MapRange>();
    while unsafe { VirtualQueryEx(handle, base as *const _, info.as_mut_ptr(), len) } > 0 {
        let info = unsafe { info.assume_init() };
        base = info.BaseAddress as usize + info.RegionSize;
        regions.push(info);
    }

    if regions.len() < 1 {
        let error = unsafe { GetLastError() };
        assert_ne!(regions.len(), 0, "{:?}", error);
        return Err(Error::ArgsError);
    }

    Ok(regions)
}

impl MemExt for Windows {
    fn write(&self, addr: usize, payload: &[u8]) -> Result<usize> {
        unsafe {
            WriteProcessMemory(
                self.proc.hprocess,
                addr as *mut _,
                payload.as_ptr() as *const _,
                payload.len(),
                null_mut(),
            )
        };

        Ok(1)
    }

    fn read(&self, addr: usize, size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; size];
        let ok =
            unsafe { ReadProcessMemory(self.proc.hprocess, addr as _, buf.as_mut_ptr() as *mut _, size, null_mut()) };
        println!("ReadProcessMemory: {:?}", ok);
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
        get_map_range(self.proc.hprocess)
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
    pub fn new(pid: u32) -> Result<Self> {
        Ok(Self {
            proc: WinProc {
                pid: pid,
                hprocess: unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, pid)? },
            },
            cache: Cache::default(),
        })
    }

    pub fn input(&mut self, v: &[u8]) {
        self.cache.input = v.to_vec()
    }
}
