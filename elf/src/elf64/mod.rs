mod ehdr;
mod phdr;
mod shdr;

use self::{ehdr::Elf64_Ehdr, phdr::Elf64_Phdr, shdr::Elf64_Shdr};

pub struct Parse<'a> {
    bytes: &'a [u8],
    ehdr: Elf64_Ehdr,
}

impl<'a> Parse<'_> {
    pub fn new(bytes: &'a [u8]) -> Parse {
        let ehdr = Elf64_Ehdr::new(bytes);

        Parse { bytes, ehdr }
    }

    pub fn ehdr(&self) -> &Elf64_Ehdr {
        &self.ehdr
    }

    fn phdr_nth(&self, index: u16) -> Elf64_Phdr {
        Elf64_Phdr::new(
            self.bytes,
            self.ehdr.e_phoff + index as u64 * self.ehdr.e_phentsize as u64,
        )
    }

    pub fn phdr_iter(&self) -> Elf64PhdrIter {
        Elf64PhdrIter {
            index: 0,
            elf64: self,
        }
    }

    fn shdr_nth(&self, index: u16) -> Elf64_Shdr {
        Elf64_Shdr::new(
            self.bytes,
            self.ehdr.e_shoff + index as u64 * self.ehdr.e_shentsize as u64,
        )
    }

    pub fn shdr_iter(&self) -> Elf64ShdrIter {
        Elf64ShdrIter {
            index: 0,
            elf64: self,
        }
    }
}

pub struct Elf64PhdrIter<'a> {
    index: u16,
    elf64: &'a Parse<'a>,
}

impl Iterator for Elf64PhdrIter<'_> {
    type Item = Elf64_Phdr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.elf64.ehdr.e_phnum {
            return None;
        }

        let phdr = self.elf64.phdr_nth(self.index);
        self.index += 1;
        Some(phdr)
    }
}

pub struct Elf64ShdrIter<'a> {
    index: u16,
    elf64: &'a Parse<'a>,
}

impl Iterator for Elf64ShdrIter<'_> {
    type Item = Elf64_Shdr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.elf64.ehdr.e_shnum {
            return None;
        }

        let shdr = self.elf64.shdr_nth(self.index);
        self.index += 1;
        Some(shdr)
    }
}
