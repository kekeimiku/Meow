use std::{collections::HashMap, path::Path, rc::Rc};

use crate::error::{Error, Result};
use libloading::{Library, Symbol};

pub trait Plugin {
    fn name(&self) -> &'static str;
    fn call(&self, args: &str);
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
        let lib = unsafe { Box::from_raw(boxed_raw) };

        self.extends.insert(lib.name(), Rc::new(lib));

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
