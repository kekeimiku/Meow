use std::io::Write;
use crate::Result;

struct ReadLine {
    cmd: Cmd,
    prompt: String,
}

impl ReadLine {
    pub fn new(c:String)->Self{
        Self{
            cmd:Cmd::FIND,
            prompt:c,
        }
    }



}

fn prompt(name: &str) -> Result<Vec<String>> {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut line)?;
    Ok(line
        .replace('\n', "")
        .split_whitespace()
        .map(String::from)
        .collect())
}

enum Cmd{
    FIND,
    SET
}

impl Cmd{

}