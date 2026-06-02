use rand::distr::uniform::SampleUniform;
use rand::RngExt;

use std::ops::Range;

pub fn rand_range<T, R>(rng: &mut R, range: Range<T>) -> Range<T>
where
    T: SampleUniform + PartialOrd + Clone,
    R: RngExt,
{
    let mut start = rng.random_range(range.clone());
    let mut end = rng.random_range(range);

    if start > end {
        std::mem::swap(&mut start, &mut end);
    }

    Range { start, end }
}
