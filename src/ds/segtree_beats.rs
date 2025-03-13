//! Segment Tree Beats
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/range_chmin_chmax_add_range_sum>

use crate::utils::bits::highest_one;
use crate::utils::range::range_bounds_to_range;
use std::cmp::{max, min, Ordering};
use std::ops::RangeBounds;

#[inline]
fn lc(i: usize) -> usize {
    i << 1
}

#[inline]
fn rc(i: usize) -> usize {
    i << 1 | 1
}

/// Segment Tree Beats
///
/// 値を区間加算・区間を最小値で更新・区間を最大値で更新、区間総和・区間最小値・区間最大値をとる操作が可能なデータ構造
#[derive(Clone, Debug)]
pub struct SegtreeBeats {
    hsize: usize,
    original_size: usize,

    fst_max: Vec<i64>,
    snd_max: Vec<i64>,
    max_count: Vec<usize>,

    fst_min: Vec<i64>,
    snd_min: Vec<i64>,
    min_count: Vec<usize>,

    sum: Vec<i64>,
    lazy_add: Vec<i64>,
}

impl SegtreeBeats {
    /// 長さ`n`の[`SegtreeBeats`]を生成する。
    pub fn new(n: usize) -> Self {
        let size = n.next_power_of_two() * 2;

        Self {
            hsize: size / 2,
            original_size: n,
            fst_max: vec![i64::MIN; size],
            snd_max: vec![i64::MIN; size],
            max_count: vec![0; size],
            fst_min: vec![i64::MAX; size],
            snd_min: vec![i64::MAX; size],
            min_count: vec![0; size],
            sum: vec![0; size],
            lazy_add: vec![0; size],
        }
    }

    fn update_node_max(&mut self, i: usize, x: i64) {
        self.sum[i] += (x - self.fst_max[i]) * (self.max_count[i] as i64);

        if self.fst_max[i] == self.fst_min[i] {
            self.fst_min[i] = x;
        } else if self.fst_max[i] == self.snd_min[i] {
            self.snd_min[i] = x;
        }

        self.fst_max[i] = x;
    }

    fn update_node_min(&mut self, i: usize, x: i64) {
        self.sum[i] += (x - self.fst_min[i]) * (self.min_count[i] as i64);

        if self.fst_max[i] == self.fst_min[i] {
            self.fst_max[i] = x;
        } else if self.snd_max[i] == self.fst_min[i] {
            self.snd_max[i] = x;
        }

        self.fst_min[i] = x;
    }

    fn update_node_add(&mut self, i: usize, x: i64) {
        let len = self.hsize >> highest_one(i as u64);

        self.sum[i] += x * len as i64;

        self.fst_max[i] += x;
        if self.snd_max[i] != i64::MIN {
            self.snd_max[i] += x;
        }

        self.fst_min[i] += x;
        if self.snd_min[i] != i64::MAX {
            self.snd_min[i] += x;
        }

        self.lazy_add[i] += x;
    }

    fn propagate(&mut self, i: usize) {
        if i >= self.hsize {
            return;
        }

        if self.lazy_add[i] != 0 {
            self.update_node_add(lc(i), self.lazy_add[i]);
            self.update_node_add(rc(i), self.lazy_add[i]);
            self.lazy_add[i] = 0;
        }

        if self.fst_max[i] < self.fst_max[lc(i)] {
            self.update_node_max(lc(i), self.fst_max[i]);
        }
        if self.fst_min[i] > self.fst_min[lc(i)] {
            self.update_node_min(lc(i), self.fst_min[i]);
        }

        if self.fst_max[i] < self.fst_max[rc(i)] {
            self.update_node_max(rc(i), self.fst_max[i]);
        }
        if self.fst_min[i] > self.fst_min[rc(i)] {
            self.update_node_min(rc(i), self.fst_min[i]);
        }
    }

