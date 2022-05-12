use std::cmp::Ordering;
use std::iter::Peekable;

pub struct SortedDiff<T, U: Iterator> {
    pub it1: T,
    pub it2: Peekable<U>,
}

impl<T, U, W> Iterator for SortedDiff<T, U>
where
    T: Iterator<Item = W>,
    U: Iterator<Item = W>,
    W: Ord,
{
    type Item = W;

    fn next(&mut self) -> Option<Self::Item> {
        for elm1 in self.it1.by_ref() {
            'inner: loop {
                match self.it2.peek().map(|elm2| elm1.cmp(elm2)) {
                    None => return Some(elm1),
                    Some(Ordering::Less) => return Some(elm1),
                    Some(Ordering::Equal) => break 'inner,
                    Some(Ordering::Greater) => {
                        self.it2.next();
                    }
                }
            }
        }

        None
    }
}

pub fn sorted_difference<T, U>(it1: T, it2: U) -> SortedDiff<T, U>
where
    T: Iterator,
    U: Iterator<Item = T::Item>,
    T::Item: Ord,
{
    SortedDiff {
        it1,
        it2: it2.peekable(),
    }
}

pub fn x1(mut i1: Vec<usize>, mut i2: Vec<usize>) -> Vec<usize> {
    i1.sort_unstable();
    i2.sort_unstable();
    sorted_difference(i2.iter(), i1.iter())
        .copied()
        .collect::<Vec<usize>>()
}
