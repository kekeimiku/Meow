use std::{
    fs::{File, OpenOptions},
    os::unix::prelude::FileExt,
};

use crate::{
    error::{Error, Result},
    mem::MemExt,
    region::InfoExt,
};

pub struct Mem<T: FileExt> {
    handle: T,
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

// TODO refactor
#[derive(Debug, Default, Clone)]
pub struct Region {
    pub range_start: usize,
    pub range_end: usize,
    pub flags: String,
    pub pathname: String,
}

impl Region {
    pub fn pathname(&self) -> &str {
        &self.pathname
    }
    pub fn is_exec(&self) -> bool {
        &self.flags[2..3] == "x"
    }
    pub fn is_write(&self) -> bool {
        &self.flags[1..2] == "w"
    }
    pub fn is_read(&self) -> bool {
        &self.flags[0..1] == "r"
    }
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
}

pub fn get_region_range(contents: &str) -> Result<Vec<Region>> {
    let mut vec: Vec<Region> = Vec::new();
    let e = || Error::ParseMapsError;
    for line in contents.split('\n') {
        let mut split = line.split_whitespace();
        let range = split.next();
        if range.is_none() {
            break;
        }

        let mut range_split = range.ok_or_else(e)?.split('-');
        let range_start = range_split.next().ok_or_else(e)?;
        let range_end = range_split.next().ok_or_else(e)?;
        let flags = split.next().ok_or_else(e)?;

        vec.push(Region {
            range_start: usize::from_str_radix(range_start, 16)?,
            range_end: usize::from_str_radix(range_end, 16)?,
            flags: flags.to_string(),
            pathname: split.by_ref().skip(3).collect::<Vec<&str>>().join(" "),
        });
    }
    Ok(vec)
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
    use super::{get_region_range, InfoExt};

    #[test]
    fn test_linux_parse_proc_maps() {
        let contents: &str = r#"563ea224a000-563ea2259000 r--p 00000000 103:05 5920780 /usr/bin/fish
563ea23ea000-563ea2569000 rw-p 00000000 00:00 0 [heap]
7f9e08000000-7f9e08031000 rw-p 00000000 00:00 0"#;
        let maps = get_region_range(contents).unwrap();
        assert_eq!(maps[0].start(), 0x563ea224a000);
        assert_eq!(maps[0].end(), 0x563ea2259000);
        assert_eq!(maps[0].pathname(), "/usr/bin/fish");
        assert_eq!(maps[1].pathname(), "[heap]");
        assert_eq!(maps[2].pathname(), "");
    }
}
