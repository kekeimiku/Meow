#![feature(thread_spawn_unchecked)]
#![feature(once_cell)]
use std::{
    env,
    lazy::{Lazy, SyncLazy},
};

use crate::{def::PID, maps::*, mem::*};

pub mod comm;
pub mod def;
pub mod maps;
pub mod mem;

fn main() {
    let args: Vec<String> = env::args().collect();
    unsafe {
        PID = comm::get_pid_by_name(args[1].as_str()).unwrap()[0];
    }

    let o = search_all_mem("".as_bytes());
    println!("{:?}", o);
}
