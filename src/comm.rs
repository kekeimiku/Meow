use std::{
    env,
    fs::{self, File},
    io::Read,
    lazy::SyncLazy,
    path::Path,
    process::exit,
};

use args::{Args, Error};

#[derive(Debug)]
struct TestArgs {
    help: bool,
    pid: Option<u32>,
    name: Option<String>,
}

fn init_args() -> Result<TestArgs, Error> {
    let mut args = Args::new()?;
    let args = TestArgs {
        help: args.contains(["--h", "-h"]),
        pid: args.init("--pid")?,
        name: args.init("--name")?,
    };

    if args.help {
        println!(
            r#"help
        -h --help
        --pid 12345
        --name hello
        "#
        );
    }

    Ok(args)
}

pub static PID: SyncLazy<i32> = SyncLazy::new(|| -> i32 {
    match env::args().nth(1) {
        Some(arg) => get_pid_by_name(&arg).unwrap_or_else(|| {
            println!("没找到");
            exit(1);
        }),
        _ => {
            println!("需要一个命令行参数");
            exit(1);
        }
    }
});

pub fn get_pid_by_name(name: &str) -> Option<i32> {
    let mut pid: Option<i32> = Default::default();
    fs::read_dir("/proc").unwrap().for_each(|process| {
        let comm = format!("{}/comm", process.unwrap().path().display());
        let file = File::open(Path::new(&comm));
        if let Ok(mut f) = file {
            let mut s = String::new();
            f.read_to_string(&mut s).unwrap();
            if s.trim() == name {
                pid = Some(
                    comm.split('/').collect::<Vec<&str>>()[2]
                        .parse::<i32>()
                        .unwrap(),
                );
            }
        }
    });
    pid
}
