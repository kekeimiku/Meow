use std::fs::File;
use std::os::unix::prelude::FileExt;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use crate::error::Result;
use crate::scan::MemScan;

impl MemScan {
    pub fn write(&self, addr: usize, payload: &[u8]) -> Result<usize> {
        self.mem_file.write_at(payload, addr as u64)?;
        Ok(payload.len())
    }

    pub fn read_bytes(&self, addr: usize, size: usize) -> Vec<u8> {
        let mut buf = vec![0; size];
        if let Err(err) = self.mem_file.read_at(&mut buf, addr as u64) {
            eprintln!("Err: {}", err)
        };
        buf
    }

    // TODO dump 一段内存 从缓存里面拿还是内存里面拿一份最新的？
    pub fn dump(&self, _start: usize, _end: usize) -> Result<()> {
        let _f = File::create(Path::new("."))?;
        //f.write_all()
        Ok(())
    }

    // 冻结一个地址中的值
    pub fn freeze(&self, va: Vec<usize>, payload: Vec<u8>) -> Result<()> {
        let f = self.mem_file.try_clone()?;
        std::thread::spawn(move || loop {
            va.iter().for_each(|addr| {
                f.write_at(&payload, *addr as u64).unwrap();
                sleep(Duration::from_millis(20));
            })
        });

        Ok(())
    }

    pub fn unfreeze(&self) {}
}
