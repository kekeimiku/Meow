use crate::{Error, Result};
use core::ffi::c_void;

#[repr(C)]
pub struct Iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

extern "C" {
    pub fn mprotect(addr: *mut c_void, len: usize, prot: i32) -> i32;
    pub fn mlock(addr: *const c_void, len: usize) -> i32;
    pub fn munlock(addr: *const c_void, len: usize) -> i32;
    pub fn dlsym(handle: *mut c_void, symbol: *const i8) -> *mut c_void;
    pub fn dlopen(filename: *const i8, flag: i8) -> *mut c_void;
    pub fn dlclose(handle: *mut c_void) -> i32;
}

pub fn lock() {}

pub fn unlock() {}

pub fn mpt(addr: usize, len: usize, prot: i32) -> Result<()> {
    match unsafe { mprotect(addr as *mut _, len, prot) } {
        0 => Ok(()),
        _ => Err(Error::SysCall(std::io::Error::last_os_error())),
    }
}

extern "C" {
    fn process_vm_readv(
        pid: i32,
        local_iov: *const Iovec,
        liovcnt: u64,
        remote_iov: *const Iovec,
        riovcnt: u64,
        flags: u64,
    ) -> isize;
    fn process_vm_writev(
        pid: i32,
        local_iov: *const Iovec,
        liovcnt: u64,
        remote_iov: *const Iovec,
        riovcnt: u64,
        flags: u64,
    ) -> isize;
}

pub fn pvr(pid: i32, addr: usize, buf: &mut [u8]) -> Result<()> {
    let local_iov = Iovec {
        iov_base: buf.as_mut_ptr() as *mut c_void,
        iov_len: buf.len(),
    };
    let remote_iov = Iovec {
        iov_base: addr as *mut c_void,
        iov_len: buf.len(),
    };
    match unsafe { process_vm_readv(pid, &local_iov, 1, &remote_iov, 1, 0) } {
        0 => Ok(()),
        _ => Err(Error::SysCall(std::io::Error::last_os_error())),
    }
}

pub fn pvw(pid: i32, addr: usize, buf: &[u8]) -> Result<()> {
    let local_iov = Iovec {
        iov_base: buf.as_ptr() as *mut c_void,
        iov_len: buf.len(),
    };
    let remote_iov = Iovec {
        iov_base: addr as *mut c_void,
        iov_len: buf.len(),
    };
    match unsafe { process_vm_writev(pid, &local_iov, 1, &remote_iov, 1, 0) } {
        0 => Ok(()),
        _ => Err(Error::SysCall(std::io::Error::last_os_error())),
    }
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct WinSize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}

extern "C" {
    pub fn ioctl(fd: i32, request: u64, ...) -> i32;
}

pub fn get_termsize() -> WinSize {
    let us = WinSize::default();
    unsafe { ioctl(1, 0x5413, &us) };
    println!("{:?}", us);
    us
}
