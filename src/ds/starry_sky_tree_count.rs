//! 区間加算・個数総和付き区間Max(Min)

use crate::misc::range::range_bounds_to_range;
use crate::num::one_zero::Zero;
use std::{
    cmp::{max, min},
    ops::{Add, RangeBounds, Sub},
};

/// 区間Max/Minを選択する。
#[derive(Copy, Clone)]
pub enum Mode {
    /// 区間Max
    Max,
    /// 区間Min
    Min,
}

impl Mode {
    fn op<T: Ord>(self, a: T, b: T) -> T {
        match self {
            Mode::Max => max(a, b),
            Mode::Min => min(a, b),
        }
    }
}

/// 区間加算・個数総和付き区間Max(Min)ができるデータ構造。
pub struct StarrySkyTreeCount<T> {
    size: usize,
    original_size: usize,
    data: Vec<T>,
    count: Vec<u64>,
    mode: Mode,
}

impl<T> StarrySkyTreeCount<T>
where
    T: Add<Output = T> + Sub<Output = T> + Ord + Copy + Zero,
{
    /// **Time complexity** $O(n)$
    pub fn new(coeffs: Vec<u64>, mode: Mode) -> Self {
        let n = coeffs.len();
        let size = n.next_power_of_two() * 2;
        let zero = T::zero();

        let mut count = vec![0; size];
        for (i, &x) in coeffs.iter().enumerate() {
            count[size / 2 + i] = x;
        }
        for i in (1..size / 2).rev() {
            count[i] = count[i << 1] + count[(i << 1) | 1];
        }

        Self {
            size,
            original_size: n,
            data: vec![zero; size],
            count,
            mode,
        }
    }

    fn rec(&self, s: usize, t: usize, i: usize, l: usize, r: usize, value: T) -> Option<(T, u64)> {
        if r <= s || t <= l {
            return None;
        }
        if s <= l && r <= t {
            return Some((value + self.data[i], self.count[i]));
        }

        let m = (l + r) / 2;
        let a = self.rec(s, t, i << 1, l, m, value + self.data[i]);
        let b = self.rec(s, t, (i << 1) | 1, m, r, value + self.data[i]);

        match (a, b) {
            (None, _) => b,
            (_, None) => a,
            (Some((a, ca)), Some((b, cb))) => {
                let t = self.mode.op(a, b);
                if a == b {
                    Some((a, ca + cb))
                } else if a == t {
                    Some((a, ca))
                } else {
                    Some((b, cb))
                }
            }
        }
    }

    /// **Time complexity** $O(\log n)$
    pub fn fold(&self, range: impl RangeBounds<usize>) -> Option<(T, u64)> {
        let (l, r) = range_bounds_to_range(range, 0, self.original_size);
        self.rec(l, r, 1, 0, self.size / 2, T::zero())
    }

    fn bottom_up(&mut self, mut i: usize) {
        if i > self.size {
            return;
        }

        while i >= 1 {
            if i < self.size / 2 {
                let d = self.mode.op(self.data[i << 1], self.data[(i << 1) | 1]);

                self.data[i << 1] = self.data[i << 1] - d;
                self.data[(i << 1) | 1] = self.data[(i << 1) | 1] - d;
                self.data[i] = self.data[i] + d;

                let l = self.data[i << 1];
                let r = self.data[(i << 1) | 1];
                let t = self.mode.op(l, r);

                if l == r {
                    self.count[i] = self.count[i << 1] + self.count[(i << 1) | 1];
                } else if l == t {
                    self.count[i] = self.count[i << 1];
                } else {
                    self.count[i] = self.count[(i << 1) | 1];
                }
            }

            i >>= 1;
        }
    }

    /// **Time complexity** $O(\log n)$
    pub fn update(&mut self, range: impl RangeBounds<usize>, value: T) {
        let (l, r) = range_bounds_to_range(range, 0, self.original_size);

        let hsize = self.size / 2;
        let mut ll = l + hsize;
        let mut rr = r + hsize;

        while ll < rr {
            if (rr & 1) != 0 {
                rr -= 1;
                self.data[rr] = self.data[rr] + value;
            }
            if (ll & 1) != 0 {
                self.data[ll] = self.data[ll] + value;
                ll += 1;
            }
            ll >>= 1;
            rr >>= 1;
        }

        self.bottom_up(l + hsize);
        self.bottom_up(r + hsize);
    }
}
