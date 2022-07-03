use std::{collections::HashMap, path::Path, rc::Rc};

use crate::error::{Error, Result};
use libloading::{Library, Symbol};

pub trait Plugin {
    fn name(&self) -> &'static str;
    fn call(&self, args: &str,meow: Meow);
}

pub struct Meow {
    pub pid:u32
}

impl Meow {
    pub fn get_pid(&self)->u32{
        self.pid
    }
}

#[derive(Default)]
pub struct PluginManager<'a> {
    pub extends: HashMap<&'a str, Rc<Box<dyn Plugin>>>,
    pub libs: Vec<Library>,
}

impl PluginManager<'_> {
    pub fn load<P>(&mut self, filepath: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        type Ext = unsafe fn() -> *mut dyn Plugin;
        let lib = unsafe { Library::new(filepath.as_ref()) }?;
        self.libs.push(lib);
        let constructor: Symbol<Ext> = unsafe {
            self.libs
                .last()
                .ok_or_else(|| Error::New("err".into()))?
                .get(b"plugin_start")
        }?;
        let boxed_raw = unsafe { constructor() };
        let app = unsafe { Box::from_raw(boxed_raw) };
        self.extends.insert(app.name(), Rc::new(app));

        Ok(())
    }

    pub fn select(&self, target: &str) -> Result<Rc<Box<dyn Plugin>>> {
        self.extends
            .get(target)
            .cloned()
            .ok_or_else(|| Error::New("err".into()))
    }

    pub fn unload() {
        todo!()
    }
}
