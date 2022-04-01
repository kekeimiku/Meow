use self::{ehdr::Elf32_Ehdr, phdr::Elf32_Phdr, shdr::Elf32_Shdr};

mod ehdr;
mod phdr;
mod shdr;

pub struct Elf32<'a> {
    bytes: &'a [u8],
    ehdr: Elf32_Ehdr,
}

impl<'a> Elf32<'_> {
    pub fn new(bytes: &'a [u8]) -> Elf32 {
        let ehdr = Elf32_Ehdr::new(bytes);
        Elf32 { bytes, ehdr }
    }

    pub fn ehdr(&self) -> &Elf32_Ehdr {
        &self.ehdr
    }

    fn phdr_nth(&self, index: u16) -> Elf32_Phdr {
        Elf32_Phdr::new(
            self.bytes,
            self.ehdr.e_phoff + index as u32 * self.ehdr.e_phentsize as u32,
        )
    }

    pub fn phdr_iter(&self) -> Elf32PhdrIter {
        Elf32PhdrIter {
            index: 0,
            elf32: self,
        }
    }

    fn shdr_nth(&self, index: u16) -> Elf32_Shdr {
        Elf32_Shdr::new(
            self.bytes,
            self.ehdr.e_shoff + index as u32 * self.ehdr.e_shentsize as u32,
        )
    }

    pub fn shdr_iter(&self) -> Elf32ShdrIter {
        Elf32ShdrIter {
            index: 0,
            elf32: self,
        }
    }
}

pub struct Elf32PhdrIter<'a> {
    index: u16,
    elf32: &'a Elf32<'a>,
}

impl Iterator for Elf32PhdrIter<'_> {
    type Item = Elf32_Phdr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.elf32.ehdr.e_phnum {
            return None;
        }

        let phdr = self.elf32.phdr_nth(self.index);
        self.index += 1;
        Some(phdr)
    }
}

pub struct Elf32ShdrIter<'a> {
    index: u16,
    elf32: &'a Elf32<'a>,
}

impl Iterator for Elf32ShdrIter<'_> {
    type Item = Elf32_Shdr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.elf32.ehdr.e_shnum {
            return None;
        }

        let shdr = self.elf32.shdr_nth(self.index);
        self.index += 1;
        Some(shdr)
    }
}
