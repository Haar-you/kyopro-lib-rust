//! 可換な加減算に特化したFenwickTree
use crate::num::one_zero::Zero;
use crate::trait_alias;
use std::ops::{Add, Range, RangeTo, Sub};

trait_alias!(
    /// [`FenwickTreeAdd<T>`]が扱える型
    Elem: Copy + Zero + Add<Output = Self> + Sub<Output = Self>
);

/// 可換な加減算に特化したFenwickTree
pub struct FenwickTreeAdd<T> {
    data: Vec<T>,
    size: usize,
}

impl<T: Elem> FenwickTreeAdd<T> {
    /// 長さ`size`の[`FenwickTreeAdd<T>`]を生成する。
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![T::zero(); size + 1],
            size,
        }
    }

    /// `i`番目の値から`value`を引く。
    ///
    /// **Time complexity** $O(\log n)$
    pub fn sub(&mut self, mut i: usize, value: T) {
        i += 1;
        while i <= self.size {
            self.data[i] = self.data[i] - value;
            i += i & (!i + 1);
        }
    }

    /// `i`番目の値に`value`を足す。
    ///
    /// **Time complexity** $O(\log n)$
    pub fn add(&mut self, mut i: usize, value: T) {
        i += 1;
        while i <= self.size {
            self.data[i] = self.data[i] + value;
            i += i & (!i + 1);
        }
    }

    /// 範囲`0..r`の総和を返す。
    ///
    /// **Time complexity** $O(\log n)$
    pub fn fold_to(&self, RangeTo { end: mut i }: RangeTo<usize>) -> T {
        let mut ret = T::zero();

        while i > 0 {
            ret = ret + self.data[i];
            i -= i & (!i + 1);
        }

        ret
    }

    /// 範囲`l..r`の総和を返す。
    ///
    /// **Time complexity** $O(\log n)$
    pub fn fold(&self, Range { start: l, end: r }: Range<usize>) -> T {
        self.fold_to(..r) - self.fold_to(..l)
    }
}

impl<T: Elem + Ord> FenwickTreeAdd<T> {
    pub fn lower_bound(&self, value: T) -> usize {
        let n = self.size;
        let mut b = 0;
        let mut len = n;

        while len > 0 {
            let half = len / 2;
            let mid = b + half;

            if self.fold_to(..mid + 1) < value {
                len -= half + 1;
                b = mid + 1;
            } else {
                len = half;
            }
        }

        b
    }
}
