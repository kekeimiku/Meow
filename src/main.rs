use std::{fs::File, os::unix::prelude::FileExt};

use memchr::memmem::find_iter;
use utils::{debug, file::Chunks, info};

fn main() {
    let file = File::open("test.txt").unwrap();
    find_addr(file, 40, 10, "2".as_bytes());
    // let iter = Chunks::from_seek(file, CHUNK_SIZE, 40, 0).unwrap();
    // let chunks = iter.collect::<Result<Vec<_>, _>>().unwrap();

    // let mut offsets = 0;

    // let addr = chunks
    //     .into_iter()
    //     .flat_map(|v| {
    //         let ret = find_iter(&v, "1".as_bytes())
    //             .map(|a| a + offsets)
    //             .collect::<Vec<_>>();
    //         offsets += CHUNK_SIZE;
    //         ret
    //     })
    //     .collect::<Vec<_>>();

    // info!("{:?}", addr)
}

fn find_addr(file: File, mut offsets: usize, size: usize, value: &[u8]) {
    const CHUNK_SIZE: usize = 30;
    let mut addr = Vec::default();
    let mut n = 0;

    let mut buf = vec![0; CHUNK_SIZE];
    for _ in 0..size / CHUNK_SIZE {
        file.read_at(&mut buf, offsets as u64).unwrap();
        let v = find_iter(&buf, value).map(|a| a + n).collect::<Vec<_>>();
        addr.push(v);
        offsets += CHUNK_SIZE;
        n += CHUNK_SIZE;
    }

    if size < CHUNK_SIZE {
        let mut buf = vec![0; size];
        file.read_at(&mut buf, offsets as u64).unwrap();
        let v = find_iter(&buf, value).map(|a| a + n).collect::<Vec<_>>();
        addr.push(v);
    }

    // let m = size % CHUNK_SIZE;

    // if m != 0 {
    //     let mut buf = vec![0; m];
    //     file.read_at(&mut buf, offsets as u64).unwrap();
    //     let v = find_iter(&buf, value).map(|a| a + n).collect::<Vec<_>>();
    //     addr.push(v);
    // }

    let ok = addr.into_iter().flat_map(|v| v).collect::<Vec<_>>();

    debug!("{:?}", ok);
}

fn findnum_addr(file: File, start: usize, size: usize, value: &[u8]) {
    const CHUNK_SIZE: usize = 3;

    let len = value.len();

    let mut addr = Vec::default();
    let mut offsets = start;
    let mut n = 0;
    let num = size / CHUNK_SIZE;
    let mut buf = vec![0; CHUNK_SIZE];
    for _ in 0..num {
        file.read_at(&mut buf, offsets as u64).unwrap();
        let v = buf
            .windows(len)
            .enumerate()
            .step_by(len)
            .filter_map(|(k, v)| if v == value { Some(k + n) } else { None })
            .collect::<Vec<_>>();
        addr.push(v);
        offsets += CHUNK_SIZE;
        n += CHUNK_SIZE;
    }

    let m = size % CHUNK_SIZE;

    if m != 0 {
        let mut buf = vec![0; m];
        file.read_at(&mut buf, offsets as u64).unwrap();
        let v = buf
            .windows(len)
            .enumerate()
            .step_by(len)
            .filter_map(|(k, v)| if v == value { Some(k + n) } else { None })
            .collect::<Vec<_>>();
        addr.push(v);
    }

    let ok = addr.into_iter().flat_map(|v| v).collect::<Vec<_>>();

    debug!("{:?}", ok);
}
