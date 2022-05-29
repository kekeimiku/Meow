use crate::{
    error::{Error, Result},
    ext::{InjectExt, MemExt, ScanExt, Type},
    utils::hexstr_to_usize,
};

#[cfg(target_os = "linux")]
use crate::linux::Linux;

pub fn prompt(name: &str) -> Result<Vec<String>> {
    let mut line = String::new();
    print!("{}", name);
    std::io::Write::flush(&mut std::io::stdout())?;
    std::io::stdin().read_line(&mut line)?;
    Ok(line.replace('\n', "").split_whitespace().map(String::from).collect())
}

// 下面的代码很拉，实在懒得动了，草拟马破比交互界面耐心给我磨没了，有很多问题，懒得弄了。 TODO 等个好心人重构
//
macro_rules! merr {
    ($m:expr,$ok:expr,$err:expr) => {
        match $m {
            Ok(v) => println!("{} {:?}", $ok, v),
            Err(e) => println!("{} {}", $err, e),
        }
    };
}

pub fn start() -> Result<()> {
    let pid = std::env::args()
        .nth(1)
        .ok_or_else(|| Error::New("parm error".into()))?
        .parse::<u32>()?;
    #[cfg(target_os = "linux")]
    let mut app = Linux::new(pid)?;

    #[cfg(target_os = "windows")]
    let mut app = Windows::new(pid)?;

    #[cfg(target_os = "macos")]
    let mut app = Macos::new();

    loop {
        if let Err(e) = cmd(&mut app) {
            println!("{}", e);
        }
    }
}

fn cmd(app: &mut Linux) -> Result<()> {
    let prompt = prompt("> ")?;
    let input = prompt.iter().map(String::as_str).collect::<Vec<&str>>();
    if input.is_empty() {
        println!("参数为空");
    } else {
        match input[0] {
            "find" | "f" => {
                app.input(Type::I32, input[1])?;
                let num = app.value_scan()?;
                println!("{}", num)
            }
            "<"=>{
                app.value_less()?;
            }
            "write" | "w" => {
                let addr = hexstr_to_usize(input[1])?;
                let val = &input[2].parse::<i32>()?.to_le_bytes();
                merr!(app.write(addr, val), "写入成功,字节数: ", "写入失败: Error: ");
            }
            "read" | "r" => {
                let addr = hexstr_to_usize(input[1])?;
                let size = &input[2].parse::<usize>()?;
                merr!(app.read(addr, *size), "", "Error: ");
            }
            "print" | "p" => {
                app.print()?;
            }
            "inject" | "inj" => {
                let libpath = &input[1];
                merr!(app.inject(libpath), "注入成功", "注入失败,Error: ");
            }
            "lock" => {
                let addr = hexstr_to_usize(input[1])?;
                let payload = &input[2].parse::<i32>()?.to_le_bytes();
                app.freeze(addr, payload.to_vec())?;
            }
            "dump" => {
                let addr = hexstr_to_usize(input[1])?;
                let size = input[2].parse::<usize>()?;
                let path = input[3];
                app.dump(addr, size, path)?;
            }
            "clear" | "clean" => app.clear(),
            _ => {}
        }
    }

    Ok(())
}
