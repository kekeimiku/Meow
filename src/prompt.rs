use crate::error::Result;

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
    let prompt = prompt("> ")?;
    let input = prompt.iter().map(String::as_str).collect::<Vec<&str>>();
    if input.is_empty() {
        println!("参数为空");
    } else {
        match input[0] {
            "find" => {
                let _num = input[1].parse::<i32>()?.to_ne_bytes();
                // app.find(&num);
            }
            _ => {}
        }
    }

    Ok(())
}
