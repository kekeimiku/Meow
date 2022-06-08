use std::{fs::File, io::Read};

use crate::{
    error::{Error, Result},
    scan::Scan,
};

// TODO 更多规则
pub trait MapsExt {
    fn region_lv0(&mut self) -> Result<Vec<MapRange>>;
    fn region_lv1(&mut self) -> Result<Vec<MapRange>>;
}

impl MapsExt for Scan {
    fn region_lv0(&mut self) -> Result<Vec<MapRange>> {
        let mut file = File::open(&self.proc.maps)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(parse_proc_maps(&contents)?.into_iter().collect())
    }

    fn region_lv1(&mut self) -> Result<Vec<MapRange>> {
        let mut file = File::open(&self.proc.maps)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(parse_proc_maps(&contents)?
            .into_iter()
            .filter(|m| m.end() - m.start() > 0 && m.is_read() && m.is_write() && m.pathname() != "[vvar]")
            .collect())
    }
}

#[derive(Debug, Clone, Default)]
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
    let e = || Error::New("failed to parse maps".into());
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
        let offset = split.next().ok_or_else(e)?;
        let dev = split.next().ok_or_else(e)?;
        let inode = split.next().ok_or_else(e)?;

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
