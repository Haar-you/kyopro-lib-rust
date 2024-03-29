//! 区間加算・区間Max(Min)

pub use crate::ds::traits::{Foldable, Updatable};
use crate::traits::one_zero::Zero;
use crate::{rec, trait_alias};
use std::{
    cmp::{max, min},
    ops::{Add, AddAssign, Range, SubAssign},
};

trait_alias!(
    Elem,
    Add<Output = Self> + AddAssign + SubAssign + Ord + Copy + Zero<Output = Self>
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
    data: Vec<T>,
    mode: Mode,
}

impl<T: Elem> StarrySkyTree<T> {
    pub fn new(n: usize, mode: Mode) -> Self {
        let size = n.next_power_of_two() * 2;
        let zero = T::zero();
        StarrySkyTree {
            size,
            data: vec![zero; size],
            mode,
        }
    }
}

impl<T: Elem> Foldable<Range<usize>> for StarrySkyTree<T> {
    type Output = Option<T>;

    fn fold(&self, Range { start: l, end: r }: Range<usize>) -> Self::Output {
        let s = l;
        let t = r;

        let rec = rec!(
            |rec, (i, l, r, value): (usize, usize, usize, T)| -> Option<T> {
                if r <= s || t <= l {
                    return None;
                }
                if s <= l && r <= t {
                    return Some(value + self.data[i]);
                }

                let a = rec((i << 1, l, (l + r) / 2, value + self.data[i]));
                let b = rec((i << 1 | 1, (l + r) / 2, r, value + self.data[i]));

                match (a, b) {
                    (None, _) => b,
                    (_, None) => a,
                    (Some(a), Some(b)) => Some(self.mode.op(a, b)),
                }
            }
        );

        rec((1, 0, self.size / 2, T::zero()))
    }
}

impl<T: Elem> Updatable<Range<usize>> for StarrySkyTree<T> {
    type Value = T;

    fn update(&mut self, Range { start: l, end: r }: Range<usize>, value: T) {
        let hsize = self.size / 2;
        let mut ll = l + self.size / 2;
        let mut rr = r + self.size / 2;

        while ll < rr {
            if (rr & 1) != 0 {
                rr -= 1;
                self.data[rr] += value;
            }
            if (ll & 1) != 0 {
                self.data[ll] += value;
                ll += 1;
            }
            ll >>= 1;
            rr >>= 1;
        }

        let mut bottom_up = |mut i: usize| {
            if i > self.size {
                return;
            }

            while i >= 1 {
                if i < self.size / 2 {
                    let d = self.mode.op(self.data[i << 1], self.data[i << 1 | 1]);

                    self.data[i << 1] -= d;
                    self.data[i << 1 | 1] -= d;
                    self.data[i] += d;
                }

                i >>= 1;
            }
        };

        bottom_up(l + hsize);
        bottom_up(r + hsize);
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
