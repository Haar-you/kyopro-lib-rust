//! 可換群の点更新・区間取得($O(\log n)$, $O(\log n)$)ができる。
pub use crate::algebra::traits::AbelianGroup;
use std::ops::{Range, RangeTo};

/// 可換群の点更新・区間取得($O(\log n)$, $O(\log n)$)ができる。
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
    /// 長さ`size`、可換群`group`から[`FenwickTree<G>`]を生成する。
    pub fn new(size: usize, group: G) -> Self {
        Self {
            data: vec![group.id(); size + 1],
            size,
            group,
        }
    }

    /// `i`番目の要素を`value`で更新する。
    ///
    /// **Time complexity** $O(\log n)$
    pub fn update(&mut self, mut i: usize, value: G::Element) {
        i += 1;
        while i <= self.size {
            self.data[i] = self.group.op(self.data[i].clone(), value.clone());
            i += i & (!i + 1);
        }
    }

    /// 範囲`0..r`で計算を集約した結果を返す。
    ///
    /// **Time complexity** $O(\log n)$
    pub fn fold_to(&self, RangeTo { end: mut i }: RangeTo<usize>) -> G::Element {
        let mut ret = self.group.id();

        while i > 0 {
            ret = self.group.op(ret.clone(), self.data[i].clone());
            i -= i & (!i + 1);
        }

        ret
    }

    /// 範囲`l..r`で計算を集約した結果を返す。
    ///
    /// **Time complexity** $O(\log n)$
    pub fn fold(&self, Range { start: l, end: r }: Range<usize>) -> G::Element {
        self.group
            .op(self.fold_to(..r), self.group.inv(self.fold_to(..l)))
    }
}
