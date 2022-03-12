#![feature(thread_spawn_unchecked)]
#![feature(once_cell)]

pub mod comm;
pub mod def;
pub mod gui;
pub mod maps;
pub mod mem;
pub mod sdiff;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // unsafe {
    //     PID = comm::get_pid_by_name(args[1].as_str()).unwrap()[0];
    // }
    // println!("{}", unsafe { PID });

    // let o = search_all_r_mem(&1_u8.to_be_bytes());
    // println!("{:?}", o.len());
}
