pub use crate::algebra::traits::AbelianGroup;
use std::ops::{Range, RangeTo};

#[derive(Clone, Default)]
pub struct FenwickTree<G: AbelianGroup> {
    data: Vec<G::Element>,
    size: usize,
    group: G,
}

impl<G: AbelianGroup> FenwickTree<G>
where
    G::Element: Clone,
{
    pub fn new(size: usize, group: G) -> Self {
        Self {
            data: vec![group.id(); size + 1],
            size,
            group,
        }
    }

    pub fn update(&mut self, mut i: usize, value: G::Element) {
        i += 1;
        while i <= self.size {
            self.data[i] = self.group.op(self.data[i].clone(), value.clone());
            i += i & (!i + 1);
        }
    }

    pub fn fold_to(&self, RangeTo { end: mut i }: RangeTo<usize>) -> G::Element {
        let mut ret = self.group.id();

        while i > 0 {
            ret = self.group.op(ret.clone(), self.data[i].clone());
            i -= i & (!i + 1);
        }

        ret
    }

    pub fn fold(&self, Range { start: l, end: r }: Range<usize>) -> G::Element {
        self.group
            .op(self.fold_to(..r), self.group.inv(self.fold_to(..l)))
    }
}
