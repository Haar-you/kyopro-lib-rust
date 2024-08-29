//! Merge-sort Tree
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc339/tasks/abc339_g>

use crate::algo::{bsearch::upper_bound, merge::inplace_merge};
use crate::num::one_zero::Zero;
use std::ops::{Add, AddAssign, Range};

pub struct MergeSortTree<T> {
    data: Vec<Vec<T>>,
    accum: Vec<Vec<T>>,
    size: usize,
}

impl<T> MergeSortTree<T>
where
    T: Copy + Clone + Zero<Output = T> + Add<Output = T> + AddAssign + PartialOrd + Ord,
{
    pub fn new(mut a: Vec<T>) -> Self {
        let n = a.len();
        let size = n.next_power_of_two() * 2;

        let mut ret = Self {
            data: vec![vec![]; size],
            accum: vec![vec![]; size],
            size,
        };

        ret._init(1, &mut a, 0, size / 2);

        ret
    }

    fn _init(&mut self, i: usize, a: &mut [T], l: usize, r: usize) {
        if r - l == 1 {
            self.data[i] = a.get(l..r).map_or(vec![], |a| a.to_vec());
        } else {
            let mid = (l + r) / 2;
            self._init(i << 1, a, l, mid);
            self._init(i << 1 | 1, a, mid, r);

            if a.len() <= l {
                self.data[i] = vec![];
            } else if a.len() <= mid {
                self.data[i] = a[l..].to_vec();
            } else {
                let k = mid - l;
                let end = r.min(a.len());
                inplace_merge(&mut a[l..end], k);

                self.data[i] = a[l..end].to_vec();
            }
        }
        self.accum[i] = Self::_accum(&self.data[i]);
    }

    fn _accum(a: &[T]) -> Vec<T> {
        let mut ret = vec![T::zero(); a.len() + 1];
        for (i, x) in a.iter().enumerate() {
            ret[i + 1] = ret[i] + *x;
        }
        ret
    }

    /// `ub`以下の総和を求める
    pub fn sum_le(&self, Range { start: l, end: r }: Range<usize>, ub: T) -> T {
        let mut l = l + self.size / 2;
        let mut r = r + self.size / 2;
        let mut ret = T::zero();

        while l < r {
            if r & 1 == 1 {
                r -= 1;
                let i = upper_bound(&self.data[r], &ub);
                ret += self.accum[r][i];
            }
            if l & 1 == 1 {
                let i = upper_bound(&self.data[l], &ub);
                ret += self.accum[l][i];
                l += 1;
            }
            r >>= 1;
            l >>= 1;
        }

        ret
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

        let n = 300;
        let t = 300;

        let a = (0..n).map(|_| rng.gen::<u64>() % 10000).collect::<Vec<_>>();
        let s = MergeSortTree::new(a.clone());

        for _ in 0..t {
            let Range { start: l, end: r } = rand_range(&mut rng, 0..n);
            let x = rng.gen::<u64>() % 10000;

            let res = s.sum_le(l..r, x);
            let ans = a[l..r].iter().filter(|&&y| y <= x).sum::<u64>();

            assert_eq!(res, ans);
        }
    }
}
