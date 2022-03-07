#![feature(test)]

use std::env;

use crate::def::Cheat;

pub mod comm;
pub mod def;
pub mod maps;
pub mod mem;



fn main() {
    //line();

    let args: Vec<String> = env::args().collect();

    let pid = comm::get_pid_by_name(args[1].as_str()).unwrap()[0];

    let app = Cheat::new(pid);

    let m = app.readmaps_c_heap();
    println!(
        "{}-{} {}",
        m.start(),
        m.end(),
        m.pathname()
    );

    let a = app.readmaps_c_heap();

    let buf = app.read_bytes(a.start(), a.end() - a.start());

    let target = Cheat::search_index(&buf.unwrap(), "hello".as_bytes())
        .iter()
        .map(|f| f + a.start())
        .collect::<Vec<usize>>();

    println!("{:?}", target);
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_main(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..1000 {
                main()
            }
        });
    }
}

// enum Type {
//     WORD,
//     QWORD,
//     DWORD,
//     FLOAT,
// }
