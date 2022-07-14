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
    loop {
        let prompt = prompt("> ")?;
        let input = prompt.iter().map(String::as_str).collect::<Vec<&str>>();
        if input.is_empty() {
            println!("参数为空");
        } else {
            match input[0] {
                "find" => {
                    let _arg1 = input[1].parse::<u8>().unwrap().to_ne_bytes();
                }

                "re" => {
                    let _arg1 = input[1].parse::<u8>().unwrap().to_ne_bytes();
                }
                "p" => {}

                "len" => {}
                "q" => {
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}
