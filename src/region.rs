use std::{mem, ops::Range};

use crate::maps::MapRange;


// 一个比较省内存的数据结构，感谢 Lonami
#[derive(Debug)]
pub enum CandidateLocations {
    Discrete { locations: Vec<usize> },
    SmallDiscrete { base: usize, offsets: Vec<u16> },
    Dense { range: Range<usize>, step: usize },
    Sparse { base: usize, mask: Vec<bool>, scale: usize },
}

impl CandidateLocations {
    pub fn len(&self) -> usize {
        match self {
            CandidateLocations::Discrete { locations } => locations.len(),
            CandidateLocations::SmallDiscrete { offsets, .. } => offsets.len(),
            CandidateLocations::Dense { range, step } => range.len() / step,
            CandidateLocations::Sparse { mask, .. } => mask.iter().filter(|x| **x).count(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn try_compact(&mut self, value_size: usize) {
        let locations = match self {
            CandidateLocations::Discrete { locations } if locations.len() >= 2 => mem::take(locations),
            _ => return,
        };

        let low = *locations.first().unwrap();
        let high = *locations.last().unwrap();
        let size = high - low;
        let size_for_aligned = size / value_size;

        if size <= u16::MAX as _ && locations.len() * mem::size_of::<u16>() < size_for_aligned {
            *self = CandidateLocations::SmallDiscrete {
                base: low,
                offsets: locations
                    .into_iter()
                    .map(|loc| (loc - low).try_into().unwrap())
                    .collect(),
            };
            return;
        }

        if size_for_aligned < locations.len() * mem::size_of::<usize>() {
            assert_eq!(low % value_size, 0);

            let mut locations = locations.into_iter();
            let mut next_set = locations.next();
            *self = CandidateLocations::Sparse {
                base: low,
                mask: (low..high)
                    .step_by(value_size)
                    .map(|addr| {
                        if Some(addr) == next_set {
                            next_set = locations.next();
                            true
                        } else {
                            false
                        }
                    })
                    .collect(),
                scale: value_size,
            };
            return;
        }

        *self = CandidateLocations::Discrete { locations };
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = usize> + 'a> {
        match self {
            CandidateLocations::Discrete { locations } => Box::new(locations.iter().copied()),
            CandidateLocations::SmallDiscrete { base, offsets } => {
                Box::new(offsets.iter().map(move |&offset| base + offset as usize))
            }
            CandidateLocations::Dense { range, step } => Box::new(range.clone().step_by(*step)),
            CandidateLocations::Sparse { base, mask, scale } => Box::new(
                mask.iter()
                    .enumerate()
                    .filter(|(_, &set)| set)
                    .map(move |(i, _)| base + i * scale),
            ),
        }
    }
}

#[derive(Debug)]
pub struct Region {
    pub info: MapRange,
    pub locations: CandidateLocations,
    pub value: Value,
}

#[derive(Debug)]
pub enum Value {
    Exact(Vec<u8>),
    AnyWithin { memory: Vec<u8>, size: usize },
}
