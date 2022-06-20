use crate::error::Result;

#[cfg(feature = "memmem")]
use memchr::memmem::find_iter;

use std::{fs::File, os::unix::prelude::FileExt};
use utils::debug;

// 文件分块的大小 默认4mb，不要瞎鸡巴动它
const CHUNK_SIZE: usize = 4096;

// 对齐搜索，用于搜索数字，足够应对大部分情况
macro_rules! find_num_addr {
    ($buf:expr,$value:expr,$len:expr,$num:expr,$tmp:expr,$eq:tt) => {
        let vec = $buf
            .windows($len)
            .enumerate()
            .step_by($len)
            .filter_map(|(k, v)| if v $eq $value { Some(k + $num) } else { None })
            .collect::<Vec<_>>();
        $tmp.push(vec);
    };
}

// 非对齐搜索，基本只有需要搜索字符串的时候才用
#[cfg(feature = "memmem")]
macro_rules! find_str_addr {
    ($buf:expr,$value:expr,$tmp:expr) => {
        let vec = find_iter(&$buf, $value).collect::<Vec<usize>>();
        $tmp.push(vec);
    };
}

// 在一个分块查找一个内存区域中值为value的地址
// file:文件句柄, start:开始区域, end:结束区域, value:目标值,
// flag:是否开启内存对齐搜索，默认true
pub fn find_region_addr(
    file: &File,
    mut start: usize,
    end: usize,
    value: &[u8],
    flag: bool,
) -> Result<Vec<usize>> {
    let mut tmp = Vec::default();

    let len = value.len();
    let mut num = 0;

    let size = end - start;
    if CHUNK_SIZE >= size {
        let mut buf = vec![0; size];
        file.read_at(&mut buf, start as u64)?;
        debug!("CHUNK_SIZE >= size");
        if flag {
            find_num_addr!(buf,value,len,num,tmp,==);
        } else {
            #[cfg(feature = "memmem")]
            find_str_addr!(buf, value, tmp);
        }

        return Ok(tmp.into_iter().flatten().collect::<Vec<_>>());
    }

    let mut buf = vec![0; CHUNK_SIZE];

    for _ in 0..(end - start) / CHUNK_SIZE {
        file.read_at(&mut buf, start as u64)?;

        if flag {
            find_num_addr!(buf,value,len,num,tmp,==);
        } else {
            #[cfg(feature = "memmem")]
            find_str_addr!(buf, value, tmp);
        }

        start += CHUNK_SIZE;
        num += CHUNK_SIZE;
        // debug!("0..(end - start) / CHUNK_SIZE");
    }

    let size = (end - start) % CHUNK_SIZE;
    if size != 0 {
        let mut buf = vec![0; size];
        file.read_at(&mut buf, start as u64)?;
        if flag {
            find_num_addr!(buf,value,len,num,tmp,==);
        } else {
            #[cfg(feature = "memmem")]
            find_str_addr!(buf, value, tmp);
        }
        debug!("(end - start) % CHUNK_SIZE");
    }
    Ok(tmp.into_iter().flatten().collect::<Vec<_>>())
}
