pub use crate::ds::traits::{Foldable, Updatable};
use std::ops::{Add, Range, RangeTo, Sub};

pub struct FenwickTreeAdd<T> {
    data: Vec<T>,
    size: usize,
    zero: T,
}

impl<T: Copy + Add<Output = T> + Sub<Output = T>> FenwickTreeAdd<T> {
    pub fn new(size: usize, zero: T) -> Self {
        Self {
            data: vec![zero; size + 1],
            size,
            zero,
        }
    }

    pub fn sub(&mut self, mut i: usize, value: T) {
        i += 1;
        while i <= self.size {
            self.data[i] = self.data[i] - value;
            i += i & (!i + 1);
        }
    }

    pub fn add(&mut self, mut i: usize, value: T) {
        i += 1;
        while i <= self.size {
            self.data[i] = self.data[i] + value;
            i += i & (!i + 1);
        }
    }
}

impl<T: Copy + Add<Output = T>> Foldable<RangeTo<usize>> for FenwickTreeAdd<T> {
    type Output = T;

    fn fold(&self, RangeTo { end: mut i }: RangeTo<usize>) -> Self::Output {
        let mut ret = self.zero;

        while i > 0 {
            ret = ret + self.data[i];
            i -= i & (!i + 1);
        }

        ret
    }
}

impl<T: Copy + Add<Output = T> + Sub<Output = T>> Foldable<Range<usize>> for FenwickTreeAdd<T> {
    type Output = T;

    fn fold(&self, Range { start: l, end: r }: Range<usize>) -> Self::Output {
        self.fold(..r) - self.fold(..l)
    }
}
