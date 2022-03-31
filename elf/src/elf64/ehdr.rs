use crate::{pub_struct, util::*};

pub_struct!(Elf64_Ehdr {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
});

impl<'a> Elf64_Ehdr {
    pub fn new(bytes: &'a [u8]) -> Self {
        let e_ident = bytes[0..16].try_into().unwrap();
        let e_type = bytes_to_u16(&bytes[16..18]);
        let e_machine = bytes_to_u16(&bytes[18..20]);
        let e_version = bytes_to_u32(&bytes[20..24]);
        let e_entry = bytes_to_u64(&bytes[24..32]);
        let e_phoff = bytes_to_u64(&bytes[32..40]);
        let e_shoff = bytes_to_u64(&bytes[40..48]);
        let e_flags = bytes_to_u32(&bytes[48..52]);
        let e_ehsize = bytes_to_u16(&bytes[52..54]);
        let e_phentsize = bytes_to_u16(&bytes[54..56]);
        let e_phnum = bytes_to_u16(&bytes[56..58]);
        let e_shentsize = bytes_to_u16(&bytes[58..60]);
        let e_shnum = bytes_to_u16(&bytes[60..62]);
        let e_shstrndx = bytes_to_u16(&bytes[62..64]);
        Elf64_Ehdr {
            e_ident,
            e_type,
            e_machine,
            e_version,
            e_entry,
            e_phoff,
            e_shoff,
            e_flags,
            e_ehsize,
            e_phentsize,
            e_phnum,
            e_shentsize,
            e_shnum,
            e_shstrndx,
        }
    }
}
