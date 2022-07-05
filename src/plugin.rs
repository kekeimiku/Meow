use std::{collections::HashMap, fs, path::Path, rc::Rc};

use crate::error::{Error, Result};
use libloading::{Library, Symbol};

// TODO
pub trait Plugin {
    fn name(&self) -> &'static str;
    fn call(&self, args: &str);
}

#[derive(Default)]
pub struct PluginManager<'a> {
    extends: HashMap<&'a str, Rc<Box<dyn Plugin>>>,
    libs: Vec<Library>,
}

impl PluginManager<'_> {
    pub fn load_all<P>(&mut self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let dir = fs::read_dir(path)?;
        for entry in dir {
            let entry = entry?.path();
            if entry.extension().is_some_and(|x| *x == "so") {
                self.load(entry)?;
            };
        }
        Ok(())
    }

    pub fn load<P>(&mut self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        type Ext = unsafe fn() -> *mut dyn Plugin;
        let lib = unsafe { Library::new(path.as_ref()) }?;
        self.libs.push(lib);
        let constructor: Symbol<Ext> = unsafe { self.libs.last().unwrap().get(b"plugin_start") }?;
        let boxed_raw = unsafe { constructor() };
        let lib = unsafe { Box::from_raw(boxed_raw) };

        self.extends.insert(lib.name(), Rc::new(lib));

        Ok(())
    }

    pub fn select(&self, target: &str) -> Result<Rc<Box<dyn Plugin>>> {
        self.extends
            .get(target)
            .cloned()
            .ok_or_else(|| Error::New("without this plugin".into()))
    }

    pub fn unload() {
        todo!()
    }
}