    fn bottom_up(&mut self, i: usize) {
        let l = lc(i);
        let r = rc(i);

        self.sum[i] = self.sum[l] + self.sum[r];

        self.fst_max[i] = max(self.fst_max[l], self.fst_max[r]);

        match self.fst_max[l].cmp(&self.fst_max[r]) {
            Ordering::Less => {
                self.max_count[i] = self.max_count[r];
                self.snd_max[i] = max(self.fst_max[l], self.snd_max[r]);
            }
            Ordering::Greater => {
                self.max_count[i] = self.max_count[l];
                self.snd_max[i] = max(self.snd_max[l], self.fst_max[r]);
            }
            Ordering::Equal => {
                self.max_count[i] = self.max_count[l] + self.max_count[r];
                self.snd_max[i] = max(self.snd_max[l], self.snd_max[r]);
            }
        }

        self.fst_min[i] = min(self.fst_min[l], self.fst_min[r]);

        match self.fst_min[l].cmp(&self.fst_min[r]) {
            Ordering::Less => {
                self.min_count[i] = self.min_count[l];
                self.snd_min[i] = min(self.snd_min[l], self.fst_min[r]);
            }
            Ordering::Greater => {
                self.min_count[i] = self.min_count[r];
                self.snd_min[i] = min(self.fst_min[l], self.snd_min[r]);
            }
            Ordering::Equal => {
                self.min_count[i] = self.min_count[l] + self.min_count[r];
                self.snd_min[i] = min(self.snd_min[l], self.snd_min[r]);
            }
        }
    }

    fn chmin_(&mut self, i: usize, l: usize, r: usize, s: usize, t: usize, x: i64) {
        if r <= s || t <= l || self.fst_max[i] <= x {
            return;
        }
        if s <= l && r <= t && self.snd_max[i] < x {
            self.update_node_max(i, x);
            return;
        }
        self.propagate(i);
        self.chmin_(lc(i), l, (l + r) / 2, s, t, x);
        self.chmin_(rc(i), (l + r) / 2, r, s, t, x);
        self.bottom_up(i);
    }

    /// 区間`range`を値`x`との最小値をとって更新する。
    pub fn chmin(&mut self, range: impl RangeBounds<usize>, x: i64) {
        let (start, end) = range_bounds_to_range(range, 0, self.original_size);
        self.chmin_(1, 0, self.hsize, start, end, x);
    }

    fn chmax_(&mut self, i: usize, l: usize, r: usize, s: usize, t: usize, x: i64) {
        if r <= s || t <= l || self.fst_min[i] >= x {
            return;
        }
        if s <= l && r <= t && self.snd_min[i] > x {
            self.update_node_min(i, x);
            return;
        }
        self.propagate(i);
        self.chmax_(lc(i), l, (l + r) / 2, s, t, x);
        self.chmax_(rc(i), (l + r) / 2, r, s, t, x);
        self.bottom_up(i);
    }

    /// 区間`range`を値`x`との最大値をとって更新する。
    pub fn chmax(&mut self, range: impl RangeBounds<usize>, x: i64) {
        let (start, end) = range_bounds_to_range(range, 0, self.original_size);
        self.chmax_(1, 0, self.hsize, start, end, x);
    }

    fn add_(&mut self, i: usize, l: usize, r: usize, s: usize, t: usize, x: i64) {
        if r <= s || t <= l {
            return;
        }
        if s <= l && r <= t {
            self.update_node_add(i, x);
            return;
        }
        self.propagate(i);
        self.add_(lc(i), l, (l + r) / 2, s, t, x);
        self.add_(rc(i), (l + r) / 2, r, s, t, x);
        self.bottom_up(i);
    }

    /// 区間`range`に値`x`を加算する。
    pub fn add(&mut self, range: impl RangeBounds<usize>, x: i64) {
        let (start, end) = range_bounds_to_range(range, 0, self.original_size);
        self.add_(1, 0, self.hsize, start, end, x);
    }

    fn get_sum_(&mut self, i: usize, l: usize, r: usize, s: usize, t: usize) -> i64 {
        if r <= s || t <= l {
            return 0;
        }
        if s <= l && r <= t {
            return self.sum[i];
        }

        self.propagate(i);
        self.get_sum_(lc(i), l, (l + r) / 2, s, t) + self.get_sum_(rc(i), (l + r) / 2, r, s, t)
    }

