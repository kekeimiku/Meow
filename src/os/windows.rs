// 这里的代码在linux上写的，还没有测试

use std::{mem, ptr::null_mut};

use windows_sys::Win32::{
    Foundation::{GetLastError, HANDLE},
    System::{
        Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory},
        Memory::{VirtualQueryEx, MEMORY_BASIC_INFORMATION},
    },
};

use crate::{
    error::{Error, Result},
    mem::MemExt,
    region::RegionExt,
};

pub struct Mem {
    handle: HANDLE,
}

impl Mem {
    pub fn from(handle: HANDLE) -> Mem {
        Self { handle }
    }
}

impl MemExt for Mem {
    fn read(&self, addr: usize, size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; size];
        // TODO 错误处理
        unsafe { ReadProcessMemory(self.handle, addr as _, buf.as_mut_ptr() as *mut _, size, null_mut()) };
        Ok(buf)
    }

    fn write(&self, addr: usize, payload: &[u8]) -> Result<usize> {
        // TODO 错误处理
        unsafe {
            WriteProcessMemory(
                self.handle,
                addr as *mut _,
                payload.as_ptr() as *const _,
                payload.len(),
                null_mut(),
            )
        };

        Ok(payload.len())
    }
}

pub type Region = MEMORY_BASIC_INFORMATION;

impl RegionExt for Region {
    fn size(&self) -> usize {
        self.RegionSize
    }

    fn start(&self) -> usize {
        self.BaseAddress as usize
    }

    fn end(&self) -> usize {
        self.BaseAddress as usize + self.RegionSize
    }

    fn pathname(&self) -> &str {
        todo!()
    }

    fn is_exec(&self) -> bool {
        todo!()
    }

    fn is_write(&self) -> bool {
        todo!()
    }

    fn is_read(&self) -> bool {
        todo!()
    }
}

pub fn get_map_range(handle: HANDLE) -> Result<Vec<Region>> {
    let mut base: usize = 0;
    let mut regions = Vec::new();
    let mut info = mem::MaybeUninit::uninit();
    let len = mem::size_of::<Region>();
    while unsafe { VirtualQueryEx(handle, base as *const _, info.as_mut_ptr(), len) } > 0 {
        let info = unsafe { info.assume_init() };
        base = info.BaseAddress as usize + info.RegionSize;
        regions.push(info);
    }

    if regions.len() < 1 {
        let error = unsafe { GetLastError() };
        assert_ne!(regions.len(), 0, "{:?}", error);
        return Err(Error::New("get regions error"));
    }

    Ok(regions)
}
