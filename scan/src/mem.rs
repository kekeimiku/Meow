use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs;
use std::os::unix::prelude::{FileExt, MetadataExt};
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;

use goblin::elf::{Elf, Sym, Symtab};
use goblin::strtab::Strtab;
use memchr::memmem::find_iter;

use crate::error::{Error, Result};
use crate::maps::{parse_proc_maps, MapRange};

pub struct Process {
    pub pid: i32,
    pub mem: File,
    pub maps: PathBuf,
    pub syscall: PathBuf,
    pub cache: Cache,
}

impl Process {
    pub fn new(pid: i32) -> Result<Self> {
        let mem = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&Path::new(&format!("/proc/{}/mem", pid)))?;
        let maps = PathBuf::from(&format!("/proc/{}/maps", pid));
        let syscall = PathBuf::from(&format!("/proc/{}/syscall", pid));
        Ok(Self {
            pid,
            mem,
            maps,
            syscall,
            cache: Cache::new(),
        })
    }
}

pub trait MemExt {
    fn write(&self, addr: usize, payload: &[u8]) -> Result<usize>;
    fn read(&self, addr: usize, size: usize) -> Vec<u8>;
    fn dump(&self, addr: usize, size: usize, path: &str) -> Result<()>;
    fn freeze(&self, va: Vec<usize>, payload: Vec<u8>) -> Result<()>;
    fn unfreeze(&self, va: Vec<usize>, payload: Vec<u8>) -> Result<()>;
}

pub trait MapsExt {
    fn readmaps_all(&mut self) -> Result<Vec<MapRange>>;
    fn readmaps_v1(&mut self) -> Result<Vec<MapRange>>;
    fn getlibc_addr(&mut self) -> Result<usize>;
}

pub trait SyscallExt {
    fn get_ip(&mut self) -> Result<u64>;
}

pub trait InjectExt {
    fn inject(&mut self, lib_path: &str) -> Result<()>;
}

pub trait ScanExt {
    fn scan(&mut self, input: Vec<u8>, cmd: u8) -> Result<()>;
}

pub struct Cache {
    pub maps_cache: Vec<MapRange>,
    pub addr_cache: Vec<Vec<usize>>,
}

impl Cache {
    fn new() -> Self {
        Self {
            maps_cache: Vec::new(),
            addr_cache: Vec::new(),
        }
    }
}

