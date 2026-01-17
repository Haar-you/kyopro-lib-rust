//! 非可換なモノイドを双方向から計算するセグメント木
use std::ops::RangeBounds;

use crate::{algebra::traits::Monoid, misc::range::range_bounds_to_range};

/// 非可換なモノイドを双方向から計算するセグメント木
pub struct SegtreeBidir<M: Monoid> {
    monoid: M,
    original_size: usize,
    size: usize,
    data_l: Vec<M::Element>,
    data_r: Vec<M::Element>,
}

impl<M: Monoid> SegtreeBidir<M>
where
    M::Element: Clone,
{
    /// **Time complexity** $O(n)$
    pub fn new(monoid: M, n: usize) -> Self {
        let size = n.next_power_of_two() * 2;
        Self {
            original_size: n,
            size,
            data_l: vec![monoid.id(); size],
            data_r: vec![monoid.id(); size],
            monoid,
        }
    }

    /// モノイド列から`SegtreeBidir`を構築する。
    ///
    /// **Time complexity** $O(|s|)$
    pub fn from_vec(monoid: M, s: Vec<M::Element>) -> Self {
        let mut this = Self::new(monoid, s.len());

        for (i, x) in s.into_iter().enumerate() {
            this.data_l[i + this.size / 2] = x.clone();
            this.data_r[i + this.size / 2] = x;
        }

        for i in (1..this.size / 2).rev() {
            this.data_l[i] = this.monoid.op(
                this.data_l[i << 1].clone(),
                this.data_l[(i << 1) | 1].clone(),
            );
            this.data_r[i] = this.monoid.op(
                this.data_r[(i << 1) | 1].clone(),
                this.data_r[i << 1].clone(),
            );
        }

        this
    }

    /// **Time complexity** $O(\log n)$
    pub fn fold_left<R: RangeBounds<usize>>(&self, range: R) -> M::Element {
        let (l, r) = range_bounds_to_range(range, 0, self.original_size);

        let mut ret_l = self.monoid.id();
        let mut ret_r = self.monoid.id();

        let mut l = l + self.size / 2;
        let mut r = r + self.size / 2;

        while l < r {
            if r & 1 == 1 {
                r -= 1;
                ret_r = self.monoid.op(self.data_l[r].clone(), ret_r);
            }
            if l & 1 == 1 {
                ret_l = self.monoid.op(ret_l, self.data_l[l].clone());
                l += 1;
            }
            r >>= 1;
            l >>= 1;
        }

        self.monoid.op(ret_l, ret_r)
    }

    /// **Time complexity** $O(\log n)$
    pub fn fold_right<R: RangeBounds<usize>>(&self, range: R) -> M::Element {
        let (l, r) = range_bounds_to_range(range, 0, self.original_size);

        let mut ret_l = self.monoid.id();
        let mut ret_r = self.monoid.id();

        let mut l = l + self.size / 2;
        let mut r = r + self.size / 2;

        while l < r {
            if r & 1 == 1 {
                r -= 1;
                ret_r = self.monoid.op(ret_r, self.data_r[r].clone());
            }
            if l & 1 == 1 {
                ret_l = self.monoid.op(self.data_r[l].clone(), ret_l);
                l += 1;
            }
            r >>= 1;
            l >>= 1;
        }

        self.monoid.op(ret_r, ret_l)
    }

    /// **Time complexity** $O(\log n)$
    pub fn assign(&mut self, i: usize, value: M::Element) {
        let mut i = i + self.size / 2;
        self.data_l[i] = value.clone();
        self.data_r[i] = value;

        while i > 1 {
            i >>= 1;
            self.data_l[i] = self.monoid.op(
                self.data_l[i << 1].clone(),
                self.data_l[(i << 1) | 1].clone(),
            );
            self.data_r[i] = self.monoid.op(
                self.data_r[(i << 1) | 1].clone(),
                self.data_r[i << 1].clone(),
            );
        }
    }
}
