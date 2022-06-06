use std::{fs::File, io::Write, os::unix::prelude::FileExt, path::Path, thread::sleep, time::Duration};

use crate::{error::Result, scan::Scan};

pub trait MemExt {
    fn write(&self, addr: usize, payload: &[u8]) -> Result<usize>;
    fn read(&self, addr: usize, size: usize) -> Result<Vec<u8>>;
    fn dump(&self, addr: usize, size: usize, path: &str) -> Result<usize>;
    fn freeze(&self, va: usize, payload: Vec<u8>) -> Result<()>;
    fn unfreeze(&self, va: usize, payload: Vec<u8>) -> Result<()>;
}

impl MemExt for Scan {
    fn write(&self, addr: usize, payload: &[u8]) -> Result<usize> {
        self.proc.mem.write_at(payload, addr as u64)?;
        Ok(payload.len())
    }

    fn read(&self, addr: usize, size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; size];
        if let Err(e) = self.proc.mem.read_at(&mut buf, addr as u64) {
            println!("err: {}", e);
        };
        println!("0x{:x}=>ok", addr);
        // self.proc.mem.read_at(&mut buf, addr as u64)?;
        Ok(buf)
    }

    fn dump(&self, addr: usize, size: usize, path: &str) -> Result<usize> {
        let mut file = File::create(Path::new(path))?;
        let buf = self.read(addr, size)?;
        file.write_all(&buf)?;
        Ok(buf.len())
    }

    fn freeze(&self, addr: usize, payload: Vec<u8>) -> Result<()> {
        let f = self.proc.mem.try_clone()?;
        std::thread::spawn(move || loop {
            if let Err(e) = f.write_at(&payload, addr as u64) {
                println!("Error freeze addr 0x{:x} fail. {}", addr, e);
            };
            sleep(Duration::from_millis(10));
        });
        Ok(())
    }

    // TODO
    fn unfreeze(&self, _va: usize, _payload: Vec<u8>) -> Result<()> {
        Ok(())
    }
}
