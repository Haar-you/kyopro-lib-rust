//! 区間一次関数加算区間総和セグメントツリー

use crate::math::linear::*;
use crate::misc::range::range_bounds_to_range;
use crate::num::one_zero::Zero;
use crate::trait_alias;
use std::ops::{Add, AddAssign, Mul, RangeBounds};

trait_alias!(
    /// [`SegtreeLinearAddRangeSum<T>`]が扱える型
    Elem: Copy + Zero + Add<Output = Self> + Mul<Output = Self> + AddAssign + PartialEq + From<u32>
);

/// 区間一次関数加算区間総和セグメントツリー
pub struct SegtreeLinearAddRangeSum<T> {
    data: Vec<T>,
    lazy: Vec<(T, T)>,
    hsize: usize,
    original_size: usize,
}

impl<T: Elem> SegtreeLinearAddRangeSum<T> {
    /// **Time complexity** $O(n)$
    ///
    /// **Space complexity** $O(n)$
    pub fn new(n: usize) -> Self {
        let hsize = n.next_power_of_two();

        Self {
            data: vec![T::zero(); hsize * 2],
            lazy: vec![(T::zero(), T::zero()); hsize * 2],
            hsize,
            original_size: n,
        }
    }

    fn _add(a: (T, T), b: (T, T)) -> (T, T) {
        (a.0 + b.0, a.1 + b.1)
    }

    fn _propagate(&mut self, i: usize, l: usize, r: usize) {
        if self.lazy[i] == (T::zero(), T::zero()) {
            return;
        }
        if i < self.hsize {
            let mut t = self.lazy[i];
            self.lazy[i << 1] = Self::_add(t, self.lazy[i << 1]);
            t.0 += t.1 * T::from(((r - l) / 2) as u32);
            self.lazy[(i << 1) | 1] = Self::_add(t, self.lazy[(i << 1) | 1]);
        }
        let len = r - l;
        let (s, d) = self.lazy[i];

        self.data[i] += s * T::from(len as u32) + d * T::from(((len - 1) * len / 2) as u32);
        self.lazy[i] = (T::zero(), T::zero());
    }

    fn _update(&mut self, i: usize, l: usize, r: usize, s: usize, t: usize, a: T, b: T) -> T {
        self._propagate(i, l, r);
        if r <= s || t <= l {
            self.data[i]
        } else if s <= l && r <= t {
            self.lazy[i] = Self::_add(self.lazy[i], (a * T::from(l as u32) + b, a));
            self._propagate(i, l, r);
            self.data[i]
        } else {
            let mid = (l + r) / 2;
            self.data[i] = self._update(i << 1, l, mid, s, t, a, b)
                + self._update((i << 1) | 1, mid, r, s, t, a, b);
            self.data[i]
        }
    }

    /// **Time complexity** $O(\log n)$
    pub fn update(&mut self, range: impl RangeBounds<usize>, linear: Linear<T>) {
        let (start, end) = range_bounds_to_range(range, 0, self.original_size);
        self._update(1, 0, self.hsize, start, end, linear.a, linear.b);
    }

    fn _fold(&mut self, i: usize, l: usize, r: usize, x: usize, y: usize) -> T {
        self._propagate(i, l, r);
        if r <= x || y <= l {
            T::zero()
        } else if x <= l && r <= y {
            self.data[i]
        } else {
            let mid = (l + r) / 2;
            self._fold(i << 1, l, mid, x, y) + self._fold((i << 1) | 1, mid, r, x, y)
        }
    }

    /// **Time complexity** $O(\log n)$
    pub fn fold(&mut self, range: impl RangeBounds<usize>) -> T {
        let (start, end) = range_bounds_to_range(range, 0, self.original_size);
        self._fold(1, 0, self.hsize, start, end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testtools::*;
    use rand::Rng;
    use std::ops::Range;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();
        let n = 100;

        let mut seg = SegtreeLinearAddRangeSum::<i64>::new(n);
        let mut vec = vec![0; n];

        for _ in 0..300 {
            let Range { start: l, end: r } = rand_range(&mut rng, 0..n);

            let a = rng.gen_range(0..100);
            let b = rng.gen_range(0..100);

            seg.update(l..r, Linear { a, b });

            for i in l..r {
                vec[i] += a * i as i64 + b;
            }

            let Range { start: l, end: r } = rand_range(&mut rng, 0..n);

            let res = seg.fold(l..r);
            let ans = vec[l..r].iter().sum::<i64>();

            assert_eq!(res, ans);
        }
    }
}
