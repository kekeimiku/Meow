// 分块读取文件

use std::io::{self, Read, Seek, SeekFrom};

pub struct Chunks<R> {
    read: R,
    size: usize,
    hint: (usize, Option<usize>),
}

impl<R> Chunks<R> {
    pub fn new(read: R, size: usize) -> Self {
        Self {
            read,
            size,
            hint: (0, None),
        }
    }

    pub fn from_seek(mut read: R, size: usize, start: usize, end: usize) -> io::Result<Self>
    where
        R: Seek,
    {
        let old_pos = read.seek(SeekFrom::Current(start as i64))?;
        let len = read.seek(SeekFrom::End(end as i64))?;
        
        let rest = (len - old_pos) as usize; // len is always >= old_pos but they are u64
        if rest != 0 {
            read.seek(SeekFrom::Start(old_pos))?;
        }

        let min = rest / size + if rest % size != 0 { 1 } else { 0 };
        Ok(Self {
            read,
            size,
            hint: (min, None), // this could be wrong I'm unsure
        })
    }
}

impl<R> Iterator for Chunks<R>
where
    R: Read,
{
    type Item = io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chunk = Vec::with_capacity(self.size);
        match self
            .read
            .by_ref()
            .take(chunk.capacity() as u64)
            .read_to_end(&mut chunk)
        {
            Ok(n) => {
                if n != 0 {
                    Some(Ok(chunk))
                } else {
                    None
                }
            }
            Err(e) => Some(Err(e)),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.hint
    }
}

trait ReadPlus: Read {
    fn chunks(self, size: usize) -> Chunks<Self>
    where
        Self: Sized,
    {
        Chunks::new(self, size)
    }
}

impl<T: ?Sized> ReadPlus for T where T: Read {}
