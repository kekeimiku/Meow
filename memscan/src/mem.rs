use std::os::unix::prelude::FileExt;

use crate::error::Result;
use crate::libc::pvr as process_vm_readv;
use crate::libc::pvw as process_vm_writev;
use crate::scan::MemScan;

impl MemScan {
    // 写入
    #[inline(always)]
    pub fn write_bytes(&self, addr: usize, payload: &[u8]) -> Result<usize> {
        // TODO 如果失败了用其它方式写入
        process_vm_writev(self.pid, addr, payload)?;
        Ok(payload.len())
    }

    // 读取内存
    #[inline(always)]
    pub fn read_bytes(&self, addr: usize, size: usize) -> Vec<u8> {
        let mut buf = vec![0; size];
        if let Err(_) = process_vm_readv(self.pid, addr, &mut buf) {
            if let Err(err) = self.mem_file.read_at(&mut buf, addr as u64) {
                eprintln!("读取出错 :{} , Error: {}", addr, err)
            };
        };
        buf
    }

    // dump
    pub fn dump(&self) {}

    // 冻结一段内存
    pub fn lock(&self) {}

    // 解冻
    pub fn unlock(&self) {}

    // 批量写入
    pub fn write_all(&self) {}

    // TODO 设置权限
    pub fn reset_perm(&self, _addr: usize, _len: usize, _prot: i32) -> Result<()> {
        Ok(())
    }
}
