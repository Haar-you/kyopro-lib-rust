//! 係数乗算付き区間加算区間総和遅延セグ木

use crate::misc::range::range_bounds_to_range;
use crate::num::one_zero::Zero;
use std::cell::Cell;
use std::ops::{Add, Mul, RangeBounds};

/// 係数乗算付き区間加算区間総和遅延セグ木
pub struct LazySegtreeCoeff<T, U = T> {
    size: usize,
    original_size: usize,
    data: Vec<Cell<T>>,
    lazy: Vec<Cell<T>>,
    coeff: Vec<U>,
}

impl<T, U> LazySegtreeCoeff<T, U>
where
    T: Copy + Zero + Add<Output = T> + Mul<U, Output = T> + PartialEq,
    U: Copy + Default + Add<Output = U>,
{
    /// ‍係数`coefficients`を設定した[`LazySegtreeCoeff`]を構築する。
    pub fn new(n: usize, coefficients: Vec<U>) -> Self {
        let size = n.next_power_of_two() * 2;

        let mut coeff = vec![U::default(); size];

        for i in 0..coefficients.len() {
            coeff[i + size / 2] = coefficients[i];
        }
        for i in (1..size / 2).rev() {
            coeff[i] = coeff[i << 1] + coeff[(i << 1) | 1];
        }

        Self {
            size,
            original_size: n,
            data: vec![Cell::new(T::zero()); size],
            lazy: vec![Cell::new(T::zero()); size],
            coeff,
        }
    }

    /// `self.fold(i..i+1) = value[i]`となるように割り当てる。
    pub fn set_vec(&mut self, value: Vec<T>) {
        self.data = vec![Cell::new(T::zero()); self.size];
        self.lazy = vec![Cell::new(T::zero()); self.size];

        for (i, x) in value.into_iter().enumerate() {
            self.data[self.size / 2 + i].set(x);
        }
        for i in (1..self.size / 2).rev() {
            self.data[i].set(self.data[i << 1].get() + self.data[(i << 1) | 1].get());
        }
    }

    fn propagate(&self, i: usize) {
        if self.lazy[i].get() != T::zero() {
            if i < self.size / 2 {
                self.lazy[i << 1].set(self.lazy[i].get() + self.lazy[i << 1].get());
                self.lazy[(i << 1) | 1].set(self.lazy[i].get() + self.lazy[(i << 1) | 1].get());
            }
            self.data[i].set(self.data[i].get() + self.lazy[i].get() * self.coeff[i]);
            self.lazy[i].set(T::zero());
        }
    }

    fn _update(&mut self, i: usize, l: usize, r: usize, s: usize, t: usize, value: T) -> T {
        self.propagate(i);
        if r <= s || t <= l {
            return self.data[i].get();
        }
        if s <= l && r <= t {
            self.lazy[i].set(self.lazy[i].get() + value);
            self.propagate(i);
            return self.data[i].get();
        }

        let m = (l + r) / 2;
        let t =
            self._update(i << 1, l, m, s, t, value) + self._update((i << 1) | 1, m, r, s, t, value);

        self.data[i].set(t);
        t
    }

    fn _fold(&self, i: usize, l: usize, r: usize, x: usize, y: usize) -> T {
        self.propagate(i);
        if r <= x || y <= l {
            return T::zero();
        }
        if x <= l && r <= y {
            return self.data[i].get();
        }

        let m = (l + r) / 2;
        self._fold(i << 1, l, m, x, y) + self._fold((i << 1) | 1, m, r, x, y)
    }

    /// 範囲`range`に値`value`を加算する。
    pub fn update(&mut self, range: impl RangeBounds<usize>, value: T) {
        let (start, end) = range_bounds_to_range(range, 0, self.original_size);
        self._update(1, 0, self.size / 2, start, end, value);
    }

    /// 範囲`range`で総和を取る。
    pub fn fold(&self, range: impl RangeBounds<usize>) -> T {
        let (start, end) = range_bounds_to_range(range, 0, self.original_size);
        self._fold(1, 0, self.size / 2, start, end)
    }
}

#[cfg(test)]
mod tests {
    use std::iter::repeat_with;

    use crate::{iter::collect::CollectVec, math::prime_mod::Prime, num::const_modint::*};

    use super::*;
    use my_testtools::rand_range;
    use rand::Rng;

    #[test]
    fn test() {
        let n = 100;
        let q = 1000;

        let modulo = ConstModIntBuilder::<Prime<998244353>>::new();

        let mut rng = rand::thread_rng();

        let mut a = repeat_with(|| modulo.from_u64(rng.gen_range(0..10)))
            .take(n)
            .collect_vec();

        let c = repeat_with(|| modulo.from_u64(rng.gen_range(0..10)))
            .take(n)
            .collect_vec();

        let mut seg = LazySegtreeCoeff::new(n, c.clone());
        seg.set_vec(a.clone());

        for _ in 0..q {
            let range = rand_range(&mut rng, 0..n);

            let value = modulo.from_u64(rng.gen_range(0..10));
            seg.update(range.clone(), value);

            for i in range {
                a[i] += c[i] * value;
            }

            let range = rand_range(&mut rng, 0..n);

            let mut ans = modulo.from_u64(0);
            for i in range.clone() {
                ans += a[i];
            }

            assert_eq!(seg.fold(range), ans);
        }
    }
}
