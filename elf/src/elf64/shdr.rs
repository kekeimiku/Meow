use crate::{pub_struct, util::*};

pub_struct!(Elf64_Shdr {
    sh_name: u32,
    sh_type: u32,
    sh_flags: u64,
    sh_addr: u64,
    sh_offset: u64,
    sh_size: u64,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u64,
    sh_entsize: u64,
});

impl<'a> Elf64_Shdr {
    pub fn new(bytes: &'a [u8], offset: u64) -> Self {
        let offset_bytes = &bytes[offset as usize..];
        let sh_name = bytes_to_u32(&offset_bytes[0..4]);
        let sh_type = bytes_to_u32(&offset_bytes[4..8]);
        let sh_flags = bytes_to_u64(&offset_bytes[8..16]);
        let sh_addr = bytes_to_u64(&offset_bytes[16..24]);
        let sh_offset = bytes_to_u64(&offset_bytes[24..32]);
        let sh_size = bytes_to_u64(&offset_bytes[32..40]);
        let sh_link = bytes_to_u32(&offset_bytes[40..44]);
        let sh_info = bytes_to_u32(&offset_bytes[44..48]);
        let sh_addralign = bytes_to_u64(&offset_bytes[48..56]);
        let sh_entsize = bytes_to_u64(&offset_bytes[56..64]);

        Elf64_Shdr {
            sh_name,
            sh_type,
            sh_flags,
            sh_addr,
            sh_offset,
            sh_size,
            sh_link,
            sh_info,
            sh_addralign,
            sh_entsize,
        }
    }
}
