// 尽量减少 Vec<usize> 的内存占用，这只损失了一点性能，但是在最佳情况下可以减少一倍的内存占用
#[derive(Debug)]
pub enum VecMinValue {
    Orig { vec: Vec<usize> },
    SmallOffset { base: usize, offsets: Vec<u16> },
    BigOffset { base: usize, offsets: Vec<u32> },
    Small { vec: Vec<u16> },
    Big { vec: Vec<u32> },
}

impl Default for VecMinValue {
    fn default() -> Self {
        Self::Orig { vec: Vec::default() }
    }
}

impl VecMinValue {
    #[inline(always)]
    pub fn compact(&mut self) {
        let vec = match self {
            VecMinValue::Orig { vec } if vec.len() >= 2 => core::mem::take(vec),
            _ => return,
        };

        let low = *vec.first().unwrap();
        let high = *vec.last().unwrap();
        let size = high - low;

        // 判断是否可以把大于 u16max 的地址以base + offset Vec<u16> 储存
        if size <= u16::MAX as _ && high >= u16::MAX as _ {
            *self = VecMinValue::SmallOffset {
                base: low,
                offsets: vec
                    .into_iter()
                    .map(|loc| (loc - low).try_into().unwrap())
                    .collect(),
            };
            return;
        }

        // 判断是否可以把小于 u16max 的地址以Vec<u16>储存
        if high <= u16::MAX as _ {
            *self = VecMinValue::Small {
                vec: vec.iter().map(|&v| v.try_into().unwrap()).collect(),
            };
            return;
        }

        // 判断是否可以把大于 u32max 的地址以base + offset Vec<u32> 储存
        if size <= u32::MAX as _ && high >= u32::MAX as _ {
            *self = VecMinValue::BigOffset {
                base: low,
                offsets: vec
                    .into_iter()
                    .map(|loc| (loc - low).try_into().unwrap())
                    .collect(),
            };
            return;
        }

        // 判断是否可以把小于 u32max 的地址以Vec<u32>储存
        if high <= u32::MAX as _ {
            *self = VecMinValue::Big {
                vec: vec.iter().map(|&v| v.try_into().unwrap()).collect(),
            };
            return;
        }

        // 以上都不行，原样储存 Vec<usize>
        *self = VecMinValue::Orig { vec }
    }

    // 获取MineVecValue的长度
    #[inline(always)]
    pub fn len(&self) -> usize {
        match self {
            VecMinValue::Orig { vec } => vec.len(),
            VecMinValue::SmallOffset { offsets, .. } => offsets.len(),
            VecMinValue::BigOffset { offsets, .. } => offsets.len(),
            VecMinValue::Small { vec } => vec.len(),
            VecMinValue::Big { vec } => vec.len(),
        }
    }

    #[inline(always)]
    pub fn remove(&mut self, index: usize) {
        match self {
            VecMinValue::Orig { vec } => vec.remove(index),
            VecMinValue::SmallOffset { offsets, .. } => offsets.remove(index).try_into().unwrap(),
            VecMinValue::BigOffset { offsets, .. } => offsets.remove(index).try_into().unwrap(),
            VecMinValue::Small { vec } => vec.remove(index).try_into().unwrap(),
            VecMinValue::Big { vec } => vec.remove(index).try_into().unwrap(),
        };
    }

    #[inline(always)]
    pub fn swap_remove(&mut self, index: usize) {
        match self {
            VecMinValue::Orig { vec } => vec.swap_remove(index),
            VecMinValue::SmallOffset { offsets, .. } => offsets.swap_remove(index).try_into().unwrap(),
            VecMinValue::BigOffset { offsets, .. } => offsets.swap_remove(index).try_into().unwrap(),
            VecMinValue::Small { vec } => vec.swap_remove(index).try_into().unwrap(),
            VecMinValue::Big { vec } => vec.swap_remove(index).try_into().unwrap(),
        };
    }

    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<usize> {
        match self {
            VecMinValue::Orig { vec } => Some(*vec.get(index)?),
            VecMinValue::SmallOffset { base, offsets } => Some(*offsets.get(index)? as usize + base),
            VecMinValue::BigOffset { base, offsets } => Some(*offsets.get(index)? as usize + base),
            VecMinValue::Small { vec } => Some(*vec.get(index)? as usize),
            VecMinValue::Big { vec } => Some(*vec.get(index)? as usize),
        }
    }

    // MineVecValue 为空
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    pub fn shrink_to_fit(&mut self) {
        match self {
            VecMinValue::Orig { vec } => vec.shrink_to_fit(),
            VecMinValue::SmallOffset { offsets, .. } => offsets.shrink_to_fit(),
            VecMinValue::BigOffset { offsets, .. } => offsets.shrink_to_fit(),
            VecMinValue::Small { vec } => vec.shrink_to_fit(),
            VecMinValue::Big { vec } => vec.shrink_to_fit(),
        }
    }

    // 删除一些元素
    #[inline(always)]
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&usize) -> bool,
    {
        match self {
            VecMinValue::Orig { vec } => {
                vec.retain_mut(|elem| f(elem));
            }
            VecMinValue::SmallOffset { base, offsets } => {
                offsets.retain_mut(|elem| f(&(*elem as usize + *base)));
            }
            VecMinValue::BigOffset { base, offsets } => {
                offsets.retain_mut(|elem| f(&(*elem as usize + *base)));
            }
            VecMinValue::Small { vec } => {
                vec.retain_mut(|elem| f(&(*elem as usize)));
            }
            VecMinValue::Big { vec } => {
                vec.retain_mut(|elem| f(&(*elem as usize)));
            }
        }
    }

    // 返回计算usize的迭代器 Vec<usize>
    #[inline(always)]
    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = usize> + 'a> {
        match self {
            VecMinValue::Orig { vec } => Box::new(vec.iter().copied()),
            VecMinValue::SmallOffset { base, offsets } => {
                Box::new(offsets.iter().map(move |&offset| base + offset as usize))
            }
            VecMinValue::BigOffset { base, offsets } => {
                Box::new(offsets.iter().map(move |&offset| base + offset as usize))
            }
            VecMinValue::Small { vec } => Box::new(vec.iter().map(move |&v| v as usize)),
            VecMinValue::Big { vec } => Box::new(vec.iter().map(move |&v| v as usize)),
        }
    }
}
