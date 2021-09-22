pub use crate::algebra::traits::Group;
pub use crate::ds::traits::{Foldable, Updatable};
use std::ops::{Range, RangeTo};

pub struct FenwickTree<T, G> {
    data: Vec<T>,
    size: usize,
    group: G,
}

impl<T: Clone, G: Group<Output = T>> FenwickTree<T, G> {
    pub fn new(size: usize, group: G) -> Self {
        Self {
            data: vec![group.id(); size + 1],
            size,
            group,
        }
    }
}

impl<T: Clone, G: Group<Output = T>> Updatable<usize> for FenwickTree<T, G> {
    type Value = T;

    fn update(&mut self, mut i: usize, value: T) {
        i += 1;
        while i <= self.size {
            self.data[i] = self.group.op(self.data[i].clone(), value.clone());
            i += (i as isize & (-(i as isize))) as usize;
        }
    }
}

impl<T: Clone, G: Group<Output = T>> Foldable<RangeTo<usize>> for FenwickTree<T, G> {
    type Value = T;

    fn fold(&self, RangeTo { end: mut i }: RangeTo<usize>) -> Self::Value {
        let mut ret = self.group.id();

        while i > 0 {
            ret = self.group.op(ret.clone(), self.data[i].clone());
            i -= ((i as isize) & (-(i as isize))) as usize;
        }

        ret
    }
}

impl<T: Clone, G: Group<Output = T>> Foldable<Range<usize>> for FenwickTree<T, G> {
    type Value = T;

    fn fold(&self, Range { start: l, end: r }: Range<usize>) -> Self::Value {
        self.group.op(self.fold(..r), self.group.inv(self.fold(..l)))
    }
}
