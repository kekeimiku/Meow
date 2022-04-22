use std::{fs::File, io::Read};

use crate::errors::{Error, Result};

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

pub fn get_process_maps(maps: &str) -> Vec<MapRange> {
    let mut vec: Vec<MapRange> = Vec::new();
    maps.split('\n').try_for_each(|line| {
        let mut split = line.split_whitespace();
        let range = split.next();
        if range.is_none() {
            None
        } else {
            let mut range_split = range?.split('-');
            let range_start = range_split.next()?;
            let range_end = range_split.next()?;
            let flags = split.next()?;
            let offset = split.next()?;
            let dev = split.next()?;
            let inode = split.next()?;

            vec.push(MapRange {
                range_start: usize::from_str_radix(range_start, 16).ok()?,
                range_end: usize::from_str_radix(range_end, 16).ok()?,
                offset: usize::from_str_radix(offset, 16).ok()?,
                dev: dev.to_string(),
                flags: flags.to_string(),
                inode: inode.parse::<usize>().ok()?,
                pathname: split.collect::<Vec<&str>>().join(" "),
            });

            Some(())
        }
    });
    vec
}
