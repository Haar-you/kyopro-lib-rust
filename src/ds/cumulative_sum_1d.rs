//! 1次元累積和

pub use crate::algebra::traits::Group;
use std::ops::{Index, Range};

/// 1次元の累積和を扱う
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct CumulativeSum1D<G: Group> {
    group: G,
    data: Vec<G::Element>,
}

/// [`CumulativeSum1D`]を構築する
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CumulativeSum1DBuilder<G: Group> {
    group: G,
    data: Vec<G::Element>,
}

impl<G: Group> CumulativeSum1D<G>
where
    G::Element: Copy,
{
    /// **Time complexity** $O(1)$
    pub fn fold(&self, Range { start: l, end: r }: Range<usize>) -> G::Element {
        self.group.op(self.data[r], self.group.inv(self.data[l]))
    }
}

impl<G: Group> Index<usize> for CumulativeSum1D<G> {
    type Output = G::Element;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl<G: Group> CumulativeSum1DBuilder<G>
where
    G::Element: Copy,
{
    /// `CumulativeSum1DBuilder`を生成する
    pub fn new(group: G, n: usize) -> Self {
        Self {
            data: vec![group.id(); n + 1],
            group,
        }
    }

    /// `i`番目に`value`を代入する
    pub fn assign(&mut self, i: usize, value: G::Element) {
        self.data[i + 1] = value;
    }

    /// 群`G`の演算に`i`番目の値と`value`を適用して`i`番目の値を更新する。
    pub fn update(&mut self, i: usize, value: G::Element) {
        self.data[i + 1] = self.group.op(self.data[i + 1], value);
    }

    /// [`CumulativeSum1D`]を構築する
    pub fn build(self) -> CumulativeSum1D<G> {
        let group = self.group;
        let data = self
            .data
            .iter()
            .scan(group.id(), |st, &x| {
                *st = group.op(*st, x);
                Some(*st)
            })
            .collect::<Vec<_>>();

        CumulativeSum1D { group, data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::sum::*;
    use my_testtools::*;
    use rand::Rng;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let n = 20;
        let g = Sum::<i32>::new();
        let mut csb = CumulativeSum1DBuilder::new(g, n);

        let mut other = vec![g.id(); n];

        for _ in 0..1000 {
            let i = rng.gen_range(0..n);
            let x = rng.gen_range(-1000..=1000);

            csb.update(i, x);
            g.op_assign_r(&mut other[i], x);
        }

        let cs = csb.build();

        for _ in 0..100 {
            let range = rand_range(&mut rng, 0..n);
            let ans = other[range.clone()].iter().cloned().fold_m(&g);
            assert_eq!(cs.fold(range), ans);
        }
    }
}
