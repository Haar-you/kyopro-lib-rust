//! Merge-sort Tree
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc339/tasks/abc339_g>
//! - <https://atcoder.jp/contests/abc351/tasks/abc351_f>

use crate::algo::{bsearch::upper_bound, merge::inplace_merge};
use crate::misc::range::range_bounds_to_range;
use crate::num::one_zero::Zero;
use std::ops::{Add, AddAssign, RangeBounds};

/// Merge-sort Tree
pub struct MergeSortTree<T> {
    data: Vec<Vec<T>>,
    accum: Vec<Vec<T>>,
    size: usize,
    original_size: usize,
}

impl<T> MergeSortTree<T>
where
    T: Copy + Clone + Zero + Add<Output = T> + AddAssign + PartialOrd + Ord,
{
    /// **Time complexity** $O(n \log n)$
    ///
    /// **Space complexity** $O(n \log n)$
    pub fn new(mut a: Vec<T>) -> Self {
        let n = a.len();
        let size = n.next_power_of_two() * 2;

        let mut this = Self {
            data: vec![vec![]; size],
            accum: vec![vec![]; size],
            size,
            original_size: n,
        };

        this._init(1, &mut a, 0, size / 2);

        this
    }

    fn _init(&mut self, i: usize, a: &mut [T], l: usize, r: usize) {
        if a.len() <= l {
            return;
        }

        if r - l == 1 {
            self.data[i] = a[l..r].to_vec();
        } else {
            let mid = (l + r) / 2;
            self._init(i << 1, a, l, mid);
            self._init(i << 1 | 1, a, mid, r);

            if a.len() <= mid {
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
    ///
    /// **Time complexity** $O((\log N) ^ 2)$
    pub fn sum_count_le(&self, range: impl RangeBounds<usize>, ub: T) -> (T, usize) {
        let (l, r) = range_bounds_to_range(range, 0, self.original_size);
        assert!(l <= r && r <= self.original_size);

        let mut l = l + self.size / 2;
        let mut r = r + self.size / 2;
        let mut sum = T::zero();
        let mut count = 0;

        while l < r {
            if r & 1 == 1 {
                r -= 1;
                let i = upper_bound(&self.data[r], &ub);
                count += i;
                sum += self.accum[r][i];
            }
            if l & 1 == 1 {
                let i = upper_bound(&self.data[l], &ub);
                count += i;
                sum += self.accum[l][i];
                l += 1;
            }
            r >>= 1;
            l >>= 1;
        }

        (sum, count)
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

        let n = 300;
        let t = 300;

        let a = (0..n).map(|_| rng.gen::<u64>() % 10000).collect::<Vec<_>>();
        let s = MergeSortTree::new(a.clone());

        for _ in 0..t {
            let Range { start: l, end: r } = rand_range(&mut rng, 0..n);
            let x = rng.gen::<u64>() % 10000;

            let (res_sum, res_count) = s.sum_count_le(l..r, x);
            let ans_sum = a[l..r].iter().filter(|&&y| y <= x).sum::<u64>();
            let ans_count = a[l..r].iter().filter(|&&y| y <= x).count();

            assert_eq!(res_sum, ans_sum);
            assert_eq!(res_count, ans_count);
        }
    }
}