    /// 区間`range`の総和を返す。
    pub fn sum(&mut self, range: impl RangeBounds<usize>) -> i64 {
        let (start, end) = range_bounds_to_range(range, 0, self.original_size);
        self.get_sum_(1, 0, self.hsize, start, end)
    }

    fn get_max_(&mut self, i: usize, l: usize, r: usize, s: usize, t: usize) -> i64 {
        if r <= s || t <= l {
            return i64::MIN;
        }
        if s <= l && r <= t {
            return self.fst_max[i];
        }
        self.propagate(i);
        max(
            self.get_max_(lc(i), l, (l + r) / 2, s, t),
            self.get_max_(rc(i), (l + r) / 2, r, s, t),
        )
    }

    /// 区間`range`の最大値を返す。
    pub fn max(&mut self, range: impl RangeBounds<usize>) -> i64 {
        let (start, end) = range_bounds_to_range(range, 0, self.original_size);
        self.get_max_(1, 0, self.hsize, start, end)
    }

    fn get_min_(&mut self, i: usize, l: usize, r: usize, s: usize, t: usize) -> i64 {
        if r <= s || t <= l {
            return i64::MAX;
        }
        if s <= l && r <= t {
            return self.fst_min[i];
        }
        self.propagate(i);
        min(
            self.get_min_(lc(i), l, (l + r) / 2, s, t),
            self.get_min_(rc(i), (l + r) / 2, r, s, t),
        )
    }

    /// 区間`range`の最小値を返す。
    pub fn min(&mut self, range: impl RangeBounds<usize>) -> i64 {
        let (start, end) = range_bounds_to_range(range, 0, self.original_size);
        self.get_min_(1, 0, self.hsize, start, end)
    }

    pub fn new_with_vec(a: Vec<i64>) -> Self {
        let mut ret = Self::new(a.len());
        let hsize = ret.hsize;

        for (i, x) in a.into_iter().enumerate() {
            ret.fst_max[hsize + i] = x;
            ret.max_count[hsize + i] = 1;
            ret.fst_min[hsize + i] = x;
            ret.min_count[hsize + i] = 1;
            ret.sum[hsize + i] = x;
        }

        for i in (1..hsize).rev() {
            ret.bottom_up(i);
        }

        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testtools::*;
    use rand::Rng;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let n = 1000;
        let limit = 1000000000;

        let mut a = vec![0; n];
        let mut seg = SegtreeBeats::new_with_vec(a.clone());

        for _ in 0..10000 {
            match rng.gen_range(0..=5) {
                0 => {
                    let lr = rand_range(&mut rng, 0..n);
                    let x = rng.gen_range(-limit..=limit);
                    seg.chmax(lr.clone(), x);
                    a[lr].iter_mut().for_each(|y| *y = std::cmp::max(x, *y));
                }
                1 => {
                    let lr = rand_range(&mut rng, 0..n);
                    let x = rng.gen_range(-limit..=limit);
                    seg.chmin(lr.clone(), x);
                    a[lr].iter_mut().for_each(|y| *y = std::cmp::min(x, *y));
                }
                2 => {
                    let lr = rand_range(&mut rng, 0..n);
                    let x = rng.gen_range(-limit..=limit);
                    seg.add(lr.clone(), x);
                    a[lr].iter_mut().for_each(|y| *y += x);
                }
                3 => {
                    let lr = rand_range(&mut rng, 0..n);
                    assert_eq!(seg.sum(lr.clone()), a[lr].iter().sum());
                }
                4 => {
                    let lr = rand_range(&mut rng, 0..n);
                    assert_eq!(
                        seg.max(lr.clone()),
                        a[lr].iter().max().copied().unwrap_or(std::i64::MIN)
                    );
                }
                5 => {
                    let lr = rand_range(&mut rng, 0..n);
                    assert_eq!(
                        seg.min(lr.clone()),
                        a[lr].iter().min().copied().unwrap_or(std::i64::MAX)
                    );
                }

                _ => unreachable!(),
            }
        }
    }
}
