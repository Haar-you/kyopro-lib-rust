//! 係数乗算付き区間加算区間総和遅延セグ木

use crate::misc::range::range_bounds_to_range;
use crate::num::one_zero::Zero;
use crate::trait_alias;
use std::cell::Cell;
use std::ops::{Add, Mul, RangeBounds};

trait_alias!(
    /// [`LazySegtreeCoeff<T>`]が扱える型
    Elem: Copy + Zero + Add<Output = Self> + Mul<Output = Self> + PartialEq
);

/// 係数乗算付き区間加算区間総和遅延セグ木
pub struct LazySegtreeCoeff<T> {
    size: usize,
    original_size: usize,
    data: Vec<Cell<T>>,
    lazy: Vec<Cell<T>>,
    coeff: Vec<T>,
}

impl<T: Elem> LazySegtreeCoeff<T> {
    /// ‍係数`coefficients`を設定した[`LazySegtreeCoeff`]を構築する。
    pub fn new(n: usize, coefficients: Vec<T>) -> Self {
        let size = n.next_power_of_two() * 2;

        let mut coeff = vec![T::zero(); size];

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

    fn update_internal(&mut self, i: usize, l: usize, r: usize, s: usize, t: usize, value: T) -> T {
        self.propagate(i);
        if r <= s || t <= l {
            return self.data[i].get();
        }
        if s <= l && r <= t {
            self.lazy[i].set(self.lazy[i].get() + value);
            self.propagate(i);
            return self.data[i].get();
        }

        let t = self.update_internal(i << 1, l, (l + r) / 2, s, t, value)
            + self.update_internal((i << 1) | 1, (l + r) / 2, r, s, t, value);

        self.data[i].replace(t)
    }

    fn get_internal(&self, i: usize, l: usize, r: usize, x: usize, y: usize) -> T {
        self.propagate(i);
        if r <= x || y <= l {
            return T::zero();
        }
        if x <= l && r <= y {
            return self.data[i].get();
        }
        self.get_internal(i << 1, l, (l + r) / 2, x, y)
            + self.get_internal((i << 1) | 1, (l + r) / 2, r, x, y)
    }

    /// 範囲`range`に値`value`を加算する。
    pub fn update(&mut self, range: impl RangeBounds<usize>, value: T) {
        let (start, end) = range_bounds_to_range(range, 0, self.original_size);
        self.update_internal(1, 0, self.size / 2, start, end, value);
    }

    /// 範囲`range`で総和を取る。
    pub fn fold(&self, range: impl RangeBounds<usize>) -> T {
        let (start, end) = range_bounds_to_range(range, 0, self.original_size);
        self.get_internal(1, 0, self.size / 2, start, end)
    }
}
