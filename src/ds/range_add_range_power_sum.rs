//! 範囲加算・範囲累乗和取得
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc455/tasks/abc455_f> (2乗和)

use std::ops::{Add, AddAssign, Mul, MulAssign, RangeBounds};

use crate::{misc::range::range_bounds_to_range, num::one_zero::*};

/// 範囲加算・範囲累乗和取得ができるデータ構造
pub struct RangeAddRangePowerSum<T> {
    original_size: usize,
    size: usize,
    lazy: Vec<T>,
    power_sum: Vec<Vec<T>>,
    power: usize,
    coeffs: Vec<Vec<T>>,
}

impl<T> RangeAddRangePowerSum<T>
where
    T: Copy + Zero + One + AddAssign + Add<Output = T> + MulAssign + Mul<Output = T> + PartialEq,
{
    /// 範囲加算・範囲`p`乗和ができる長さ`n`の列を作る。
    pub fn new(n: usize, p: usize) -> Self {
        assert!(p >= 1);
        let size = n.next_power_of_two() * 2;

        let mut power_sum = vec![vec![T::zero(); p + 1]; size];
        for power_sum_i in power_sum.iter_mut().skip(size / 2) {
            power_sum_i[0] = T::one();
        }
        for i in (1..size / 2).rev() {
            power_sum[i][0] = power_sum[i << 1][0] + power_sum[(i << 1) | 1][0];
        }

        let mut coeffs = vec![vec![T::zero(); p + 1]; p + 1];
        for (j, coeffs_j) in coeffs.iter_mut().enumerate() {
            coeffs_j[0] = T::one();
            coeffs_j[j] = T::one();
        }
        for j in 1..=p {
            for k in 1..=j {
                coeffs[j][k] = coeffs[j - 1][k - 1] + coeffs[j - 1][k];
            }
        }

        Self {
            original_size: n,
            size,
            lazy: vec![T::zero(); size],
            power_sum,
            power: p,
            coeffs,
        }
    }

    fn propagate(&mut self, i: usize) {
        if self.lazy[i] == T::zero() {
            return;
        }
        if i < self.size / 2 {
            let l = i << 1;
            let r = (i << 1) | 1;

            self.lazy[l] = self.lazy[l] + self.lazy[i];
            self.lazy[r] = self.lazy[r] + self.lazy[i];
        }

        for k in (1..=self.power).rev() {
            let mut lp = self.lazy[i];
            for j in (0..k).rev() {
                self.power_sum[i][k] =
                    self.power_sum[i][k] + self.coeffs[k][j] * self.power_sum[i][j] * lp;
                lp *= self.lazy[i];
            }
        }

        self.lazy[i] = T::zero();
    }

    fn propagate_top_down(&mut self, mut i: usize) {
        let mut temp = vec![i];
        while i > 1 {
            i >>= 1;
            temp.push(i);
        }

        for i in temp.into_iter().rev() {
            self.propagate(i);
        }
    }

    fn bottom_up(&mut self, mut i: usize) {
        while i > 1 {
            i >>= 1;
            self.propagate(i << 1);
            self.propagate((i << 1) | 1);

            for k in 0..=self.power {
                self.power_sum[i][k] = self.power_sum[i << 1][k] + self.power_sum[(i << 1) | 1][k];
            }
        }
    }

    /// `i`番目の値の累乗を返す。
    pub fn get(&mut self, i: usize) -> Vec<T> {
        self.propagate_top_down(i + self.size / 2);
        self.power_sum[i + self.size / 2].clone()
    }

    /// 区間`range`で累乗和を計算する。
    pub fn fold(&mut self, range: impl RangeBounds<usize>) -> Vec<T> {
        let (l, r) = range_bounds_to_range(range, 0, self.original_size);

        self.propagate_top_down(l + self.size / 2);
        if r < self.size / 2 {
            self.propagate_top_down(r + self.size / 2);
        }

        let mut ret = vec![T::zero(); self.power + 1];

        let mut l = l + self.size / 2;
        let mut r = r + self.size / 2;

        while l < r {
            if r & 1 == 1 {
                r -= 1;
                self.propagate(r);
                for (ret, sum) in ret.iter_mut().zip(self.power_sum[r].iter()) {
                    *ret += *sum;
                }
            }
            if l & 1 == 1 {
                self.propagate(l);
                for (ret, sum) in ret.iter_mut().zip(self.power_sum[l].iter()) {
                    *ret += *sum;
                }
                l += 1;
            }
            r >>= 1;
            l >>= 1;
        }

        ret
    }

    /// `i`番目の値を`value`で置き換える。
    pub fn assign(&mut self, i: usize, value: T) {
        self.propagate_top_down(i + self.size / 2);

        let mut p = value;
        for k in 1..=self.power {
            self.power_sum[i + self.size / 2][k] = p;
            p *= value;
        }
        self.bottom_up(i + self.size / 2);
    }

    /// 区間`range`で値`x`を範囲加算する。
    pub fn update(&mut self, range: impl RangeBounds<usize>, x: T) {
        let (l, r) = range_bounds_to_range(range, 0, self.original_size);

        self.propagate_top_down(l + self.size / 2);
        if r < self.size / 2 {
            self.propagate_top_down(r + self.size / 2);
        }

        {
            let mut l = l + self.size / 2;
            let mut r = r + self.size / 2;

            while l < r {
                if r & 1 == 1 {
                    r -= 1;
                    self.lazy[r] += x;
                }
                if l & 1 == 1 {
                    self.lazy[l] += x;
                    l += 1;
                }
                r >>= 1;
                l >>= 1;
            }
        }

        self.bottom_up(l + self.size / 2);
        if r < self.size / 2 {
            self.bottom_up(r + self.size / 2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::prime_mod::*, num::const_modint::*};

    use my_testtools::rand_range;
    use rand::prelude::*;

    type P = Prime<998244353>;
    type M = ConstModInt<P>;

    #[test]
    fn test() {
        let mut rng = rand::rng();
        let ff = ConstModIntBuilder::<P>::new();
        let n = 500;
        let p = 5;

        let mut s = RangeAddRangePowerSum::<M>::new(n, p);
        let mut a = vec![ff.from_u64(0); n];

        for (i, ai) in a.iter_mut().enumerate() {
            let x = ff.from_u64(rng.random());
            *ai = x;
            s.assign(i, x);
        }

        for _ in 0..1000 {
            let range = rand_range(&mut rng, 0..n);
            let value = ff.from_u64(rng.random());

            a[range.clone()].iter_mut().for_each(|x| *x += value);
            s.update(range, value);

            let range = rand_range(&mut rng, 0..n);

            for p in 1..=p {
                let ans = a[range.clone()]
                    .iter()
                    .map(|&x| x.pow(p as u64))
                    .fold(M::zero(), |a, b| a + b);

                assert_eq!(ans, s.fold(range.clone())[p]);
            }
        }
    }
}
