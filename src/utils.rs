use crate::error::Result;

pub fn hexstr_to_usize(s: &str) -> Result<usize> {
    Ok(usize::from_str_radix(&s.replace("0x", ""), 16)?)
}
