// 这里的代码在linux上写的，还没有测试

use std::{mem, ptr::null_mut};

use utils::warn;
use windows_sys::Win32::{
    Foundation::{GetLastError, HANDLE},
    System::{
        Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory},
        Memory::{
            VirtualQueryEx, MEMORY_BASIC_INFORMATION, PAGE_EXECUTE_READWRITE, PAGE_EXECUTE_WRITECOPY,
            PAGE_READWRITE, PAGE_WRITECOPY,
        },
        Threading::{OpenProcess, PROCESS_ALL_ACCESS},
    },
};

use crate::{
    error::{Error, Result},
    mem::MemExt,
    region::InfoExt,
};

pub struct Mem {
    pub handle: HANDLE,
}

impl Mem {
    pub fn new(handle: HANDLE) -> Mem {
        Self { handle }
    }
}

impl MemExt for Mem {
    fn read(&self, addr: usize, size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; size];
        let code = unsafe {
            ReadProcessMemory(self.handle, addr as _, buf.as_mut_ptr() as *mut _, size, null_mut())
        };
        if code == 0 {
            let error = unsafe { GetLastError() };
            warn!("err: {}", error);
            // todo 处理这个错误
            // return Err(Error::GetLastError(error));
        }

        Ok(buf)
    }

    fn write(&self, addr: usize, payload: &[u8]) -> Result<usize> {
        let code = unsafe {
            WriteProcessMemory(
                self.handle,
                addr as *mut _,
                payload.as_ptr() as *const _,
                payload.len(),
                null_mut(),
            )
        };

        if code == 0 {
            let error = unsafe { GetLastError() };
            warn!("err: {}", error);
            // todo 处理这个错误
            // return Err(Error::GetLastError(error));
        }

        Ok(payload.len())
    }
}

pub type Region = MEMORY_BASIC_INFORMATION;

impl InfoExt for Region {
    fn size(&self) -> usize {
        self.RegionSize
    }

    fn start(&self) -> usize {
        self.BaseAddress as usize
    }

    fn end(&self) -> usize {
        self.BaseAddress as usize + self.RegionSize
    }

    // todo
    fn is_read(&self) -> bool {
        self.Protect == PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY | PAGE_READWRITE | PAGE_WRITECOPY
    }

    // todo
    fn is_write(&self) -> bool {
        self.Protect == PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY | PAGE_READWRITE | PAGE_WRITECOPY
    }
}

// todo
pub fn get_region_range(handle: HANDLE) -> Result<Vec<Region>> {
    let mut base: usize = 0;
    let mut regions = Vec::new();
    let mut info = mem::MaybeUninit::uninit();
    let len = mem::size_of::<Region>();
    while unsafe { VirtualQueryEx(handle, base as *const _, info.as_mut_ptr(), len) } > 0 {
        let info = unsafe { info.assume_init() };
        base = info.BaseAddress as usize + info.RegionSize;
        regions.push(info);
    }

    if regions.is_empty() {
        let error = unsafe { GetLastError() };
        return Err(Error::GetLastError(error));
    }

    Ok(regions)
}

pub fn get_memory_handle(pid: u32) -> Result<Mem> {
    let handle: HANDLE = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, pid) };
    if handle <= 0 {
        let error = unsafe { GetLastError() };
        return Err(Error::GetLastError(error));
    }

    Ok(Mem::new(handle))
}
