use std::fs;

use meow::{
    error::{Error, Result},
    platform::RegionIter,
    region::InfoExt,
};
use utils::debug;
fn main() -> Result<(), Error> {
    let s = fs::read_to_string("/home/keke/maps").unwrap();
    RegionIter::new(&s).for_each(|m| {
        debug!("{}", m.pathname());
    });

    Ok(())
}
