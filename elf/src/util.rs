#[macro_export]
macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[repr(C)]
        #[derive(Debug)]
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

pub fn bytes_to_u16(bytes: &[u8]) -> u16 {
    u16::from_le_bytes(bytes[..2].try_into().unwrap())
}

pub fn bytes_to_u32(bytes: &[u8]) -> u32 {
    let arr = bytes[..4].try_into().unwrap();
    u32::from_le_bytes(arr)
}

pub fn bytes_to_u64(bytes: &[u8]) -> u64 {
    let arr = bytes[..8].try_into().unwrap();
    u64::from_le_bytes(arr)
}
