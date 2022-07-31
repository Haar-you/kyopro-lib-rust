use rand::distributions::uniform::SampleUniform;
use rand::Rng;

use std::ops::Range;

pub fn rand_range<T, R>(rng: &mut R, range: Range<T>) -> Range<T>
where
    T: SampleUniform + PartialOrd + Clone,
    R: Rng,
{
    let mut start = rng.gen_range(range.clone());
    let mut end = rng.gen_range(range);

    if start > end {
        std::mem::swap(&mut start, &mut end);
    }

    Range { start, end }
}

pub trait RangeIsEmpty {
    fn is_empty(&self) -> bool;
}

impl<T: PartialOrd> RangeIsEmpty for Range<T> {
    fn is_empty(&self) -> bool {
        !(self.start < self.end)
    }
}
