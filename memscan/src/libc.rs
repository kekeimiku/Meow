use crate::{Error, Result};
use core::ffi::c_void;

#[repr(C)]
pub struct Iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

extern "C" {
    pub fn mprotect(addr: *mut c_void, len: usize, prot: i32) -> i32;
    pub fn dlsym(handle: *mut c_void, symbol: *const i8) -> *mut c_void;
    pub fn dlopen(filename: *const i8, flag: i8) -> *mut c_void;
    pub fn dlclose(handle: *mut c_void) -> i32;
}

pub fn mpt(addr: usize, len: usize, prot: i32) -> Result<()> {
    let ret = unsafe { mprotect(addr as *mut _, len, prot) };
    if ret == -1 {
        return Err(Error::MprotectError);
    }
    Ok(())
}

extern "C" {
    pub fn process_vm_readv(
        pid: i32,
        local_iov: *const Iovec,
        liovcnt: u64,
        remote_iov: *const Iovec,
        riovcnt: u64,
        flags: u64,
    ) -> isize;
    pub fn process_vm_writev(
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
    let result = unsafe { process_vm_readv(pid, &local_iov, 1, &remote_iov, 1, 0) };
    if result == -1 {
        // TODO 如果失败了换其它方式
        Err(Error::ReadMemError)
    } else {
        Ok(())
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
    let result = unsafe { process_vm_writev(pid, &local_iov, 1, &remote_iov, 1, 0) };
    if result == -1 {
        // TODO 如果失败了换其它方式
        Err(Error::WriteMemError)
    } else {
        Ok(())
    }
}

// let mut file = OpenOptions::new()
//     .read(true)
//     .write(true)
//     .open(&Path::new(&format!("/proc/{}/mem", self.pid)))?;
// file.seek(SeekFrom::Start(addr as u64))?;
// file.write_all(payload)?;

// let mut file = File::open(&Path::new(&format!("/proc/{}/mem", self.pid)))?;
// file.seek(SeekFrom::Start(addr as u64))?;
// let mut buf = vec![0; size];
// file.read_exact(&mut buf);
