use crate::error::{Error, Result};

#[derive(Debug, Default)]
pub struct Maps {
    range_start: usize,
    range_end: usize,
    flags: String,
    pathname: String,
}

impl Maps {
    pub fn size(&self) -> usize {
        self.range_end - self.range_start
    }
    pub fn start(&self) -> usize {
        self.range_start
    }
    pub fn end(&self) -> usize {
        self.range_end
    }
    pub fn pathname(&self) -> &str {
        &self.pathname
    }
    pub fn is_exec(&self) -> bool {
        &self.flags[2..3] == "x"
    }
    pub fn is_write(&self) -> bool {
        &self.flags[1..2] == "w"
    }
    pub fn is_read(&self) -> bool {
        &self.flags[0..1] == "r"
    }
}

pub fn parse_proc_maps(contents: &str) -> Result<Vec<Maps>> {
    let mut vec: Vec<Maps> = Vec::new();
    let e = || Error::ParseMapsError;
    for line in contents.split('\n') {
        let mut split = line.split_whitespace();
        let range = split.next();
        if range.is_none() {
            break;
        }

        let mut range_split = range.ok_or_else(e)?.split('-');
        let range_start = range_split.next().ok_or_else(e)?;
        let range_end = range_split.next().ok_or_else(e)?;
        let flags = split.next().ok_or_else(e)?;
        split.next().ok_or_else(e)?;
        split.next().ok_or_else(e)?;
        split.next().ok_or_else(e)?;

        vec.push(Maps {
            range_start: usize::from_str_radix(range_start, 16)?,
            range_end: usize::from_str_radix(range_end, 16)?,
            flags: flags.to_string(),
            pathname: split.collect::<Vec<&str>>().join(" "),
        });
    }
    Ok(vec)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_proc_maps() {
        use crate::maps::parse_proc_maps;
        let contents: &str = r#"563ea224a000-563ea2259000 r--p 00000000 103:05 5920780 /usr/bin/fish
563ea23ea000-563ea2569000 rw-p 00000000 00:00 0 [heap]
7f9e08000000-7f9e08031000 rw-p 00000000 00:00 0"#;
        let maps = parse_proc_maps(contents).unwrap();
        assert_eq!(maps[0].start(), 0x563ea224a000);
        assert_eq!(maps[0].end(), 0x563ea2259000);
        assert_eq!(maps[0].pathname(), "/usr/bin/fish");
        assert_eq!(maps[1].pathname(), "[heap]");
        assert_eq!(maps[2].pathname(), "");
    }
}