impl ScanExt for Process {
    // cmd: 1：新搜索， 2：过滤值为input的地址，3：过滤小于值input的地址，4：过滤值大于input的地址，5：打印地址列表，6:打印maps列表
    fn scan(&mut self, input: Vec<u8>, cmd: u8) -> Result<()> {
        match cmd {
            1 => {
                self.cache.maps_cache = self.readmaps_v1()?.into_iter().collect::<Vec<MapRange>>();
                self.cache.addr_cache = self
                    .cache
                    .maps_cache
                    .iter()
                    .map(|m| {
                        find_iter(&self.read(m.start(), m.end() - m.start()), &input)
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();
            }
            3 => {
                let v: [u8; 4] = input[0..4].try_into().unwrap();
                for (m, k1) in self
                    .cache
                    .maps_cache
                    .iter()
                    .zip(0..self.cache.addr_cache.len())
                {
                    let mem = self.read(m.start(), m.end() - m.start());
                    for k2 in (0..self.cache.addr_cache[k1].len()).rev() {
                        if mem
                            [self.cache.addr_cache[k1][k2]..self.cache.addr_cache[k1][k2] + v.len()]
                            != v
                        {
                            self.cache.addr_cache[k1].swap_remove(k2);
                            self.cache.addr_cache[k1].shrink_to_fit();
                        }
                    }
                }
            }

            5 => {
                println!("maps{}", self.cache.maps_cache.len());
                println!("addrf{} ", self.cache.addr_cache.len());
                let val = self
                    .cache
                    .addr_cache
                    .iter()
                    .enumerate()
                    .map(|(k, v)| {
                        v.iter()
                            .map(|a| a + self.cache.maps_cache[k].start())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();
                val.iter()
                    .for_each(|f| f.iter().for_each(|x| println!("0x{:x}", x)));
                // println!("{:?}", addr_cache);
            }
            _ => {}
        }

        Ok(())
    }
}

impl MemExt for Process {
    fn write(&self, addr: usize, payload: &[u8]) -> Result<usize> {
        self.mem.write_at(payload, addr as u64)?;
        Ok(payload.len())
    }

    fn read(&self, addr: usize, size: usize) -> Vec<u8> {
        let mut buf = vec![0; size];
        if let Err(err) = self.mem.read_at(&mut buf, addr as u64) {
            eprintln!("Err: {}", err)
        };
        buf
    }

    fn dump(&self, addr: usize, size: usize, path: &str) -> Result<()> {
        let mut file = File::create(Path::new(path))?;
        let data = self.read(addr, size);
        file.write_all(&data)?;
        Ok(())
    }

    fn freeze(&self, va: Vec<usize>, payload: Vec<u8>) -> Result<()> {
        let f = self.mem.try_clone()?;
        std::thread::spawn(move || loop {
            va.iter().for_each(|addr| {
                f.write_at(&payload, *addr as u64).unwrap();
                sleep(Duration::from_millis(20));
            })
        });
        Ok(())
    }

    // TODO
    fn unfreeze(&self, _va: Vec<usize>, _payload: Vec<u8>) -> Result<()> {
        Ok(())
    }
}

impl MapsExt for Process {
    fn readmaps_all(&mut self) -> Result<Vec<MapRange>> {
        let mut file = File::open(&self.maps)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(parse_proc_maps(&contents)?.into_iter().collect())
    }

    fn readmaps_v1(&mut self) -> Result<Vec<MapRange>> {
        let mut file = File::open(&self.maps)?;
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

    fn getlibc_addr(&mut self) -> Result<usize> {
        Ok(self
            .readmaps_all()?
            .into_iter()
            .find(|m| m.pathname() == "/usr/lib/libc.so.6")
            .ok_or(Error::PidNotFound)?
            .start())
    }
}

impl SyscallExt for Process {
    fn get_ip(&mut self) -> Result<u64> {
        let mut file = File::open(&self.syscall)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(u64::from_str_radix(
            content.trim().rsplit_once('x').ok_or(Error::PidNotFound)?.1,
            16,
        )?)
    }
}

// TODO 移除goblin依赖，使用更好的方式组装payload
impl InjectExt for Process {
    fn inject(&mut self, lib_path: &str) -> Result<()> {
        let buf = std::fs::read("/usr/lib/libc.so.6")?;
        let sym = Elf::parse(&buf)
            .ok()
            .unwrap()
            .find_sym_by_name("__libc_dlopen_mode")
            .ok_or(Error::PidNotFound)?;
        let dl = self.getlibc_addr()? as u64 + sym.st_value;

        let ip = self.get_ip()?;

        let payload = "/tmp/tmp.bin";

        let mut p1: Vec<u8> = vec![
            80, 83, 81, 82, 85, 86, 87, 65, 80, 65, 81, 65, 82, 65, 83, 65, 84, 65, 85, 65, 86, 65,
            87, 72, 199, 192, 2, 0, 0, 0, 72, 141, 61, 100, 0, 0, 0, 72, 199, 198, 0, 0, 0, 0, 72,
            199, 194, 0, 0, 0, 0, 15, 5, 73, 137, 198, 72, 199, 192, 9, 0, 0, 0, 72, 199, 199, 0,
            0, 0, 0, 72, 199, 198, 0, 2, 0, 0, 72, 199, 194, 5, 0, 0, 0, 73, 199, 194, 2, 0, 0, 0,
            77, 137, 240, 73, 199, 193, 0, 0, 0, 0, 15, 5, 73, 137, 199, 72, 199, 192, 3, 0, 0, 0,
            76, 137, 247, 15, 5, 72, 199, 192, 87, 0, 0, 0, 72, 141, 61, 5, 0, 0, 0, 15, 5, 65,
            255, 231,
        ];
        p1.extend(payload.as_bytes());
        p1.extend([0]);

        let mut code = vec![0; p1.len()];

        self.mem.read_exact_at(&mut code, ip).unwrap();
        self.mem.write_all_at(&p1, ip).unwrap();

        let mut p2 = vec![
            72, 199, 192, 2, 0, 0, 0, 72, 141, 61, 123, 0, 0, 0, 72, 199, 198, 2, 0, 0, 0, 72, 199,
            194, 0, 0, 0, 0, 15, 5, 73, 137, 199, 72, 199, 192, 18, 0, 0, 0, 76, 137, 255, 72, 141,
            53, 102, 0, 0, 0, 72, 139, 21, 245, 0, 0, 0, 76, 139, 21, 246, 0, 0, 0, 15, 5, 72, 199,
            192, 3, 0, 0, 0, 76, 137, 255, 15, 5, 72, 137, 229, 72, 131, 228, 240, 72, 141, 61,
            226, 0, 0, 0, 72, 199, 198, 1, 0, 0, 0, 255, 21, 0, 1, 0, 0, 72, 137, 236, 65, 95, 65,
            94, 65, 93, 65, 92, 65, 91, 65, 90, 65, 89, 65, 88, 95, 94, 93, 90, 89, 91, 88, 255,
            37, 173, 0, 0, 0, 47, 112, 114, 111, 99, 47, 115, 101, 108, 102, 47, 109, 101, 109, 0,
        ];

        p2.extend(&code);
        p2.extend((code.len() as u64).to_le_bytes());
        p2.extend(ip.to_le_bytes());
        p2.extend(lib_path.as_bytes());
        p2.extend([0]);
        p2.extend(dl.to_le_bytes());
        let mut file = File::create(payload).unwrap();
        let metadata = self.mem.metadata().unwrap();

        fs::chown(payload, Some(metadata.uid()), Some(metadata.gid()))?;
        file.write_all(&p2)?;

        Ok(())
    }
}

pub(crate) trait ElfExt {
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

pub fn ttt() {
    let mut app = Process::new(22266).unwrap();
    app.inject("/home/keke/testlib/target/debug/libtttt.so")
        .unwrap();
}
