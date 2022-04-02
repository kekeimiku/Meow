#![no_std]

extern crate alloc;
use alloc::{vec::Vec, string::String};

#[derive(Debug)]
pub struct MapRange<'a> {
    pub range_start: usize,
    pub range_end: usize,
    pub offset: usize,
    pub dev: &'a str,
    pub flags: &'a str,
    pub inode: usize,
    pub pathname: String,
}

impl MapRange<'_> {
    pub fn size(&self) -> usize {
        self.range_end - self.range_start
    }
    pub fn start(&self) -> usize {
        self.range_start
    }
    pub fn end(&self) -> usize {
        self.range_end
    }
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

pub fn parse_proc_maps(maps_file: &str) -> Vec<MapRange> {
    let mut vec: Vec<MapRange> = Vec::new();
    for line in maps_file.split('\n') {
        let mut split = line.split_whitespace();
        let range = split.next();
        if range.is_none() {
            break;
        }
        let mut range_split = range.unwrap().split('-');
        let range_start = range_split.next().unwrap();
        let range_end = range_split.next().unwrap();
        let flags = split.next().unwrap();
        let offset = split.next().unwrap();
        let dev = split.next().unwrap();
        let inode = split.next().unwrap();
        let pathname = split.collect::<Vec<&str>>().join(" ");

        vec.push(MapRange {
            range_start: usize::from_str_radix(range_start, 16).unwrap(),
            range_end: usize::from_str_radix(range_end, 16).unwrap(),
            offset: usize::from_str_radix(offset, 16).unwrap(),
            dev,
            flags,
            inode: inode.parse::<usize>().unwrap(),
            pathname,
        });
    }
    vec
}
