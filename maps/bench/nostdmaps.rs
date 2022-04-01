#![no_std]
#![feature(test)]

extern crate alloc;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

extern crate test;
use test::Bencher;

static MAPS: &str = r#"55ec34929000-55ec3492a000 r--p 00000000 103:02 538486                    /home/keke/Templates/哈 哈/test
55ec3492a000-55ec3492b000 r-xp 00001000 103:02 538486                    /home/keke/Templates/哈 哈/test
55ec3492b000-55ec3492c000 r--p 00002000 103:02 538486                    /home/keke/Templates/哈 哈/test
55ec3492c000-55ec3492d000 r--p 00002000 103:02 538486                    /home/keke/Templates/哈 哈/test
55ec3492d000-55ec3492e000 rw-p 00003000 103:02 538486                    /home/keke/Templates/哈 哈/test
55ec354d9000-55ec354fa000 rw-p 00000000 00:00 0                          [heap]
7f5c416cb000-7f5c416ce000 rw-p 00000000 00:00 0 
7f5c416ce000-7f5c416fa000 r--p 00000000 103:02 10489193                  /usr/lib/libc.so.6
7f5c416fa000-7f5c41870000 r-xp 0002c000 103:02 10489193                  /usr/lib/libc.so.6
7f5c41870000-7f5c418c4000 r--p 001a2000 103:02 10489193                  /usr/lib/libc.so.6
7f5c418c4000-7f5c418c5000 ---p 001f6000 103:02 10489193                  /usr/lib/libc.so.6
7f5c418c5000-7f5c418c8000 r--p 001f6000 103:02 10489193                  /usr/lib/libc.so.6
7f5c418c8000-7f5c418cb000 rw-p 001f9000 103:02 10489193                  /usr/lib/libc.so.6
7f5c418cb000-7f5c418d8000 rw-p 00000000 00:00 0 
7f5c418fe000-7f5c41900000 rw-p 00000000 00:00 0 
7f5c41900000-7f5c41902000 r--p 00000000 103:02 10489181                  /usr/lib/ld-linux-x86-64.so.2
7f5c41902000-7f5c41929000 r-xp 00002000 103:02 10489181                  /usr/lib/ld-linux-x86-64.so.2
7f5c41929000-7f5c41934000 r--p 00029000 103:02 10489181                  /usr/lib/ld-linux-x86-64.so.2
7f5c41935000-7f5c41937000 r--p 00034000 103:02 10489181                  /usr/lib/ld-linux-x86-64.so.2
7f5c41937000-7f5c41939000 rw-p 00036000 103:02 10489181                  /usr/lib/ld-linux-x86-64.so.2
7ffe40b6b000-7ffe40b8c000 rw-p 00000000 00:00 0                          [stack]
7ffe40bf3000-7ffe40bf7000 r--p 00000000 00:00 0                          [vvar]
7ffe40bf7000-7ffe40bf9000 r-xp 00000000 00:00 0                          [vdso]
ffffffffff600000-ffffffffff601000 --xp 00000000 00:00 0                  [vsyscall]
"#;

#[bench]
fn test_std(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..50 {
            std_parse_proc_maps(&MAPS);
        }
    });
}

#[bench]
fn test_no_std(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..50 {
            no_std_parse_proc_maps(&MAPS);
        }
    });
}

pub struct MapRangeStd {
    pub range_start: usize,
    pub range_end: usize,
    pub offset: usize,
    pub dev: String,
    pub flags: String,
    pub inode: usize,
    pub pathname: String,
}

// impl MapRangeStd {
//     pub fn size(&self) -> usize {
//         self.range_end - self.range_start
//     }
//     pub fn start(&self) -> usize {
//         self.range_start
//     }
//     pub fn end(&self) -> usize {
//         self.range_end
//     }
//     pub fn pathname(&self) -> &String {
//         &self.pathname
//     }
//     pub fn is_exec(&self) -> bool {
//         &self.flags[2..3] == "x"
//     }
//     pub fn is_write(&self) -> bool {
//         &self.flags[1..2] == "w"
//     }
//     pub fn is_read(&self) -> bool {
//         &self.flags[0..1] == "r"
//     }
// }

pub fn std_parse_proc_maps(contents: &str) -> Vec<MapRangeStd> {
    let mut vec: Vec<MapRangeStd> = Vec::new();
    for line in contents.split('\n') {
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

        vec.push(MapRangeStd {
            range_start: usize::from_str_radix(range_start, 16).unwrap(),
            range_end: usize::from_str_radix(range_end, 16).unwrap(),
            offset: usize::from_str_radix(offset, 16).unwrap(),
            dev: dev.to_string(),
            flags: flags.to_string(),
            inode: inode.parse::<usize>().unwrap(),
            pathname: split.collect::<Vec<&str>>().join(" "),
        });
    }
    vec
}

// =======================================================================================

pub struct MapRange<'a> {
    pub range_start: usize,
    pub range_end: usize,
    pub offset: usize,
    pub dev: &'a str,
    pub flags: &'a str,
    pub inode: usize,
    pub pathname: &'a str,
}

// impl MapRange<'_> {
//     pub fn size(&self) -> usize {
//         self.range_end - self.range_start
//     }
//     pub fn start(&self) -> usize {
//         self.range_start
//     }
//     pub fn end(&self) -> usize {
//         self.range_end
//     }
//     pub fn pathname(&self) -> &str {
//         &self.pathname
//     }
//     pub fn is_exec(&self) -> bool {
//         &self.flags[2..3] == "x"
//     }
//     pub fn is_write(&self) -> bool {
//         &self.flags[1..2] == "w"
//     }
//     pub fn is_read(&self) -> bool {
//         &self.flags[0..1] == "r"
//     }
// }

pub fn no_std_parse_proc_maps(maps_file: &str) -> Vec<MapRange> {
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
        let mut pathname = split.collect::<Vec<&str>>();
        if pathname.is_empty() {
            pathname.push("");
        };

        vec.push(MapRange {
            range_start: usize::from_str_radix(range_start, 16).unwrap(),
            range_end: usize::from_str_radix(range_end, 16).unwrap(),
            offset: usize::from_str_radix(offset, 16).unwrap(),
            dev,
            flags,
            inode: inode.parse::<usize>().unwrap(),
            pathname: pathname[0],
        });
    }
    vec
}
