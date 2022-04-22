use crate::error::{Error, Result};

#[derive(Debug)]
pub struct MapRange {
    pub range_start: usize,
    pub range_end: usize,
    pub offset: usize,
    pub dev: String,
    pub flags: String,
    pub inode: usize,
    pub pathname: String,
}

impl MapRange {
    pub fn size(&self) -> usize {
        self.range_end - self.range_start
    }
    pub fn start(&self) -> usize {
        self.range_start
    }
    pub fn end(&self) -> usize {
        self.range_end
    }
    pub fn pathname(&self) -> &String {
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

pub fn parse_proc_maps(contents: &str) -> Result<Vec<MapRange>> {
    let mut vec: Vec<MapRange> = Vec::new();
    for line in contents.split('\n') {
        let mut split = line.split_whitespace();
        let range = split.next();
        if range.is_none() {
            break;
        }
        let mut range_split = range.ok_or(Error::SplitNextError)?.split('-');
        let range_start = range_split.next().ok_or(Error::SplitNextError)?;
        let range_end = range_split.next().ok_or(Error::SplitNextError)?;
        let flags = split.next().ok_or(Error::SplitNextError)?;
        let offset = split.next().ok_or(Error::SplitNextError)?;
        let dev = split.next().ok_or(Error::SplitNextError)?;
        let inode = split.next().ok_or(Error::SplitNextError)?;

        vec.push(MapRange {
            range_start: usize::from_str_radix(range_start, 16)?,
            range_end: usize::from_str_radix(range_end, 16)?,
            offset: usize::from_str_radix(offset, 16)?,
            dev: dev.to_string(),
            flags: flags.to_string(),
            inode: inode.parse::<usize>()?,
            pathname: split.collect::<Vec<&str>>().join(" "),
        });
    }
    Ok(vec)
}
