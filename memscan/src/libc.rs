use crate::Error;
use crate::Result;
use core::ffi::c_void;
use std::io::{Read, Seek};

#[repr(C)]
pub struct Iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

// PROT_READ：可写，值为 1
// PROT_WRITE：可读， 值为 2
// PROT_EXEC：可执行，值为 4
// PROT_NONE：不允许访问，值为 0
// 需要注意的是，指定的内存区间必须包含整个内存页（4K），起始地址 start 必须是一个内存页的起始地址，并且区间长度 len 必须是页大小的整数倍。
// 如果执行成功，函数返回 0；如果执行失败，函数返回 -1，并且通过 errno 变量表示具体原因。错误的原因主要有以下几个：
// EACCES：该内存不能设置为相应权限。这是可能发生的，比如 mmap(2) 映射一个文件为只读的，接着使用 mprotect() 修改为 PROT_WRITE。
// EINVAL：start 不是一个有效指针，指向的不是某个内存页的开头。
// ENOMEM：内核内部的结构体无法分配。
// ENOMEM：进程的地址空间在区间 [start, start+len] 范围内是无效，或者有一个或多个内存页没有映射。
// 当一个进程的内存访问行为违背了内存的保护属性，内核将发出 SIGSEGV（Segmentation fault，段错误）信号，并且终止该进程。
// println!("{}", 1 | 2 | 4);

extern "C" {
    pub fn mprotect(addr: *mut c_void, len: usize, prot: i32) -> i32;
    pub fn dlsym(handle: *mut c_void, symbol: *const i8) -> *mut c_void;
    pub fn dlopen(filename: *const i8, flag: i8) -> *mut c_void;
    pub fn dlclose(handle: *mut c_void) -> i32;
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
        Err(Error::ReadMemError)
    } else {
        Ok(())
    }
}
