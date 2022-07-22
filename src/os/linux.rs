use std::{
    fs::{File, OpenOptions},
    os::unix::prelude::FileExt,
    str::Lines,
};

use crate::{error::Result, mem::MemExt, region::InfoExt};

pub struct Mem<T: FileExt> {
    pub handle: T,
}

impl<T> MemExt for Mem<T>
where
    T: FileExt,
{
    fn read(&self, addr: usize, size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; size];
        self.handle.read_at(&mut buf, addr as u64)?;
        Ok(buf)
    }

    fn write(&self, addr: usize, payload: &[u8]) -> Result<usize> {
        self.handle.write_at(payload, addr as u64)?;
        Ok(payload.len())
    }
}

#[derive(Debug, Default, Clone)]
pub struct Region<'a> {
    pub start: usize,
    pub end: usize,
    pub flags: &'a str,
    pub pathname: &'a str,
}

impl InfoExt for Region<'_> {
    fn size(&self) -> usize {
        self.end - self.start
    }
    fn start(&self) -> usize {
        self.start
    }
    fn end(&self) -> usize {
        self.end
    }
    fn is_read(&self) -> bool {
        &self.flags[0..1] == "r"
    }
    fn is_write(&self) -> bool {
        &self.flags[1..2] == "w"
    }
    fn pathname(&self) -> &str {
        self.pathname
    }
}

pub struct RegionIter<'a> {
    lines: Lines<'a>,
}

impl<'a> RegionIter<'a> {
    pub fn new(contents: &'a str) -> Self {
        Self { lines: contents.lines() }
    }
}

impl<'a> Iterator for RegionIter<'a> {
    type Item = Region<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next()?;
        let mut split = line.splitn(6, ' ');
        let mut range_split = split.next()?.split('-');
        let start = usize::from_str_radix(range_split.next()?, 16).unwrap();
        let end = usize::from_str_radix(range_split.next()?, 16).unwrap();
        let flags = split.next()?;
        let pathname = split.nth(3).unwrap_or("").trim_start();

        Some(Region { start, end, flags, pathname })
    }
}

pub fn get_memory_handle(pid: u32) -> Result<Mem<File>> {
    Ok(Mem {
        handle: OpenOptions::new()
            .read(true)
            .write(true)
            .open(format!("/proc/{}/mem", pid))?,
    })
}

#[cfg(test)]
mod tests {
    use super::{InfoExt, RegionIter};
    #[test]
    fn test_linux_parse_proc_maps() {
        let contents: &str = r#"563ea224a000-563ea2259000 r--p 00000000 103:05 5920780 /usr/bin/fish
563ea23ea000-563ea2569000 rw-p 00000000 00:00 0 [heap]
7f9e08000000-7f9e08031000 rw-p 00000000 00:00 0 
563ea224a000-563ea2259000 r--p 00000000 103:05 5920780     /usr/b in/fish  
7f9e08000000-7f9e08031000 rw-p 00000000 00:00 0 "#;
        let maps = RegionIter::new(contents).collect::<Vec<_>>();
        assert_eq!(maps[0].start(), 0x563ea224a000);
        assert_eq!(maps[0].end(), 0x563ea2259000);
        assert_eq!(maps[0].pathname(), "/usr/bin/fish");
        assert_eq!(maps[1].pathname(), "[heap]");
        assert_eq!(maps[2].pathname(), "");
        assert_eq!(maps[3].pathname(), "/usr/b in/fish  ");
        assert_eq!(maps[4].pathname(), "");
    }
}
