use std::{
    fs::{File, OpenOptions},
    os::unix::prelude::FileExt,
    str::Lines,
};

use crate::{error::Result, mem::MemExt, region::InfoExt};

pub struct Mem<T: FileExt> {
    pub handle: T,
}

impl<T> Mem<T>
where
    T: FileExt,
{
    pub fn new(handle: T) -> Mem<T> {
        Mem { handle }
    }
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
pub struct Region {
    pub range_start: usize,
    pub range_end: usize,
    pub flags: String,
    pub pathname: String,
}

impl InfoExt for Region {
    fn size(&self) -> usize {
        self.range_end - self.range_start
    }
    fn start(&self) -> usize {
        self.range_start
    }
    fn end(&self) -> usize {
        self.range_end
    }
    fn is_read(&self) -> bool {
        &self.flags[0..1] == "r"
    }
    fn is_write(&self) -> bool {
        &self.flags[1..2] == "w"
    }
    fn pathname(&self) -> &str {
        &self.pathname
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

impl Iterator for RegionIter<'_> {
    type Item = Region;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next();
        let mut split = line?.split_whitespace();
        let range = split.next();
        let mut range_split = range?.split('-');
        let start = range_split.next()?;
        let end = range_split.next()?;
        let flags = split.next()?;

        Some(Region {
            range_start: usize::from_str_radix(start, 16).unwrap(),
            range_end: usize::from_str_radix(end, 16).unwrap(),
            flags: flags.to_string(),
            pathname: split.by_ref().skip(3).collect::<Vec<&str>>().join(" "),
        })
    }
}

pub fn get_memory_handle(pid: u32) -> Result<Mem<File>> {
    Ok(Mem::new(
        OpenOptions::new()
            .read(true)
            .write(true)
            .open(format!("/proc/{}/mem", pid))?,
    ))
}

#[cfg(test)]
mod tests {
    use super::{InfoExt, RegionIter};
    #[test]
    fn test_linux_parse_proc_maps() {
        let contents: &str = r#"563ea224a000-563ea2259000 r--p 00000000 103:05 5920780 /usr/bin/fish
563ea23ea000-563ea2569000 rw-p 00000000 00:00 0 [heap]
7f9e08000000-7f9e08031000 rw-p 00000000 00:00 0"#;
        let maps = RegionIter::new(contents).collect::<Vec<_>>();
        assert_eq!(maps[0].start(), 0x563ea224a000);
        assert_eq!(maps[0].end(), 0x563ea2259000);
        assert_eq!(maps[0].pathname(), "/usr/bin/fish");
        assert_eq!(maps[1].pathname(), "[heap]");
        assert_eq!(maps[2].pathname(), "");
    }
}
