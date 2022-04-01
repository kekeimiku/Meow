use crate::{pub_struct, util::*};

pub_struct!(Elf32_Phdr {
    p_type: u32,
    p_flags: u32,
    p_offset: u32,
    p_vaddr: u32,
    p_paddr: u32,
    p_filesz: u32,
    p_memsz: u32,
    p_align: u32,
});

impl<'a> Elf32_Phdr {
    pub fn new(bytes: &'a [u8], offset: u32) -> Self {
        let offset_bytes = &bytes[offset as usize..];
        let p_type = bytes_to_u32(&offset_bytes[0..4]);
        let p_offset = bytes_to_u32(&offset_bytes[4..8]);
        let p_vaddr = bytes_to_u32(&offset_bytes[8..12]);
        let p_paddr = bytes_to_u32(&offset_bytes[12..16]);
        let p_filesz = bytes_to_u32(&offset_bytes[16..20]);
        let p_memsz = bytes_to_u32(&offset_bytes[20..24]);
        let p_flags = bytes_to_u32(&offset_bytes[24..28]);
        let p_align = bytes_to_u32(&offset_bytes[28..32]);
        Elf32_Phdr {
            p_type,
            p_offset,
            p_vaddr,
            p_paddr,
            p_filesz,
            p_memsz,
            p_flags,
            p_align,
        }
    }
}
