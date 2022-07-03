use crate::{
    error::Result,
    mem::{Chunks, MemExt},
};

const CHUNK_SIZE: usize = 8192;

pub fn find_addr_by_region<T: MemExt + ?Sized>(
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
