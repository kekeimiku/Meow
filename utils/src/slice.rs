// u8 slice 转数字

pub fn bytes_to_i16(v: &[u8]) -> i16 {
    i16::from_ne_bytes([v[0], v[1]])
}

pub fn bytes_to_u16(v: &[u8]) -> u16 {
    u16::from_ne_bytes([v[0], v[1]])
}

pub fn bytes_to_i32(v: &[u8]) -> i32 {
    i32::from_ne_bytes([v[0], v[1], v[2], v[3]])
}

pub fn bytes_to_u32(v: &[u8]) -> u32 {
    u32::from_ne_bytes([v[0], v[1], v[2], v[3]])
}

pub fn bytes_to_i64(v: &[u8]) -> i64 {
    i64::from_ne_bytes([v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]])
}

pub fn bytes_to_u64(v: &[u8]) -> u64 {
    u64::from_ne_bytes([v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]])
}

pub fn hexstr_to_usize(s: &str) -> Result<usize, ParseIntError> {
    Ok(usize::from_str_radix(&s.replace("0x", ""), 16)?)
}
