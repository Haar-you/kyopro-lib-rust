//! 可換群の点更新・区間取得($O(\log n)$, $O(\log n)$)ができる。
pub use crate::algebra::traits::AbelianGroup;
use std::ops::{Range, RangeTo};

/// 可換群の点更新・区間取得($O(\log n)$, $O(\log n)$)ができる。
#[derive(Clone, Default)]
pub struct FenwickTree<G: AbelianGroup> {
    data: Vec<G>,
    size: usize,
}

impl<G: AbelianGroup + Clone> FenwickTree<G> {
    /// 長さ`size`、可換群`group`から[`FenwickTree<G>`]を生成する。
    pub fn new(size: usize) -> Self {
        let data = vec![G::id(); size + 1];
        Self { data, size }
    }

    /// `i`番目の要素を`value`で更新する。
    ///
    /// **Time complexity** $O(\log n)$
    pub fn update(&mut self, mut i: usize, value: G) {
        i += 1;
        while i <= self.size {
            self.data[i] = G::op(self.data[i].clone(), value.clone());
            i += i & (!i + 1);
        }
    }

    /// 範囲`0..r`で計算を集約した結果を返す。
    ///
    /// **Time complexity** $O(\log n)$
    pub fn fold_to(&self, RangeTo { end: mut i }: RangeTo<usize>) -> G {
        let mut ret = G::id();

        while i > 0 {
            ret = G::op(ret.clone(), self.data[i].clone());
            i -= i & (!i + 1);
        }

        ret
    }

    /// 範囲`l..r`で計算を集約した結果を返す。
    ///
    /// **Time complexity** $O(\log n)$
    pub fn fold(&self, Range { start: l, end: r }: Range<usize>) -> G {
        G::op(self.fold_to(..r), self.fold_to(..l).inv())
    }
}
