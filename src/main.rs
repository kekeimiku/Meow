#![feature(thread_spawn_unchecked)]
#![feature(once_cell)]
use std::{
    collections::HashSet,
    env,
    lazy::{Lazy, SyncLazy},
};

use dashmap::{DashMap, DashSet};
use lince::mem::search_index;

use crate::def::Cheat;

pub mod comm;
pub mod def;
pub mod maps;
pub mod mem;

pub static DDD: SyncLazy<DashMap<usize, usize>> = SyncLazy::new(|| DashMap::new());

fn main() {
    let args: Vec<String> = env::args().collect();

    let pid = comm::get_pid_by_name(args[1].as_str()).unwrap()[0];

    let app = Cheat::new(pid);

    let m = app.readmaps_all();

    let mut ok: Vec<usize> = Default::default();
    m.iter().for_each(|f| {
        let buf = app.read_bytes(f.start(), f.end() - f.start());
        let target = search_index(&buf.unwrap(), &1_u8.to_be_bytes())
            .iter()
            .map(|m| m + f.start())
            .collect::<Vec<usize>>();
        if !target.is_empty() {
            target.iter().for_each(|f| ok.push(*f))
        }
    });

    println!("{:?}",ok);
}
