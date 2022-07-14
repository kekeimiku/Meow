use utils::{debug, info};

use crate::{
    error::Result,
    mem::{Chunks, MemExt},
    region::RegionExt,
};

pub struct Scan<'a, H: MemExt, R: RegionExt> {
    handle: &'a H,
    region: &'a R,
    tmp: Vec<Vec<u16>>,
}

impl<'a, H, R> Scan<'_, H, R>
where
    H: MemExt,
    R: RegionExt,
{
    pub fn new(handle: &'a H, region: &'a R) -> Result<Scan<'a, H, R>> {
        Ok(Scan {
            handle,
            region,
            tmp: Vec::new(),
        })
    }

    pub fn find(&mut self, value: &[u8]) -> Result<()> {
        let region = self.region;
        self.tmp = scan_region(self.handle, region.start(), region.end(), value)?;
        Ok(())
    }

    pub fn refind(&mut self, value: &[u8]) -> Result<()> {
        let region = self.region;
        rescan_region(self.handle, region.start(), region.end(), value, &mut self.tmp)?;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.tmp.iter().map(|v| v.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // TODO 不要全部列出，可选列出指定数量，类似分页的效果
    pub fn list(&self, offset: usize) -> Result<Vec<usize>> {
        debug!("{:?}", self.tmp);
        let mut num = 0;
        let new = self.tmp.iter().fold(Vec::default(), |mut init, next| {
            init.extend(
                next.iter()
                    .map(|v| *v as usize + num + offset)
                    .collect::<Vec<_>>(),
            );
            num += CHUNK_SIZE;
            init
        });
        new.iter().for_each(|v| {
            info!("0x{:x}", v);
        });

        Ok(new)
    }
}

const CHUNK_SIZE: usize = 63488;

fn scan_region<T: MemExt>(handle: &T, start: usize, end: usize, value: &[u8]) -> Result<Vec<Vec<u16>>> {
    Chunks::new(handle, start, end, CHUNK_SIZE)
        .into_iter()
        .try_fold(Vec::default(), |mut init, next| {
            init.push(
                next?
                    .windows(value.len())
                    .enumerate()
                    .step_by(value.len())
                    .filter_map(|(k, v)| {
                        if v == value {
                            Some(k.try_into().unwrap())
                        } else {
                            None
                        }
                    })
                    .collect(),
            );
            Ok(init)
        })
}

// TODO refactor
fn rescan_region<T: MemExt>(
    handle: &T,
    start: usize,
    end: usize,
    value: &[u8],
    vec: &mut [Vec<u16>],
) -> Result<()> {
    let mem = Chunks::new(handle, start, end, CHUNK_SIZE);
    for (mem, k1) in mem.into_iter().zip(0..vec.len()) {
        for k2 in (0..vec[k1].len()).rev() {
            let start = vec[k1][k2] as usize;
            let end = start + value.len();
            let data = &mem.as_ref().unwrap()[start..end];
            if data != value {
                vec[k1].swap_remove(k2);
            }
        }
        vec[k1].shrink_to_fit();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::platform::Mem;

    use super::MemExt;

    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    fn test_find_addr_by_region_linux() {
        use crate::{platform::Region, region::RegionExt};

        use super::Scan;

        let mem = Mem::new(tempfile::tempfile().unwrap());
        mem.write(0, &[49, 49, 50, 50, 51, 51, 52, 52, 51, 51, 53, 53])
            .unwrap();
        let region = Region {
            range_start: 2,
            range_end: 10,
            flags: 'x'.into(),
            pathname: "".into(),
        };

        let mut scan = Scan::new(&mem, &region).unwrap();
        scan.find(&[51, 51]).unwrap();
        assert_eq!(scan.list(region.start()).unwrap(), vec![4, 8]);
        mem.write(4, &[50, 50]).unwrap();
        scan.refind(&[50, 50]).unwrap();
        assert_eq!(scan.list(region.start()).unwrap(), vec![4]);
    }

    #[test]
    fn test_find_addr_by_region_windows() {}
}
