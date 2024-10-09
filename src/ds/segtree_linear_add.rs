//! 区間一次関数加算セグメントツリー
//!
//! # Problems
//!
//! - [HUPC 2020 B 三角形足し算](https://onlinejudge.u-aizu.ac.jp/challenges/sources/VPC/HUPC/3165?year=2020)

use crate::num::one_zero::Zero;
use crate::trait_alias;
use crate::utils::{linear::*, range::range_bounds_to_range};
use std::{
    cell::Cell,
    mem::size_of,
    ops::{Add, Mul, RangeBounds},
};

trait_alias!(
    Elem,
    Copy + Add<Output = Self> + Mul<Output = Self> + Zero + From<u32>
);

pub struct SegtreeLinearAdd<T> {
    hsize: usize,
    original_size: usize,
    data: Vec<Cell<(T, T)>>,
    from: Vec<usize>,
}

fn add<T: Add<Output = T>>((a, b): (T, T), (c, d): (T, T)) -> (T, T) {
    (a + c, b + d)
}

impl<T: Elem> SegtreeLinearAdd<T> {
    /// **Time complexity O(n)**
    pub fn new(n: usize) -> Self {
        let size = n.next_power_of_two() * 2;
        let hsize = size / 2;
        let mut from = vec![0; size];

        let mut s = 0;
        for (i, x) in from.iter_mut().enumerate().skip(1) {
            *x = s;
            let l = hsize >> (size_of::<usize>() as u32 * 8 - 1 - i.leading_zeros());
            s += l;
            if s == hsize {
                s = 0;
            }
        }

        Self {
            hsize,
            original_size: n,
            data: vec![Cell::new((T::zero(), T::zero())); size],
            from,
        }
    }

    /// 範囲`l..r`に一次関数`ax + b`の値を加算する。(`x`の値は`l..r`の範囲)
    ///
    /// **Time complexity O(log n)**
    pub fn update(&mut self, range: impl RangeBounds<usize>, linear: Linear<T>) {
        let (l, r) = range_bounds_to_range(range, 0, self.original_size);

        let mut l = l + self.hsize;
        let mut r = r + self.hsize;

        while l < r {
            if r & 1 == 1 {
                r -= 1;
                self.data[r].set(add(
                    self.data[r].get(),
                    (linear.apply(T::from(self.from[r] as u32)), linear.a),
                ));
            }
            if l & 1 == 1 {
                self.data[l].set(add(
                    self.data[l].get(),
                    (linear.apply(T::from(self.from[l] as u32)), linear.a),
                ));
                l += 1;
            }

            l >>= 1;
            r >>= 1;
        }
    }

    fn propagate(&self, i: usize) {
        if i < self.hsize {
            self.data[i << 1].set(add(self.data[i << 1].get(), self.data[i].get()));

            let len = self.hsize >> (size_of::<usize>() as u32 * 8 - i.leading_zeros());
            self.data[i].set((
                self.data[i].get().0 + self.data[i].get().1 * T::from(len as u32),
                self.data[i].get().1,
            ));
            self.data[i << 1 | 1].set(add(self.data[i << 1 | 1].get(), self.data[i].get()));

            self.data[i].set((T::zero(), T::zero()));
        }
    }

    fn propagate_top_down(&self, mut i: usize) {
        let mut temp = vec![];
        while i > 1 {
            i >>= 1;
            temp.push(i);
        }

        for i in temp.into_iter().rev() {
            self.propagate(i);
        }
    }

    /// **Time complexity O(log n)**
    pub fn get(&self, i: usize) -> T {
        self.propagate_top_down(i + self.hsize);
        self.data[i + self.hsize].get().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testtools::*;
    use rand::Rng;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let n = 100;
        let mut seg = SegtreeLinearAdd::<u64>::new(n);
        let mut vec = vec![0; n];

        for _ in 0..300 {
            let lr = rand_range(&mut rng, 0..n);

            let a = rng.gen_range(0..100);
            let b = rng.gen_range(0..100);

            seg.update(lr.clone(), Linear { a, b });

            for i in lr {
                vec[i] += a * i as u64 + b;
            }

            assert_eq!((0..n).map(|i| seg.get(i)).collect::<Vec<_>>(), vec);
        }
    }
}
