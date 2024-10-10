//! 区間加算・区間Max(Min)

use crate::num::one_zero::Zero;
use crate::trait_alias;
use crate::utils::range::range_bounds_to_range;
use std::{
    cmp::{max, min},
    ops::{Add, RangeBounds, Sub},
};

trait_alias!(
    Elem,
    Add<Output = Self> + Sub<Output = Self> + Ord + Copy + Zero
);

#[derive(Copy, Clone)]
pub enum Mode {
    Max,
    Min,
}

impl Mode {
    fn op<T: Ord + Copy>(self, a: T, b: T) -> T {
        match self {
            Mode::Max => max(a, b),
            Mode::Min => min(a, b),
        }
    }
}

pub struct StarrySkyTree<T> {
    size: usize,
    original_size: usize,
    data: Vec<T>,
    mode: Mode,
}

impl<T: Elem> StarrySkyTree<T> {
    /// **Time complexity O(n)**
    pub fn new(n: usize, mode: Mode) -> Self {
        let size = n.next_power_of_two() * 2;
        let zero = T::zero();
        Self {
            size,
            original_size: n,
            data: vec![zero; size],
            mode,
        }
    }

    fn rec(&self, s: usize, t: usize, i: usize, l: usize, r: usize, value: T) -> Option<T> {
        if r <= s || t <= l {
            return None;
        }
        if s <= l && r <= t {
            return Some(value + self.data[i]);
        }

        let a = self.rec(s, t, i << 1, l, (l + r) / 2, value + self.data[i]);
        let b = self.rec(s, t, i << 1 | 1, (l + r) / 2, r, value + self.data[i]);

        match (a, b) {
            (None, _) => b,
            (_, None) => a,
            (Some(a), Some(b)) => Some(self.mode.op(a, b)),
        }
    }

    /// **Time complexity O(log n)**
    pub fn fold(&self, range: impl RangeBounds<usize>) -> Option<T> {
        let (l, r) = range_bounds_to_range(range, 0, self.original_size);
        self.rec(l, r, 1, 0, self.size / 2, T::zero())
    }

    fn bottom_up(&mut self, mut i: usize) {
        if i > self.size {
            return;
        }

        while i >= 1 {
            if i < self.size / 2 {
                let d = self.mode.op(self.data[i << 1], self.data[i << 1 | 1]);

                self.data[i << 1] = self.data[i << 1] - d;
                self.data[i << 1 | 1] = self.data[i << 1 | 1] - d;
                self.data[i] = self.data[i] + d;
            }

            i >>= 1;
        }
    }

    /// **Time complexity O(log n)**
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testtools::*;
    use rand::Rng;

    #[test]
    fn test_max() {
        let mut rng = rand::thread_rng();

        let size = 100;
        let mut other = vec![0; size];
        let mut s = StarrySkyTree::<i32>::new(size, Mode::Max);

        for _ in 0..1000 {
            let ty = rng.gen_range(0..2);
            let lr = rand_range(&mut rng, 0..size);

            if ty == 0 {
                let x = rng.gen_range(-1000..=1000);

                s.update(lr.clone(), x);
                for i in lr {
                    other[i] += x;
                }
            } else {
                let ans = lr.clone().map(|i| other[i]).max();

                assert_eq!(s.fold(lr), ans);
            }
        }
    }

    #[test]
    fn test_min() {
        let mut rng = rand::thread_rng();

        let size = 100;
        let mut other = vec![0; size];
        let mut s = StarrySkyTree::<i32>::new(size, Mode::Min);

        for _ in 0..1000 {
            let ty = rng.gen_range(0..2);
            let lr = rand_range(&mut rng, 0..size);

            if ty == 0 {
                let x = rng.gen_range(-1000..=1000);

                s.update(lr.clone(), x);
                for i in lr {
                    other[i] += x;
                }
            } else {
                let ans = lr.clone().map(|i| other[i]).min();

                assert_eq!(s.fold(lr), ans);
            }
        }
    }
}
