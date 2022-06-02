use crate::{
    error::{Error, Result},
    scan::Scan,
};

pub fn prompt(name: &str) -> Result<Vec<String>> {
    let mut line = String::new();
    print!("{}", name);
    std::io::Write::flush(&mut std::io::stdout())?;
    std::io::stdin().read_line(&mut line)?;
    Ok(line
        .replace('\n', "")
        .split_whitespace()
        .map(String::from)
        .collect())
}

pub fn start() -> Result<()> {
    let pid = std::env::args()
        .nth(1)
        .ok_or_else(|| Error::New("parm error".into()))?
        .parse::<u32>()?;
    let mut app = Scan::new(pid)?;

    loop {
        if let Err(e) = cmd(&mut app) {
            println!("{}", e);
        }
    }
}

fn cmd(app: &mut Scan) -> Result<()> {
    let prompt = prompt("> ")?;
    let input = prompt.iter().map(String::as_str).collect::<Vec<&str>>();
    if input.is_empty() {
        println!("parm is empty");
    } else {
        match input[0] {
            "find" | "f" => {
                app.scan(&input[1].parse::<i32>()?.to_le_bytes())?;
            }
            "p" => {
                app.print();
            }
            _ => {}
        }
    }

    Ok(())
}
