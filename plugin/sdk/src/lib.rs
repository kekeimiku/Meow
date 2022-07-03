#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub trait Plugin {
    fn name(&self) -> &'static str;
    fn call(&self, args: &str, meow: Meow);
}

#[derive(Debug, Default, Clone)]
pub struct Region {
    pub range_start: usize,
    pub range_end: usize,
    pub flags: String,
    pub pathname: String,
}

pub trait MemExt {
    fn read(&self, addr: usize, size: usize) -> Result<Vec<u8>>;
    fn write(&self, addr: usize, payload: &[u8]) -> Result<usize>;
}

pub struct Meow<'a> {
    pub pid: u32,
    pub maps: &'a [Region],
    pub handle: &'a dyn MemExt,
}

impl<'a> Meow<'a> {
    pub fn new(pid: u32, maps: &'a [Region], handle: &'a dyn MemExt) -> Meow<'a> {
        Self { pid, maps, handle }
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
}

#[macro_export(local_inner_macros)]
macro_rules! plugin_start {
    ($pt:ty,$ph:path) => {
        #[allow(improper_ctypes_definitions)]
        #[no_mangle]
        pub extern "C" fn plugin_start() -> *mut dyn Plugin {
            let constructor: fn() -> $pt = $ph;
            let object = constructor();
            let boxed: Box<dyn Plugin> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}
