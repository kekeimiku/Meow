use crate::{
    error::Result,
    mem::{Chunks, MemExt},
    region::RegionExt,
};

pub struct Scan<'a, T: MemExt, R: RegionExt> {
    handle: &'a T,
    region: &'a R,
}

impl<'a, T, R> Scan<'_, T, R>
where
    T: MemExt,
    R: RegionExt,
{
    pub fn from(handle: &'a T, region: &'a R) -> Result<Scan<'a, T, R>> {
        Ok(Scan { handle, region })
    }

    pub fn find(&self, value: &[u8]) -> Result<Vec<usize>> {
        let region = self.region;
        find_addr_by_region(self.handle, region.start(), region.end(), value)
    }
}


// TODO refactor 
// 或许应该返回一个 Vec<Vec<usize>> 并且不加上 CHUNK_SIZE ，以便根据地址下一次分块读取内存搜索? 
// 计算实际地址的时候再加上 CHUNK_SIZE 和 region.start
const CHUNK_SIZE: usize = 8192;

pub fn find_addr_by_region<T: MemExt>(
    handle: &T,
    start: usize,
    end: usize,
    value: &[u8],
) -> Result<Vec<usize>> {
    let mut num = 0;
    Chunks::new(handle, start, end, CHUNK_SIZE)
        .into_iter()
        .try_fold(Vec::default(), |mut init, next| {
            init.extend(
                next?
                    .windows(value.len())
                    .enumerate()
                    .step_by(value.len())
                    .filter_map(|(k, v)| if v == value { Some(k + num) } else { None })
                    .collect::<Vec<_>>(),
            );
            num += CHUNK_SIZE;
            Ok(init)
        })
}
