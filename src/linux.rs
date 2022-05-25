use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    os::unix::prelude::{FileExt, MetadataExt},
    path::{Path, PathBuf},
    thread::sleep,
    time::Duration,
};

use goblin::{
    elf::{Elf, Sym, Symtab},
    strtab::Strtab,
};

use memchr::memmem::find_iter;

use crate::{
    error::{Error, Error::ParseMapsError, Result},
    ext::{Cache, InjectExt, MapsExt, MemExt, Region, ScanExt, SyscallExt, Type},
    schedule,
};

pub struct Linux {
    proc: Process,
    cache: Cache,
}

macro_rules! typev {
    ($v:expr,$t:ty) => {
        $v.parse::<$t>()?.to_le_bytes().to_vec()
    };
}

impl Linux {
    pub fn new(pid: u32) -> Result<Self> {
        let mem = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&Path::new(&format!("/proc/{}/mem", pid)))?;
        let maps = PathBuf::from(&format!("/proc/{}/maps", pid));
        let syscall = PathBuf::from(&format!("/proc/{}/syscall", pid));

        Ok(Self {
            proc: Process {
                pid,
                mem,
                maps,
                syscall,
            },
            cache: Cache::default(),
        })
    }

    pub fn input(&mut self, t: Type, v: &str) -> Result<()> {
        self.cache.input = match t {
            Type::U8 | Type::U16 | Type::U32 | Type::U64 => v.parse::<u64>()?.to_le_bytes()[0..t as usize].to_vec(),
            Type::I8 | Type::I16 | Type::I32 | Type::I64 => {
                v.parse::<i64>()?.to_le_bytes()[0..t as usize - 16].to_vec()
            }
            Type::STR => v.as_bytes().to_vec(),
            Type::UNKNOWN => {
                let num = v.parse::<isize>()?;
                match num {
                    -128..127 => typev!(v, i8),
                    -32768..32767 => typev!(v, i16),
                    -2147483648..2147483647 => typev!(v, i32),
                    -9223372036854775808..9223372036854775807 => typev!(v, i64),
                    _ => typev!(v, i32),
                }
            }
        };

        Ok(())
    }

    pub fn clear(&mut self) {
        // self.cache.addr.clear();
        self.cache.maps.clear();
        self.cache.input.clear();
        // self.cache.addr.shrink_to_fit();
        self.cache.maps.shrink_to_fit();
        self.cache.input.shrink_to_fit();
    }
}

pub struct Process {
    pub pid: u32,
    pub mem: File,
    pub maps: PathBuf,
    pub syscall: PathBuf,
}

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

