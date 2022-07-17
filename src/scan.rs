use utils::info;

use crate::{
    error::Result,
    mem::{Chunks, MemExt},
    region::InfoExt,
};

#[derive(Debug)]
pub struct RegionChunkData<'a, R: InfoExt> {
    pub info: &'a R,
    pub local: Vec<Chunk>,
}

type Chunk = Vec<u16>;

pub struct Scan<'a, H: MemExt, R: InfoExt> {
    handle: &'a H,
    region: &'a [R],
    pub data: Vec<RegionChunkData<'a, R>>,
}

impl<'a, H, R> Scan<'a, H, R>
where
    H: MemExt,
    R: InfoExt,
{
    pub fn new(handle: &'a H, region: &'a [R]) -> Result<Scan<'a, H, R>> {
        Ok(Scan {
            handle,
            region,
            data: Vec::new(),
        })
    }

    pub fn find(&mut self, value: &[u8]) -> Result<()> {
        let mut v = Vec::new();

        self.region.iter().for_each(|region| {
            let local: Vec<_> = scan_region(self.handle, region.start(), region.end(), value).collect();
            let data = RegionChunkData { info: region, local };
            v.push(data);
        });

        self.data = v;

        Ok(())
    }

    pub fn refind(&mut self, value: &[u8]) -> Result<()> {
        self.data.iter_mut().for_each(|d| {
            rescan_region(self.handle, d.info.start(), d.info.end(), value, &mut d.local).unwrap();
        });
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.data
            .iter()
            .map(|v| v.local.iter().map(|n| n.len()).sum::<usize>())
            .sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // TODO 不要全部列出，可选列出指定数量，类似分页的效果
    pub fn list(&self) -> Result<()> {
        self.data.iter().for_each(|vec| {
            let mut num = 0;
            let new = vec.local.iter().fold(Vec::default(), |mut init, next| {
                init.extend(
                    next.iter()
                        .map(|v| *v as usize + num + vec.info.start())
                        .collect::<Vec<_>>(),
                );
                num += CHUNK_SIZE;
                init
            });
            new.iter().for_each(|v| {
                info!("0x{:x}", v);
            });
        });

        Ok(())
    }
}

// 这使扫描和储存结果的内存占用很低
// TODO 如果我们需要储存扫描结果到硬盘，我们将允许它超过 u16::max ，例如设置为8mb。
// 这将在不占用过多内存的情况下加快扫描速度 减少syscall次数
// TODO 储存到硬盘的格式？如何读取？顺序？
// TODO 也许储存在内存还是硬盘应该可选
const CHUNK_SIZE: usize = 63488;

fn scan_region<'a, T: MemExt>(
    handle: &'a T,
    start: usize,
    end: usize,
    value: &'a [u8],
) -> impl Iterator<Item = Vec<u16>> + 'a {
    Chunks::new(handle, start, end, CHUNK_SIZE)
        .into_iter()
        .map(move |v| {
            // todo 处理这个错误，它会导致内存泄漏....
            v.unwrap()
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
                .collect()
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

// #[cfg(test)]
// mod tests {

//     #[cfg(any(target_os = "linux", target_os = "android"))]
//     #[test]
//     fn test_find_addr_by_region_linux() {
//         use crate::{
//             mem::MemExt,
//             platform::{Mem, Region},
//             region::InfoExt,
//             scan::Scan,
//         };

//         let mem = Mem::new(tempfile::tempfile().unwrap());
//         mem.write(0, &[49, 49, 50, 50, 51, 51, 52, 52, 51, 51, 53, 53])
//             .unwrap();
//         let region = Region {
//             range_start: 2,
//             range_end: 10,
//             flags: 'x'.into(),
//             pathname: "".into(),
//         };

//         let mut scan = Scan::new(&mem, &region).unwrap();
//         scan.find(&[51, 51]).unwrap();
//         assert_eq!(scan.list(region.start()).unwrap(), vec![4, 8]);
//         mem.write(4, &[50, 50]).unwrap();
//         scan.refind(&[50, 50]).unwrap();
//         assert_eq!(scan.list(region.start()).unwrap(), vec![4]);
//     }

//     #[test]
//     fn test_find_addr_by_region_windows() {
//         // TODO
//     }
// }
