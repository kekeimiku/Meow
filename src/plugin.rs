use std::{collections::HashMap, path::Path, rc::Rc};

use crate::{
    error::{Error, Result},
    mem::MemExt,
    region::Region,
    scan::find_addr_by_region,
};
use libloading::{Library, Symbol};

pub trait Plugin<T: MemExt> {
    fn name(&self) -> &'static str;
    fn call(&self, args: &str, meow: Meow<T>);
}

pub struct Meow<'a, T: MemExt> {
    pub pid: u32,
    pub maps: &'a [Region],
    pub handle: &'a T,
    pub addr: &'a [usize],
}

impl<'a, T> Meow<'a, T>
where
    T: MemExt,
{
    pub fn new(pid: u32, maps: &'a [Region], addr: &'a [usize], handle: &'a T) -> Meow<'a, T> {
        Self {
            pid,
            maps,
            handle,
            addr,
        }
    }

    pub fn get_pid(&self) -> u32 {
        self.pid
    }

    pub fn getmaps(&self) -> Vec<Region> {
        self.maps.to_vec()
    }

    pub fn read(&self, addr: usize, size: usize) {
        let v = self.handle.read(addr, size).unwrap();
        println!("{:?}", v)
    }

    pub fn write(&self, addr: usize, payload: &[u8]) {
        let v = self.handle.write(addr, payload);
        println!("{:?}", v)
    }

    pub fn scan(&self, start: usize, end: usize, value: &[u8]) {
        let v = find_addr_by_region(self.handle, start, end, value);
        println!("{:?}", v)
    }
}

pub struct PluginManager<'a, T: MemExt> {
    pub extends: HashMap<&'a str, Rc<Box<dyn Plugin<T>>>>,
    pub libs: Vec<Library>,
}

impl<T> PluginManager<'_, T>
where
    T: MemExt,
{
    pub fn new() -> Self {
        Self {
            extends: HashMap::new(),
            libs: Vec::new(),
        }
    }
    pub fn load<P>(&mut self, filepath: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        type Ext<T> = unsafe fn() -> *mut dyn Plugin<T>;
        let lib = unsafe { Library::new(filepath.as_ref()) }?;
        self.libs.push(lib);
        let constructor: Symbol<Ext<T>> = unsafe {
            self.libs
                .last()
                .ok_or_else(|| Error::New("err".into()))?
                .get(b"plugin_start")
        }?;
        let boxed_raw = unsafe { constructor() };
        let lib = unsafe { Box::from_raw(boxed_raw) };

        self.extends.insert(lib.name(), Rc::new(lib));

        Ok(())
    }

    pub fn select(&self, target: &str) -> Result<Rc<Box<dyn Plugin<T>>>> {
        self.extends
            .get(target)
            .cloned()
            .ok_or_else(|| Error::New("err".into()))
    }

    pub fn unload() {
        todo!()
    }
}
