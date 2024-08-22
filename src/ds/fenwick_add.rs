use crate::num::one_zero::Zero;
use crate::trait_alias;
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

    pub fn fold_to(&self, RangeTo { end: mut i }: RangeTo<usize>) -> T {
        let mut ret = T::zero();

        while i > 0 {
            ret = ret + self.data[i];
            i -= i & (!i + 1);
        }

        ret
    }

    pub fn fold(&self, Range { start: l, end: r }: Range<usize>) -> T {
        self.fold_to(..r) - self.fold_to(..l)
    }
}
