use crate::error::Result;

pub trait MemExt {
    fn read(&self, addr: usize, size: usize) -> Result<Vec<u8>>;
    fn write(&self, addr: usize, payload: &[u8]) -> Result<usize>;
}

#[derive(Debug)]
pub struct Chunks<'a, T: MemExt> {
    mem: &'a T,
    start: usize,
    size: usize,
    num: usize,
    last: usize,
}

impl<'a, T> Chunks<'a, T>
where
    T: MemExt,
{
    pub fn new(mem: &'a T, start: usize, end: usize, mut size: usize) -> Self {
        let mut last = 0;
        let mut num = 1;
        if end < start {
            panic!("seek error")
        }
        if size < end - start {
            num = (end - start) / size;
            last = (end - start) % size;
        } else {
            size = end - start;
        }
        Self {
            mem,
            start,
            size,
            num,
            last,
        }
    }
}

impl<T> Iterator for Chunks<'_, T>
where
    T: MemExt,
{
    type Item = Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num != 0 {
            match self.mem.read(self.start, self.size) {
                Ok(chunk) => {
                    self.start += self.size;
                    self.num -= 1;
                    return Some(Ok(chunk));
                }
                Err(err) => return Some(Err(err)),
            };
        }

        if self.last != 0 {
            match self.mem.read(self.start, self.last) {
                Ok(chunk) => {
                    self.last = 0;
                    return Some(Ok(chunk));
                }
                Err(err) => return Some(Err(err)),
            };
        }

        None
    }
}
