//! Li-Chao tree
//!
//! # Problems
//!
//! - [Line Add Get Min](https://judge.yosupo.jp/submission/217829)
//! - [Segment Add Get Min](https://judge.yosupo.jp/submission/217834)

use crate::algo::bsearch_slice::BinarySearch;
use crate::math::linear::*;
use crate::trait_alias;
use std::{
    cmp::{max, min},
    mem::swap,
    ops::{Add, Mul, RangeInclusive},
};

trait_alias!(
    /// [`LiChaoTree<T>`]が扱える型
    Elem: Copy + Ord + Add<Output = Self> + Mul<Output = Self>
);

/// 最大値クエリか最小値クエリかを表す
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Mode {
    /// 最大値クエリ
    Max,
    /// 最小値クエリ
    Min,
}

impl Mode {
    fn op<T: Elem>(self, a: T, b: T) -> T {
        match self {
            Self::Max => max(a, b),
            Self::Min => min(a, b),
        }
    }

    fn cmp<T: Elem>(self, a: T, b: T) -> bool {
        match self {
            Self::Max => a > b,
            Self::Min => a < b,
        }
    }
}

/// 直線や線分を追加して、事前指定した点での最大値(/最小値)を得る。
pub struct LiChaoTree<T> {
    xs: Vec<T>,
    size: usize,
    data: Vec<Option<Linear<T>>>,
    range: Vec<(usize, usize)>,
    mode: Mode,
}

impl<T: Elem> LiChaoTree<T> {
    fn init_range(
        range: &mut Vec<(usize, usize)>,
        size: usize,
        i: usize,
        left: usize,
        right: usize,
    ) {
        if i < size * 2 {
            range[i] = (left, right);
            let mid = (left + right) / 2;
            Self::init_range(range, size, i << 1, left, mid);
            Self::init_range(range, size, (i << 1) | 1, mid, right);
        }
    }

    /// `query`で使用する点列`xs`と最大値クエリ/最小値クエリを指定して[`LiChaoTree`]を構築する。
    pub fn new(mut xs: Vec<T>, mode: Mode) -> Result<Self, &'static str> {
        if xs.is_empty() {
            Err("`xs`が空なので、`LiChaoTree`は構築されない。")
        } else {
            xs.sort();
            xs.dedup();

            let size = xs.len().next_power_of_two();

            xs.resize(size, *xs.last().unwrap());

            let data = vec![None; size * 2];
            let mut range = vec![(0, 0); size * 2];
            Self::init_range(&mut range, size, 1, 0, size);

            Ok(Self {
                xs,
                size,
                data,
                range,
                mode,
            })
        }
    }

    fn update(&mut self, i: usize, mut new_line: Linear<T>, l: usize, r: usize) {
        if let Some(line) = &self.data[i] {
            let m = (l + r) / 2;
            let lx = self.xs[l];
            let mx = self.xs[m];
            let rx = self.xs[r - 1];

            let left = self.mode.cmp(new_line.apply(lx), line.apply(lx));
            let mid = self.mode.cmp(new_line.apply(mx), line.apply(mx));
            let right = self.mode.cmp(new_line.apply(rx), line.apply(rx));

            if left && right {
                self.data[i] = Some(new_line);
                return;
            }

            if !left && !right {
                return;
            }

            if mid {
                swap(self.data[i].as_mut().unwrap(), &mut new_line);
            }

            if left != mid {
                self.update(i << 1, new_line, l, m);
            } else {
                self.update((i << 1) | 1, new_line, m, r);
            }
        } else {
            self.data[i] = Some(new_line);
        }
    }

    /// 直線を追加する。
    pub fn add_line(&mut self, line: Linear<T>) {
        self.update(1, line, 0, self.size);
    }

    /// 線分を追加する。
    pub fn add_segment(&mut self, segment: Linear<T>, range: RangeInclusive<T>) {
        let mut l = self.xs.lower_bound(range.start()) + self.size;
        let mut r = self.xs.lower_bound(range.end()) + self.size;

        while l < r {
            if r & 1 == 1 {
                r -= 1;
                self.update(r, segment, self.range[r].0, self.range[r].1);
            }
            if l & 1 == 1 {
                self.update(l, segment, self.range[l].0, self.range[l].1);
                l += 1;
            }

            r >>= 1;
            l >>= 1;
        }
    }

    /// `x`での最大値/最小値を返す。
    pub fn query(&self, x: T) -> Option<T> {
        let i = self.xs.binary_search(&x).expect("`x` must be in `xs`");
        let mut k = i + self.size;
        let mut ret = None;

        while k > 0 {
            if let Some(line) = &self.data[k] {
                let a = line.apply(self.xs[i]);
                ret = Some(ret.map_or(a, |y| self.mode.op(y, a)));
            }

            k >>= 1;
        }

        ret
    }
}
