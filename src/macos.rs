use crate::ext::{InjectExt, MapsExt, MemExt, ScanExt};

pub struct Macos {}

impl MapsExt for Macos {
    fn region_lv0(&mut self) -> crate::error::Result<Vec<crate::maps::MapRange>> {
        todo!()
    }

    fn region_lv1(&mut self) -> crate::error::Result<Vec<crate::maps::MapRange>> {
        todo!()
    }
}

impl MemExt for Macos {
    fn write(&self, addr: usize, payload: &[u8]) -> crate::error::Result<usize> {
        todo!()
    }

    fn read(&self, addr: usize, size: usize) -> crate::error::Result<Vec<u8>> {
        todo!()
    }

    fn dump(&self, addr: usize, size: usize, path: &str) -> crate::error::Result<usize> {
        todo!()
    }

    fn freeze(&self, va: usize, payload: Vec<u8>) -> crate::error::Result<()> {
        todo!()
    }

    fn unfreeze(&self, va: usize, payload: Vec<u8>) -> crate::error::Result<()> {
        todo!()
    }
}

impl InjectExt for Macos {
    fn inject(&mut self, lib_path: &str) -> crate::error::Result<()> {
        todo!()
    }
}

impl ScanExt for Macos {
    fn scan(&mut self) -> crate::error::Result<()> {
        todo!()
    }

    fn print(&mut self) -> crate::error::Result<()> {
        todo!()
    }
}

impl Macos {}
