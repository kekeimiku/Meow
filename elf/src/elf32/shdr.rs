use crate::{pub_struct, util::*};

pub_struct!(Elf32_Shdr {
    sh_name: u32,
    sh_type: u32,
    sh_flags: u32,
    sh_addr: u32,
    sh_offset: u32,
    sh_size: u32,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u32,
    sh_entsize: u32,
});

impl<'a> Elf32_Shdr {
    pub fn new(bytes: &'a [u8], offset: u32) -> Self {
        let offset_bytes = &bytes[offset as usize..];
        let sh_name = bytes_to_u32(&offset_bytes[0..4]);
        let sh_type = bytes_to_u32(&offset_bytes[4..8]);
        let sh_flags = bytes_to_u32(&offset_bytes[8..12]);
        let sh_addr = bytes_to_u32(&offset_bytes[12..16]);
        let sh_offset = bytes_to_u32(&offset_bytes[16..20]);
        let sh_size = bytes_to_u32(&offset_bytes[20..24]);
        let sh_link = bytes_to_u32(&offset_bytes[24..28]);
        let sh_info = bytes_to_u32(&offset_bytes[28..32]);
        let sh_addralign = bytes_to_u32(&offset_bytes[32..36]);
        let sh_entsize = bytes_to_u32(&offset_bytes[36..40]);

        Elf32_Shdr {
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
