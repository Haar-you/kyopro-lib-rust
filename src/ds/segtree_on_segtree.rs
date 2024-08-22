use crate::{
    algebra::traits::*,
    algo::{bsearch::lower_bound, merge::merge},
    ds::segtree::*,
};

use std::ops::Range;

#[derive(Clone, Default)]
pub struct SegtreeOnSegtreeBuilder {
    xs: Vec<i64>,
    ys: Vec<i64>,
}

pub struct SegtreeOnSegtree<M: Monoid> {
    c_xs: Vec<i64>,
    c_ys: Vec<Vec<i64>>,
    x_size: usize,
    segs: Vec<Option<Segtree<M>>>,
    monoid: M,
}

impl SegtreeOnSegtreeBuilder {
    pub fn new() -> Self {
        Self {
            xs: vec![],
            ys: vec![],
        }
    }

    pub fn add(&mut self, x: i64, y: i64) {
        self.xs.push(x);
        self.ys.push(y);
    }

    pub fn build<T: Clone, M: Monoid<Output = T> + Clone>(self, monoid: M) -> SegtreeOnSegtree<M> {
        let n = self.xs.len();
        let mut c_xs = self.xs.clone();
        c_xs.sort_unstable();
        c_xs.dedup();

        let x_size = c_xs.len().next_power_of_two() * 2;

        let mut c_ys = vec![vec![]; x_size];

        for i in 0..n {
            let j = lower_bound(&c_xs, &self.xs[i]);
            c_ys[j + x_size / 2].push(self.ys[i]);
        }

        for i in 0..x_size / 2 {
            let v = &mut c_ys[i + x_size / 2];
            v.sort();
            v.dedup();
        }

        for i in (1..x_size / 2).rev() {
            c_ys[i] = merge(c_ys[i << 1].clone(), c_ys[i << 1 | 1].clone());
            c_ys[i].dedup();
        }

        let mut segs = vec![None; x_size];
        for i in 1..x_size {
            segs[i] = Some(Segtree::new(c_ys[i].len(), monoid.clone()));
        }

        SegtreeOnSegtree {
            c_xs,
            c_ys,
            x_size,
            segs,
            monoid,
        }
    }
}

impl<T: Clone, M: Monoid<Output = T>> SegtreeOnSegtree<M> {
    pub fn update(&mut self, x: i64, y: i64, value: T) {
        let mut i = lower_bound(&self.c_xs, &x) + self.x_size / 2;
        while i >= 1 {
            let j = lower_bound(&self.c_ys[i], &y);
            self.segs[i].as_mut().unwrap().update(j, value.clone());
            i >>= 1;
        }
    }

    fn fold_sub(&self, i: usize, y1: i64, y2: i64) -> T {
        let l = lower_bound(&self.c_ys[i], &y1);
        let r = lower_bound(&self.c_ys[i], &y2);
        self.segs[i].as_ref().unwrap().fold(l..r)
    }

    pub fn fold_2d(
        &self,
        Range { start: x1, end: x2 }: Range<i64>,
        Range { start: y1, end: y2 }: Range<i64>,
    ) -> T {
        let mut l = lower_bound(&self.c_xs, &x1) + self.x_size / 2;
        let mut r = lower_bound(&self.c_xs, &x2) + self.x_size / 2;

        let mut ret = self.monoid.id();

        while l < r {
            if r & 1 == 1 {
                r -= 1;
                ret = self.monoid.op(ret, self.fold_sub(r, y1, y2));
            }
            if l & 1 == 1 {
                ret = self.monoid.op(ret, self.fold_sub(l, y1, y2));
                l += 1;
            }

            l >>= 1;
            r >>= 1;
        }

        ret
    }
}
