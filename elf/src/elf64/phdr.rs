use crate::{pub_struct, util::*};

pub_struct!(Elf64_Phdr {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
});

impl<'a> Elf64_Phdr {
    pub fn new(bytes: &'a [u8], offset: u64) -> Self {
        let offset_bytes = &bytes[offset as usize..];
        let p_type = bytes_to_u32(&offset_bytes[0..4]);
        let p_flags = bytes_to_u32(&offset_bytes[4..8]);
        let p_offset = bytes_to_u64(&offset_bytes[8..16]);
        let p_vaddr = bytes_to_u64(&offset_bytes[16..24]);
        let p_paddr = bytes_to_u64(&offset_bytes[24..32]);
        let p_filesz = bytes_to_u64(&offset_bytes[32..40]);
        let p_memsz = bytes_to_u64(&offset_bytes[40..48]);
        let p_align = bytes_to_u64(&offset_bytes[48..56]);
        Elf64_Phdr {
            p_type,
            p_flags,
            p_offset,
            p_vaddr,
            p_paddr,
            p_filesz,
            p_memsz,
            p_align,
        }
    }
}
