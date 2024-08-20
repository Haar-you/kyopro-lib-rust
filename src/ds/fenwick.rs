pub use crate::algebra::traits::AbelianGroup;
pub use crate::ds::traits::{Foldable, Updatable};
use std::ops::{Range, RangeTo};

#[derive(Clone, Default)]
pub struct FenwickTree<G: AbelianGroup> {
    data: Vec<G::Output>,
    size: usize,
    group: G,
}

impl<T: Clone, G: AbelianGroup<Output = T>> FenwickTree<G> {
    pub fn new(size: usize, group: G) -> Self {
        Self {
            data: vec![group.id(); size + 1],
            size,
            group,
        }
    }
}

impl<T: Clone, G: AbelianGroup<Output = T>> Updatable<usize> for FenwickTree<G> {
    type Value = T;

    fn update(&mut self, mut i: usize, value: T) {
        i += 1;
        while i <= self.size {
            self.data[i] = self.group.op(self.data[i].clone(), value.clone());
            i += i & (!i + 1);
        }
    }
}

impl<T: Clone, G: AbelianGroup<Output = T>> Foldable<RangeTo<usize>> for FenwickTree<G> {
    type Output = T;

    fn fold(&self, RangeTo { end: mut i }: RangeTo<usize>) -> Self::Output {
        let mut ret = self.group.id();

        while i > 0 {
            ret = self.group.op(ret.clone(), self.data[i].clone());
            i -= i & (!i + 1);
        }

        ret
    }
}

impl<T: Clone, G: AbelianGroup<Output = T>> Foldable<Range<usize>> for FenwickTree<G> {
    type Output = T;

    fn fold(&self, Range { start: l, end: r }: Range<usize>) -> Self::Output {
        self.group
            .op(self.fold(..r), self.group.inv(self.fold(..l)))
    }
}
