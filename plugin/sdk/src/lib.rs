pub struct Meow {
    pid:u32
}

impl Meow {
    pub fn get_pid(&self)->u32{
        self.pid
    }
}

pub trait Plugin {
    fn name(&self) -> &'static str;
    fn call(&self, args: &str,meow: Meow);
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