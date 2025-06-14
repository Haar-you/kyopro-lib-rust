//! セグメント木上にセグメント木を構築する。
use crate::{
    algebra::traits::*,
    algo::{bsearch_slice::BinarySearch, merge::merge},
    ds::segtree::*,
};

use std::ops::Range;

/// [`SegtreeOnSegtree`]を構築するための構造体。
#[derive(Clone, Default)]
pub struct SegtreeOnSegtreeBuilder {
    xs: Vec<i64>,
    ys: Vec<i64>,
}

/// セグメント木上にセグメント木を構築する。
pub struct SegtreeOnSegtree<M: Monoid> {
    c_xs: Vec<i64>,
    c_ys: Vec<Vec<i64>>,
    x_size: usize,
    segs: Vec<Option<Segtree<M>>>,
}

impl SegtreeOnSegtreeBuilder {
    /// 空の[`SegtreeOnSegtreeBuilder`]を用意する。
    pub fn new() -> Self {
        Self {
            xs: vec![],
            ys: vec![],
        }
    }

    /// 点`(x, y)`を登録する。
    pub fn add(&mut self, x: i64, y: i64) {
        self.xs.push(x);
        self.ys.push(y);
    }

    /// [`SegtreeOnSegtree`]を構築する。
    pub fn build<M: Monoid + Clone>(self) -> SegtreeOnSegtree<M> {
        let n = self.xs.len();
        let mut c_xs = self.xs.clone();
        c_xs.sort_unstable();
        c_xs.dedup();

        let x_size = c_xs.len().next_power_of_two() * 2;

        let mut c_ys = vec![vec![]; x_size];

        for i in 0..n {
            let j = c_xs.lower_bound(&self.xs[i]);
            c_ys[j + x_size / 2].push(self.ys[i]);
        }

        for i in 0..x_size / 2 {
            let v = &mut c_ys[i + x_size / 2];
            v.sort();
            v.dedup();
        }

        for i in (1..x_size / 2).rev() {
            c_ys[i] = merge(c_ys[i << 1].clone(), c_ys[(i << 1) | 1].clone());
            c_ys[i].dedup();
        }

        let mut segs = vec![None; x_size];
        for i in 1..x_size {
            segs[i] = Some(Segtree::new(c_ys[i].len()));
        }

        SegtreeOnSegtree {
            c_xs,
            c_ys,
            x_size,
            segs,
        }
    }
}

impl<M: Monoid + Clone> SegtreeOnSegtree<M> {
    /// 点`(x, y)`の値を`value`で更新する。
    pub fn update(&mut self, x: i64, y: i64, value: M) {
        let mut i = self.c_xs.lower_bound(&x) + self.x_size / 2;
        while i >= 1 {
            let j = self.c_ys[i].lower_bound(&y);
            self.segs[i].as_mut().unwrap().update(j, value.clone());
            i >>= 1;
        }
    }

    fn fold_sub(&self, i: usize, y1: i64, y2: i64) -> M {
        let l = self.c_ys[i].lower_bound(&y1);
        let r = self.c_ys[i].lower_bound(&y2);
        self.segs[i].as_ref().unwrap().fold(l..r)
    }

    /// `[x1, x2), [y1, y2)`で計算を集約する。
    pub fn fold_2d(
        &self,
        Range { start: x1, end: x2 }: Range<i64>,
        Range { start: y1, end: y2 }: Range<i64>,
    ) -> M {
        let mut l = self.c_xs.lower_bound(&x1) + self.x_size / 2;
        let mut r = self.c_xs.lower_bound(&x2) + self.x_size / 2;

        let mut ret = M::id();

        while l < r {
            if r & 1 == 1 {
                r -= 1;
                ret = M::op(ret, self.fold_sub(r, y1, y2));
            }
            if l & 1 == 1 {
                ret = M::op(ret, self.fold_sub(l, y1, y2));
                l += 1;
            }

            l >>= 1;
            r >>= 1;
        }

        ret
    }
}
