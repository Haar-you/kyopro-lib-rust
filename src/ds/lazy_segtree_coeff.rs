//! 係数乗算付き区間加算区間総和遅延セグ木

pub use crate::ds::traits::{Foldable, Updatable};
use std::cell::Cell;
use std::ops::{Add, Mul, Range};

pub struct LazySegmentTreeCoeff<T> {
    size: usize,
    data: Vec<Cell<T>>,
    lazy: Vec<Cell<T>>,
    coeff: Vec<T>,
}

impl<T> LazySegmentTreeCoeff<T>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + PartialEq,
{
    pub fn new(n: usize, coefficients: Vec<T>) -> Self {
        let size = n.next_power_of_two() * 2;

        let mut coeff = vec![T::default(); size];

        for i in 0..coefficients.len() {
            coeff[i + size / 2] = coefficients[i];
        }
        for i in (1..size / 2).rev() {
            coeff[i] = coeff[i << 1] + coeff[i << 1 | 1];
        }

        Self {
            size,
            data: vec![Cell::new(T::default()); size],
            lazy: vec![Cell::new(T::default()); size],
            coeff,
        }
    }

    pub fn init_with_vec(&mut self, value: Vec<T>) {
        self.data = vec![Cell::new(T::default()); self.size];
        self.lazy = vec![Cell::new(T::default()); self.size];

        for (i, x) in value.into_iter().enumerate() {
            self.data[self.size / 2 + i].set(x);
        }
        for i in (1..self.size / 2).rev() {
            self.data[i].set(self.data[i << 1].get() + self.data[i << 1 | 1].get());
        }
    }

    fn propagate(&self, i: usize) {
        if self.lazy[i].get() != T::default() {
            if i < self.size / 2 {
                self.lazy[i << 1].set(self.lazy[i].get() + self.lazy[i << 1].get());
                self.lazy[i << 1 | 1].set(self.lazy[i].get() + self.lazy[i << 1 | 1].get());
            }
            self.data[i].set(self.data[i].get() + self.lazy[i].get() * self.coeff[i]);
            self.lazy[i].set(T::default());
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
            + self.update_internal(i << 1 | 1, (l + r) / 2, r, s, t, value);

        self.data[i].replace(t)
    }

    fn get_internal(&self, i: usize, l: usize, r: usize, x: usize, y: usize) -> T {
        self.propagate(i);
        if r <= x || y <= l {
            return T::default();
        }
        if x <= l && r <= y {
            return self.data[i].get();
        }
        self.get_internal(i << 1, l, (l + r) / 2, x, y)
            + self.get_internal(i << 1 | 1, (l + r) / 2, r, x, y)
    }
}

impl<T> Updatable<Range<usize>> for LazySegmentTreeCoeff<T>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + PartialEq,
{
    type Value = T;
    fn update(&mut self, Range { start, end }: Range<usize>, value: T) {
        self.update_internal(1, 0, self.size / 2, start, end, value);
    }
}

impl<T> Foldable<Range<usize>> for LazySegmentTreeCoeff<T>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + PartialEq,
{
    type Output = T;
    fn fold(&self, Range { start, end }: Range<usize>) -> T {
        self.get_internal(1, 0, self.size / 2, start, end)
    }
}
