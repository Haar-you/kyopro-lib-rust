pub use crate::ds::traits::{Foldable, Updatable};
use crate::trait_alias;
use crate::traits::one_zero::Zero;
use std::ops::{Add, Range, RangeTo, Sub};

trait_alias!(
    Elem,
    Copy + Zero<Output = Self> + Add<Output = Self> + Sub<Output = Self>
);

pub struct FenwickTreeAdd<T> {
    data: Vec<T>,
    size: usize,
}

impl<T: Elem> FenwickTreeAdd<T> {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![T::zero(); size + 1],
            size,
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

impl<T: Elem> Foldable<RangeTo<usize>> for FenwickTreeAdd<T> {
    type Output = T;

    fn fold(&self, RangeTo { end: mut i }: RangeTo<usize>) -> Self::Output {
        let mut ret = T::zero();

        while i > 0 {
            ret = ret + self.data[i];
            i -= i & (!i + 1);
        }

        ret
    }
}

impl<T: Elem> Foldable<Range<usize>> for FenwickTreeAdd<T> {
    type Output = T;

    fn fold(&self, Range { start: l, end: r }: Range<usize>) -> Self::Output {
        self.fold(..r) - self.fold(..l)
    }
}