impl Region for MapRange {
    fn size(&self) -> usize {
        self.range_end - self.range_start
    }
    fn start(&self) -> usize {
        self.range_start
    }
    fn end(&self) -> usize {
        self.range_end
    }
    fn pathname(&self) -> &String {
        &self.pathname
    }
    fn is_exec(&self) -> bool {
        &self.flags[2..3] == "x"
    }
    fn is_write(&self) -> bool {
        &self.flags[1..2] == "w"
    }
    fn is_read(&self) -> bool {
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
        let mut range_split = range.ok_or(ParseMapsError)?.split('-');
        let range_start = range_split.next().ok_or(ParseMapsError)?;
        let range_end = range_split.next().ok_or(ParseMapsError)?;
        let flags = split.next().ok_or(ParseMapsError)?;
        let offset = split.next().ok_or(ParseMapsError)?;
        let dev = split.next().ok_or(ParseMapsError)?;
        let inode = split.next().ok_or(ParseMapsError)?;

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

macro_rules! find_cmp {
    ($self:ident,$op:tt,$t:ident) => {
        (0..$self.cache.$t.len()).rev().for_each(|k| {
            if $self.cache.$t[k].is_empty() {
                $self.cache.$t.swap_remove(k);
                $self.cache.maps.swap_remove(k);
            }
        });
        let mut num = 0;
        (0..$self.cache.maps.len())
            .zip(0..$self.cache.$t.len())
            .for_each(|(k1, k2)| {
                schedule!(num, $self.cache.maps.len(), $self.cache.maps[k1].start(), $self.cache.maps[k1].end());
                let mem = $self
                    .read($self.cache.maps[k1].start(), $self.cache.maps[k1].end() - $self.cache.maps[k1].start())
                    .unwrap_or_default();
                (0..$self.cache.$t[k2].len()).rev().for_each(|k3| {
                    if &mem[$self.cache.$t[k2][k3] as usize..$self.cache.$t[k2][k3] as usize + $self.cache.input.len()]
                        $op &$self.cache.input
                    {
                        $self.cache.$t[k2].swap_remove(k3);
                        $self.cache.$t[k2].shrink_to_fit();
                    }
                });
            });
    };
}

macro_rules! value_sacan {
    ($self:ident,$t:ident) => {
        let mut num = 0;
        $self.cache.$t = $self
            .cache
            .maps
            .iter()
            .map(|m| {
                schedule!(num, $self.cache.maps.len(), m.start(), m.end());
                find_iter(&$self.read(m.start(), m.end() - m.start()).unwrap_or_default(), &$self.cache.input)
                    .map(|x| x.try_into().unwrap())
                    .collect()
            })
            .collect()
    };
}

macro_rules! print_abs {
    ($self:ident,$t:ident) => {
        let mut num = 0;
        $self.cache.$t.iter().for_each(|f| num += f.len());

        if num > 10 {
            let mut n = 0;
            for (av, v) in $self.cache.$t.iter().zip($self.cache.maps.iter()) {
                if !av.is_empty() {
                    for a in av.iter() {
                        println!("0x{:x}", *a as usize + v.start());
                        n += 1;
                        if n == 10 {
                            break;
                        }
                    }
                }
            }
        } else {
            $self.cache.$t.iter().enumerate().for_each(|(k, v)| {
                v.iter()
                    .for_each(|a| println!("0x{:x}", *a as usize + $self.cache.maps[k].start()))
            });
        }
    };
}

impl ScanExt for Linux {
    fn value_scan(&mut self) -> Result<usize> {
        if self.cache.addr_u16.is_empty() && self.cache.addr_u64.is_empty() && self.cache.addr_u64.is_empty() {
            self.cache.maps = self.region_lv1()?.into_iter().collect::<Vec<MapRange>>();
            self.cache
                .maps
                .iter()
                .for_each(|f| self.cache.max += f.end() - f.start());

            match self.cache.max {
                0..65535 => {
                    value_sacan!(self, addr_u16);
                }
                65536..4294967295 => {
                    value_sacan!(self, addr_u32);
                }
                4294967296..18446744073709551615 => {
                    value_sacan!(self, addr_u64);
                }
                _ => {}
            }
        } else {
            match self.cache.max {
                0..65535 => {
                    find_cmp!(self,!=,addr_u16);
                }
                65536..4294967295 => {
                    find_cmp!(self,!=,addr_u32);
                }
                4294967296..18446744073709551615 => {
                    find_cmp!(self,!=,addr_u64);
                }
                _ => {}
            }
        }

        let mut retnum = 0;
        self.cache.addr_u16.iter().for_each(|f| retnum += f.len());
        self.cache.addr_u32.iter().for_each(|f| retnum += f.len());
        self.cache.addr_u64.iter().for_each(|f| retnum += f.len());
        Ok(retnum)
    }

    // TODO
    fn print(&mut self) -> Result<()> {
        match self.cache.max {
            0..65535 => {
                print_abs!(self, addr_u16);
            }
            65536..4294967295 => {
                print_abs!(self, addr_u32);
            }
            4294967296..18446744073709551615 => {
                print_abs!(self, addr_u64);
            }
            _ => {}
        }

        Ok(())
    }

    // TODO 未知值搜索
    fn unknown_scan(&mut self) -> Result<usize> {
        todo!()
    }

    fn value_more(&mut self) -> Result<usize> {
        // find!(self,<,addr_u64);
        let mut retnum = 0;
        self.cache.addr_u16.iter().for_each(|f| retnum += f.len());
        self.cache.addr_u32.iter().for_each(|f| retnum += f.len());
        self.cache.addr_u64.iter().for_each(|f| retnum += f.len());
        Ok(retnum)
    }

    fn value_less(&mut self) -> Result<usize> {
        // find!(self,>);
        let mut retnum = 0;
        self.cache.addr_u16.iter().for_each(|f| retnum += f.len());
        self.cache.addr_u32.iter().for_each(|f| retnum += f.len());
        self.cache.addr_u64.iter().for_each(|f| retnum += f.len());
        Ok(retnum)
    }

    fn value_change(&mut self) -> Result<usize> {
        // find!(self,==);
        let mut retnum = 0;
        self.cache.addr_u16.iter().for_each(|f| retnum += f.len());
        self.cache.addr_u32.iter().for_each(|f| retnum += f.len());
        self.cache.addr_u64.iter().for_each(|f| retnum += f.len());
        Ok(retnum)
    }
}

impl MemExt for Linux {
    fn write(&self, addr: usize, payload: &[u8]) -> Result<usize> {
        self.proc.mem.write_at(payload, addr as u64)?;
        Ok(payload.len())
    }

    fn read(&self, addr: usize, size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; size];
        self.proc.mem.read_at(&mut buf, addr as u64)?;
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

impl MapsExt for Linux {
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
            .filter(|m| {
                m.end() - m.start() > 0 && (m.pathname() == "[heap]" || m.pathname() == "[stack]")
                    || (m.pathname().is_empty() && m.is_read() && m.is_write())
                    || (m.pathname().is_empty() && m.is_read() && m.is_write() && m.is_exec())
            })
            .collect())
    }
}

impl SyscallExt for Linux {
    fn get_ip(&mut self) -> Result<u64> {
        let mut file = File::open(&self.proc.syscall)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(u64::from_str_radix(content.trim().rsplit_once('x').ok_or(Error::PidNotFound)?.1, 16)?)
    }
}

// glibc我草你妈，尼马比你瞎鸡巴改尼马的__libc_dlopen_mode，握草尼马的，怎么不给你妈放glibc里栓起来呢呢，草，给你妈放出来
// todo 由于glibc的瞎b更新此功能在2.34版本以上失效
impl InjectExt for Linux {
    fn inject(&mut self, lib_path: &str) -> Result<()> {
        if lib_path.chars().next().ok_or(Error::ArgsError)? != '/' {
            return Err(Error::ArgsError);
        }

        let libc = "/usr/lib/libc.so.6";

        let buf = std::fs::read(libc)?;

        let sym = Elf::parse(&buf)?
            .find_sym_by_name("__libc_dlopen_mode")
            .ok_or(Error::PidNotFound)?;

        println!("================ {}",sym.st_value);

        let dl = self
            .region_lv0()?
            .iter()
            .find(|m| m.pathname() == libc)
            .ok_or(Error::PidNotFound)?
            .start() as u64
            + sym.st_value;

        let ip = self.get_ip()?;

        let payload = "/tmp/b8b7a4b6-6214-40b1.bin";

        let mut p1: Vec<u8> = vec![
            0x50, 0x53, 0x51, 0x52, 0x55, 0x56, 0x57, 0x41, 0x50, 0x41, 0x51, 0x41, 0x52, 0x41, 0x53, 0x41, 0x54, 0x41,
            0x55, 0x41, 0x56, 0x41, 0x57, 0x48, 0xc7, 0xc0, 0x2, 0x0, 0x0, 0x0, 0x48, 0x8d, 0x3d, 0x64, 0x0, 0x0, 0x0,
            0x48, 0xc7, 0xc6, 0x0, 0x0, 0x0, 0x0, 0x48, 0xc7, 0xc2, 0x0, 0x0, 0x0, 0x0, 0xf, 0x5, 0x49, 0x89, 0xc6,
            0x48, 0xc7, 0xc0, 0x9, 0x0, 0x0, 0x0, 0x48, 0xc7, 0xc7, 0x0, 0x0, 0x0, 0x0, 0x48, 0xc7, 0xc6, 0x0, 0x2,
            0x0, 0x0, 0x48, 0xc7, 0xc2, 0x5, 0x0, 0x0, 0x0, 0x49, 0xc7, 0xc2, 0x2, 0x0, 0x0, 0x0, 0x4d, 0x89, 0xf0,
            0x49, 0xc7, 0xc1, 0x0, 0x0, 0x0, 0x0, 0xf, 0x5, 0x49, 0x89, 0xc7, 0x48, 0xc7, 0xc0, 0x3, 0x0, 0x0, 0x0,
            0x4c, 0x89, 0xf7, 0xf, 0x5, 0x48, 0xc7, 0xc0, 0x57, 0x0, 0x0, 0x0, 0x48, 0x8d, 0x3d, 0x5, 0x0, 0x0, 0x0,
            0xf, 0x5, 0x41, 0xff, 0xe7,
        ];
        p1.extend(payload.as_bytes());
        p1.extend([0]);

        assert_eq!(27, payload.len());

        let mut code = vec![0; p1.len()];

        self.proc.mem.read_exact_at(&mut code, ip)?;
        self.proc.mem.write_all_at(&p1, ip)?;

        // 0xff, 0x15, 0x0 debug

        let mut p2 = vec![
            0x48, 0xc7, 0xc0, 0x2, 0x0, 0x0, 0x0, 0x48, 0x8d, 0x3d, 0x7b, 0x0, 0x0, 0x0, 0x48, 0xc7, 0xc6, 0x2, 0x0,
            0x0, 0x0, 0x48, 0xc7, 0xc2, 0x0, 0x0, 0x0, 0x0, 0xf, 0x5, 0x49, 0x89, 0xc7, 0x48, 0xc7, 0xc0, 0x12, 0x0,
            0x0, 0x0, 0x4c, 0x89, 0xff, 0x48, 0x8d, 0x35, 0x66, 0x0, 0x0, 0x0, 0x48, 0x8b, 0x15, 0x4, 0x1, 0x0, 0x0,
            0x4c, 0x8b, 0x15, 0x5, 0x1, 0x0, 0x0, 0xf, 0x5, 0x48, 0xc7, 0xc0, 0x3, 0x0, 0x0, 0x0, 0x4c, 0x89, 0xff,
            0xf, 0x5, 0x48, 0x89, 0xe5, 0x48, 0x83, 0xe4, 0xf0, 0x48, 0x8d, 0x3d, 0xf1, 0x0, 0x0, 0x0, 0x48, 0xc7,
            0xc6, 0x1, 0x0, 0x0, 0x0, 0xff, 0x15, 0x11, 0x1, 0x0, 0x0, 0x48, 0x89, 0xec, 0x41, 0x5f, 0x41, 0x5e, 0x41,
            0x5d, 0x41, 0x5c, 0x41, 0x5b, 0x41, 0x5a, 0x41, 0x59, 0x41, 0x58, 0x5f, 0x5e, 0x5d, 0x5a, 0x59, 0x5b, 0x58,
            0xff, 0x25, 0xbc, 0x0, 0x0, 0x0, 0x2f, 0x70, 0x72, 0x6f, 0x63, 0x2f, 0x73, 0x65, 0x6c, 0x66, 0x2f, 0x6d,
            0x65, 0x6d, 0x0,
        ];

        p2.extend(&code);
        p2.extend((code.len() as u64).to_le_bytes());
        p2.extend(ip.to_le_bytes());
        p2.extend(lib_path.as_bytes());
        p2.extend([0]);
        p2.extend(dl.to_le_bytes());
        let mut file = File::create(payload)?;
        let metadata = self.proc.mem.metadata()?;

        std::os::unix::fs::chown(payload, Some(metadata.uid()), Some(metadata.gid()))?;
        file.write_all(&p2)?;

        Ok(())
    }
}

// TODO 去掉这个煞笔依赖
pub trait ElfExt {
    fn find_sym_by_name(&self, sym_name: &str) -> Option<Sym>;
}

impl<'a> ElfExt for Elf<'a> {
    fn find_sym_by_name(&self, name: &str) -> Option<Sym> {
        let iter = |syms: &Symtab, strtab: &Strtab| -> Option<Sym> {
            for sym in syms.iter() {
                if let Some(cur_name) = strtab.get_at(sym.st_name) {
                    if cur_name == name {
                        return Some(sym);
                    }
                }
            }
            None
        };

        iter(&self.syms, &self.strtab).or_else(|| iter(&self.dynsyms, &self.dynstrtab))
    }
}
